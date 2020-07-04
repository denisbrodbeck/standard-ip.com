use async_std::task::block_on;
use http_types::headers::HeaderValue;

use tide::log;
use tide::security::{CorsMiddleware, Origin};
use tide::{http::mime, Body, Response};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Data {
    ip: String,
}

fn parse(data: Option<&str>) -> Option<&str> {
    match data?.rfind(']') {
        // [::1] or [::1]:34323 - just split after last `]`
        Some(index) => data?.get(0..index + 1),
        None => {
            match data?.rfind(":") {
                // 127.0.0.1:43432
                Some(index) => data?.get(0..index),
                // 127.0.0.1
                None => data,
            }
        }
    }
}

fn escape_xml_value(val: &str) -> String {
    // this isn't very efficient but the input string is short and who wants xml anyways ¯\_(ツ)_/¯
    val.replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("'", "&apos;")
        .replace(r#"""#, "&quot;")
}

async fn reply_plain(req: tide::Request<()>) -> tide::Result<Response> {
    let mut res = Response::new(200);
    res.set_content_type(mime::PLAIN);
    res.set_body(parse(req.remote()).unwrap_or_default());
    Ok(res)
}

async fn reply_json(req: tide::Request<()>) -> tide::Result<Response> {
    let data = Data {
        ip: parse(req.remote()).unwrap_or_default().to_string(),
    };
    let mut res = Response::new(200);
    res.set_content_type(mime::JSON);
    res.set_body(Body::from_json(&data)?);
    Ok(res)
}

async fn reply_xml(req: tide::Request<()>) -> tide::Result<Response> {
    use std::str::FromStr;
    let mut res = Response::new(200);
    res.set_content_type(mime::Mime::from_str("application/xml")?);
    res.set_body(format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Address><IP>{}</IP></Address>"#,
        escape_xml_value(parse(req.remote()).unwrap_or_default())
    ));
    Ok(res)
}

enum Mime {
    PLAIN,
    HTML,
    JSON,
    XML,
}

fn parse_accept(values: Option<&http_types::headers::HeaderValues>) -> Option<Mime> {
    // Typical accept header:
    // Accept: text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8
    let vec = values?
        .get(0)?
        .as_str()
        .split(',')
        .map(|v| v.trim().split(';').collect::<Vec<&str>>()[0])
        .collect::<Vec<&str>>();
    for v in vec {
        match v {
            "text/plain" => {
                return Some(Mime::PLAIN);
            }
            "text/html" => {
                return Some(Mime::HTML);
            }
            "application/xhtml+xml" => {
                return Some(Mime::HTML);
            }
            "application/json" => {
                return Some(Mime::JSON);
            }
            "text/javascript" => {
                return Some(Mime::JSON);
            }
            "text/xml" => {
                return Some(Mime::XML);
            }
            "application/xml" => {
                return Some(Mime::XML);
            }
            _ => {
                return None;
            }
        }
    }
    None
}

async fn reply_accepted(req: tide::Request<()>) -> tide::Result<Response> {
    match parse_accept(req.header(http_types::headers::ACCEPT)) {
        Some(mime) => match mime {
            Mime::PLAIN => {
                return reply_plain(req).await;
            }
            Mime::HTML => {
                // not sure if html should ever be supported
                return reply_plain(req).await;
            }
            Mime::JSON => {
                return reply_json(req).await;
            }
            Mime::XML => {
                return reply_xml(req).await;
            }
        },
        None => {
            return reply_plain(req).await;
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    block_on(async {
        log::start();
        let cors = CorsMiddleware::new()
            .allow_methods("GET, OPTIONS".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::Any);

        let mut app = tide::new();
        app.middleware(cors);
        app.at("/").get(reply_accepted);
        app.at("/hello").get(|_| async { Ok("Hello, world!") });
        app.at("/plain").get(reply_plain);
        app.at("/json").get(reply_json);
        app.at("/xml").get(reply_xml);

        app.at("/health").get(|_| async { Ok("OK") });
        app.listen("127.0.0.1:8080").await?;
        Ok(())
    })
}

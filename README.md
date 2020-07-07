# standard-ip.com

## App

`api.standard-ip.com` supports multiple ways to find out your own external ip address:

```text
https://api.standard-ip.com/json  -- {"ip":"127.0.0.1"}
https://api.standard-ip.com/plain -- 127.0.0.1
https://api.standard-ip.com/xml   -- <?xml version="1.0" encoding="UTF-8" standalone="yes"?><Address><IP>127.0.0.1</IP></Address>
https://api.standard-ip.com/      -- Response depending on your provided `ACCEPT` header, falling back to plain
```

### TODO

- [] Control proxy `Forward` with flag, only trust `Forward` & co if proxy flag set, block header spoofing
- [] CLI arguments for default port or log level
- [] Gracefull shutdown (see [#528](https://github.com/http-rs/tide/issues/528))
- [] Metrics with Prometheus
- [] Default logger is kind of meh
- [] Handle errors in middleware AND return error codes with body (see [#570](https://github.com/http-rs/tide/pull/570))

## Deployment

### Ansible

This playbook changes default SSH port from 22 to `{{ssh_port}}` defined in `site.yml` (on the first run).

Check ansible syntax:

```bash
ansible-playbook --syntax-check -i hosts.yml site.yml
```

Initial deployment:

```bash
ansible-playbook -i hosts.yml site.yml --ask-become-pass -e 'ansible_ssh_port=22'
```

Consecutive calls:

```bash
ansible-playbook -i hosts.yml site.yml --ask-become-pass
```

### User

On the first playbook run a new user `deploy` gets created, which:

- has admin privileges (added to group `wheel`)
- has a new password (provided by *you* via an environment variable)
- copies your `public ssh key` from `~/.ssh/id_hosting.pub` to the new user's `~/.ssh/authorized_keys` file (configurable)
- disables the built-in `root` account

Create a new password for user `deploy` with [`mkpasswd`](https://docs.ansible.com/ansible/latest/reference_appendices/faq.html#how-do-i-generate-encrypted-passwords-for-the-user-module):

```shell
mkpasswd --method=scrypt
```

### Host

Deploy a fresh VM with latest CentOS (currently 8.2).
Ensure your public SSH key gets deployed (for user `root`).
Ensure your VM name is a FQN, because that name gets used as hostname.

Hetzner modifies their CentOS install in (maybe unexpected) ways:

- `selinux` is in permissive mode
- `firewalld` is missing
- `rsyslog` is missing

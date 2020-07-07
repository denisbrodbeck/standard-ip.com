# User specific aliases and functions
alias rm='rm -i'
alias cp='cp -i'
alias mv='mv -i'
alias ..="cd .."
alias ...="cd ../.."
alias ....="cd ../../.."
alias .....="cd ../../../.."

LSOPTS='-lAvF --si --color=auto'  # long mode, show all, natural sort, type squiggles, friendly sizes
alias ls="ls $LSOPTS"
alias ll="ls $LSOPTS"

export HISTFILESIZE=20000
export HISTSIZE=10000
shopt -s histappend
# Combine multiline commands into one in history
shopt -s cmdhist
# Ignore duplicates, ls without options and builtin commands
HISTCONTROL=ignoredups
# ... and ignore same sucessive entries.
HISTCONTROL=ignoreboth
export HISTIGNORE="&:ls:[bf]g:exit"
# use extra globing features. See man bash, search extglob.
shopt -s extglob
# include .files when globbing.
shopt -s dotglob
# when a glob expands to nothing, make it an empty string instead of the literal characters.
shopt -s nullglob
# fix spelling errors for cd, only in interactive shell
shopt -s cdspell

# Source global definitions
if [ -f /etc/bashrc ]; then
        . /etc/bashrc
fi

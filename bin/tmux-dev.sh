#!/bin/sh
DEV_DIR_DEFAULT=~/work/cogov/cogov-rust
DEV_DIR="${DEV_DIR:-$DEV_DIR_DEFAULT}"

cd $DEV_DIR
tmux rename-window cogov-rust
tmux split-window -h $SHELL
tmux send-keys 'nix-shell https://holochain.love' C-m
tmux split-window -v $SHELL
tmux send-keys 'tig' C-m
tmux select-pane -t 0
tmux split-window -v $SHELL
tmux send-keys 'hc-run.sh' C-m
tmux select-pane -t 0

tmux select-window -t 0

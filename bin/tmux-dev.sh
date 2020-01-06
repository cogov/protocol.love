#!/bin/sh
DEV_DIR_DEFAULT=~/work/cogov/cogov-dev
DEV_DIR="${DEV_DIR:-$DEV_DIR_DEFAULT}"

cd $DEV_DIR
tmux rename-window cogov-dev
tmux split-window -h $SHELL
tmux send-keys 'nix-shell nix-shell https://github.com/holochain/holonix/archive/v0.0.60.tar.gz' 'C-m'
tmux split-window -v $SHELL
tmux send-keys 'tig' 'C-m'
tmux select-pane -t 0
tmux split-window -v $SHELL
tmux send-keys 'nix-shell --run "hc run -i http" nix-shell https://github.com/holochain/holonix/archive/v0.0.60.tar.gz' 'C-m'
tmux select-pane -t 0

tmux select-window -t 0

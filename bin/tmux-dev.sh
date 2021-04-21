#!/bin/sh
DIR_DEFAULT=~/work/cogov/protocol.love
DIR="${DIR:-$DIR_DEFAULT}"

cd $DIR
tmux rename-window protocol.love
tmux split-window -h $SHELL
tmux send-keys 'nix-shell' C-m
tmux split-window -v $SHELL
tmux send-keys 'tig' C-m
tmux select-pane -t 0
tmux split-window -v $SHELL
tmux send-keys 'hc-run.sh' C-m
tmux select-pane -t 0

tmux select-window -t 0

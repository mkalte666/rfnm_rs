#!/usr/bin/env bash
SCRIPT_DIR=$(dirname "$0")
SYSCRATE_DIR="$SCRIPT_DIR/../rfnm_sys"
LIB_DIR=$SYSCRATE_DIR/librfnm

rm -rf "$LIB_DIR"
git clone https://github.com/rfnm/librfnm "$LIB_DIR"
rm -rf "$LIB_DIR/.git"
rm -rf "$LIB_DIR/.github"
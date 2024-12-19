#!/bin/bash
set -e

main() {
    local _dir="assets/admin"
    local _file="admin_panel.tar.gz"
    local _url="https://github.com/SeaQL/sea-orm-pro/releases/latest/download/$_file"

    rm -rf assets/admin
    mkdir -p assets/admin

    curl -sSfL "$_url" -o "$_dir/$_file"
    tar xf "$_dir/$_file" --strip-components 1 -C "$_dir"
    rm -f "$_dir/$_file"
}

main "$@" || exit 1

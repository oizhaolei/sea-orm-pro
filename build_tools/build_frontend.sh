#!/bin/bash
set -e

cd pro_admin_frontend

rm -rf dist_prod
mkdir -p dist_prod

npm install
npm run build

tar -C "dist" -czf "dist_prod/admin_panel.tar.gz" .

rm -rf ../assets/admin
mkdir -p ../assets/admin
tar xf "dist_prod/admin_panel.tar.gz" --strip-components 1 -C "../assets/admin"

if [ -d ../../sea-orm-pro ]; then
    echo "../../sea-orm-pro";
    rm -rf ../../sea-orm-pro/assets/admin
    mkdir -p ../../sea-orm-pro/assets/admin
    tar xf "dist_prod/admin_panel.tar.gz" --strip-components 1 -C "../../sea-orm-pro/assets/admin"
fi

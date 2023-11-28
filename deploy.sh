#!/bin/bash

current_datetime=$(date +"%Y-%m-%d %H:%M:%S")
TEMP_DIR=$(mktemp -d)
cp -r public/* "$TEMP_DIR"
cd "$TEMP_DIR"
git init

git add .
git commit -m "Update: $current_datetime"
git remote add origin "git@github.com:fengyarnom/fengyarnom.github.io.git"
git push -f origin "master:gh-pages"
# 清理临时目录
cd ..
rm -rf "$TEMP_DIR"


TEMP_DIR=$(mktemp -d)
cp -r sources/content/posts/* "$TEMP_DIR"
cd "$TEMP_DIR"
git init

git add .
git commit -m "Update: $current_datetime"
git remote add origin "git@github.com:fengyarnom/fengyarnom.github.io.git"
git push -f origin "master:posts"
# 清理临时目录
cd ..
rm -rf "$TEMP_DIR"
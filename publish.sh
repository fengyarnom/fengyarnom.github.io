#!/bin/bash

# 设置 GitHub 仓库地址
REPO_URL="git@github.com:fengyarnom/fengyarnom.github.io.git"

# 设置仓库本地路径
REPO_PATH="/archive/code/fengyarnom.github.io"

# 获取当前时间
CURRENT_TIME=$(date +"%Y-%m-%d %H:%M:%S")

# 进入仓库目录
cd "$REPO_PATH" || exit

# 添加所有文件到 main 分支
git checkout main
git add .
git commit -m "Update main branch - $CURRENT_TIME"
git push origin main

# 添加 ./sources/content/posts/ 目录到 post 分支
git checkout -b post
git add ./sources/content/posts/
git commit -m "Update post branch - $CURRENT_TIME"
git push origin post

# 添加 ./public 目录到 gh-pages 分支
git checkout -b gh-pages
git add ./public/
git commit -m "Update gh-pages branch - $CURRENT_TIME"
git push origin gh-pages

# 返回到 main 分支
git checkout main

echo "Upload completed."

---
title: Rustic-pages 需求与开发文档
date: 2023-08-31 12:36:47
tags:
- rustic-pages
- 需求文档
categories:
- 开发
---

## 1. 引言

要求使用 Rust 编写一个高效且易于使用的静态博客生成器，它将能正确处理 Markdown 和 SCSS 文件的文件。

它将支持以下命令行指令：

- `init`： 初始化程序，创建相应文件夹及内容

- `generate` : 渲染和生成所有的静态网站内容
- `clean` : 清除缓存
- `server` : 本地部署服务，该指令应该默认调用 `clean` 和 `generate` 
- `deploy` : 自动部署静态文件到指定仓库



所有的内容应该均以变量的形式预先准备，然后再根据规则渲染文件。

## 2. 功能需求

### 2.1 博文编辑

- 用户可以创建、编辑和删除博文。
- 博文内容支持 Markdown 格式，允许用户使用 Markdown 语法编写博文。
- 用户可以设置博文的标题、发布日期、标签和归档。
- 用户可以上传图片和附件，并在博文中引用它们。
- 用户可以保存博文为草稿，随时继续编辑。

### 2.2 博文生成

- 用户可以使用命令行工具生成静态网站。
- 博客生成器将从用户指定的目录中读取博文和资源文件，并生成静态 HTML 文件。
- 用户可以指定生成的网站目标路径。
- 生成的网站应包括主页、博文详细页、归档页、分类页面和标签页面。

### 2.3 主题和样式

- 用户可以选择不同的博客主题和样式。
- 博客生成器应支持多个默认主题，用户也可以自定义主题。
- 用户可以自定义网站的样式表（CSS）。



## 3. 功能模块

### 3.1 数据结构

#### 3.1.1 Post

- title: String

- date: String

- date_simple : String

- published: bool

- tags: Vec<String>

- categories: Vec<String>

- raw_content: String

- content: String

- slug: String

- link: String

  



#### 3.1.2 Page

-  title: String,

-  date: String,

-  template: String,

-  raw_content: String,

-  content: String,

-  limited_cows:usize,

-  total: usize,

-  current: usize,

-  prev:usize,

-  prev_link: String,

-  next:usize,

-  next_link: String

  

#### 3.1.3 Archive

- posts - Vec<Post>
- pages: Vec<Page>
- tags: HashMap<String,Vec<& Post>>
- categories: HashMap<String,Vec<& Post>>



#### 3.1.4 Config

pass

### 3.2 构建逻辑（Generate）

1. 程序首先构造一个 Archive 实例，用于保存接下来所有的变量数据。

2. Post 处理过程

   1. 从 /source/content/posts 中获取所有的 markdown 文件，并进行解析，封装为一个可操作的 Post 数据实例。
   2. 所有的Post 插入到  Archive.posts

3. Tag 处理过程

   1. 遍历 Archive.posts 将每个Post 内的 Tag 解析
   2. 所有的 Tag 插入到 Archive.tags  

4. Category 处理过程

   1. 遍历 Archive.posts 将每个Post 内的 Category 解析
   2. 所有的 Category 插入到 Archive.categories

5. 页面构造逻辑

   1. 从 /source/content/pages 中获取所有的 markdown 文件，进行解析，封装为一个可操作的 Page 数据实例
   2. 所有的 Page 插入到 Archive.pages
   3. 若是front-matter 带有 pagination: tags,则对 Archive.tags 触发分页
   4. 若是front-matter 带有 template: xxx ，则渲染模板，否则默认输出 content

   

### 3.3 路径渲染逻辑

- Index - Page
  - /public/index.html
  - /public/page/N/index.html
- Archive
  - /public/archive/posts/2023/12/10/hello/index.html
  - /public/archive/tags/技术/index.html
  - /public/archive/categories/算法/index.html
- Tags - Page
  - /public/tags/index.html
  - /public/tags/page/N/index.html
- Categories - Page
  - /public/categories/index.html
  - /public/categories/page/N/index.html
- Custom Page
  - /public/custom/index.html
  - /public/custom/page/N/index.html












## README 文档

A lightweight static blog generator developed using Rust

## Usage
You can compile the source code of this project using the cargo build command.
```shell
cargo build
```
When you run cargo build, it searches for the Cargo.toml file in the current directory and builds the project based on the configuration information specified in that file.
`Cargo.toml` is the configuration file for Rust projects and contains the project's dependencies and other build configurations.




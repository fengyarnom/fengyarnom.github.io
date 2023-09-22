use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime, ParseError};
use comrak::{ComrakOptions, markdown_to_html};
use gray_matter::Matter;
use gray_matter::engine::YAML;
#[derive(Deserialize,Clone, Debug)]
pub struct PostFrontMatter {
    pub title: String,
    pub date: String,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub published:Option<bool>
}

#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Post{
    pub title: String,
    pub date: String,
    pub date_simp: String,
    pub tags: Vec<String>,
    pub categories: Vec<String>,

    pub raw_content: String,
    pub content: String,

    pub link: String,
}
impl Post {
    pub fn new(
        title: String,
        date: String,
        date_simp: String,
        tags: Vec<String>,
        categories: Vec<String>,
        raw_content: String,
        content: String,
        link: String,
    ) -> Self {
        Post {
            title,
            date,
            date_simp,
            tags,
            categories,
            raw_content,
            content,
            link
        }
    }
    pub fn parse_date_string(date_str: &str) -> Result<NaiveDateTime, ParseError> {
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
    }
}
#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Page{
    pub published: bool,
    pub title: String,
    pub date: String,
    pub template: String,
    pub raw_content: String,
    pub content: String,
    pub limited_cows:usize,
    pub total: usize,
    pub current: usize,
    pub prev:usize,
    pub prev_link: String,
    pub next:usize,
    pub next_link: String
}

#[derive(Debug)]
pub struct Archive<'a> {
    pub posts: Vec<Post>,
    pub pages: Vec<Page>,
    pub tags: HashMap<String,Vec<&'a Post>>,
    pub categories: HashMap<String,Vec<&'a Post>>,
}

pub fn generate_site(){
    let content_path = PathBuf::from("./sources/content");

    let mut archive_global = Archive{
        posts: vec![],
        pages: vec![],
        tags: HashMap::new(),
        categories: HashMap::new()
    };

    // 处理 posts
    let posts_path = format!("{}/posts",content_path.to_string_lossy());
    for entry in fs::read_dir(&posts_path).unwrap() {
        if let Ok(entry) = entry{
            if entry.path().is_file() {
                let post = parse_markdown_file(&fs::read_to_string(entry.path()).unwrap());
                // global 吸纳
                archive_global.posts.push(post);
            }
        }
    }
    archive_global.posts.sort_by(|a, b| {
        let date_a = Post::parse_date_string(&a.date).unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());
        let date_b = Post::parse_date_string(&b.date).unwrap_or_else(|_| NaiveDateTime::from_timestamp_opt(0, 0).unwrap());

        date_b.cmp(&date_a) // 降序排列
    });

    // 处理 tags categories
    for post in &archive_global.posts{
        for tag in &post.tags{
            archive_global.tags.entry(tag.to_string()).or_insert(Vec::new()).push(&post);
        }

        for category in &post.categories{
            archive_global.categories.entry(category.to_string()).or_insert(Vec::new()).push(&post);
        }
    }

    for post in &archive_global.posts{
        println!("{}",post.title);
    }


}

pub fn parse_markdown_file(markdown_content: &str) -> Post {
    let matter = Matter::<YAML>::new();
    let result = matter.parse(markdown_content);

    let mut front_matter: PostFrontMatter = result.data.unwrap().deserialize().unwrap();
    let raw_content = result.content;

    if front_matter.tags == None{
        front_matter.tags = Some(vec!["post".to_string()]);
    }

    if front_matter.categories == None{
        front_matter.categories = Some(vec!["default".to_string()]);
    }

    let content = markdown_to_html(&raw_content, &ComrakOptions::default());
    let link: String = format!("{}",front_matter.title);
    let parsed_date = Post::parse_date_string(&front_matter.date).unwrap();
    let date_simp = parsed_date.date().to_string();

    Post::new(
        front_matter.title,
        front_matter.date,
        date_simp,
        front_matter.tags.unwrap(),
        front_matter.categories.unwrap(),
        raw_content,content,link)
}
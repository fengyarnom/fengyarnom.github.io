use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Deserialize,Clone, Debug)]
pub struct FrontMatter {
    pub title: String,
    pub date: String,
    pub tags: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
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

    pub slug: String,
    pub link: String,
}

#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Page{
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

fn main() {
    println!("Hello, world!");
}

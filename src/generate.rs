use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use chrono::{Datelike, NaiveDateTime, ParseError};
use comrak::{ComrakOptions, markdown_to_html};
use gray_matter::Matter;
use gray_matter::engine::YAML;
use tera::{Context, Tera};

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

    pub source_link: String,
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
        source_link: String,
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
            source_link,
            link
        }
    }
    pub fn parse_date_string(date_str: &str) -> Result<NaiveDateTime, ParseError> {
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")
    }
}

#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Tag{
    name: String,
    posts: Vec<Post>,
    source_link: String,
    link: String,
}

#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Category{
    name: String,
    posts: Vec<Post>,
    source_link: String,
    link: String,
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
    pub next_link: String,
    pub link: String,

    pub posts: Vec<Post>,
}

#[derive( Deserialize,Serialize,Clone, Debug)]
pub struct Archive {
    pub posts: Vec<Post>,
    pub pages: Vec<Page>,
    pub tags: HashMap<String,Tag>,
    pub categories: HashMap<String,Category>,
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
        for tag in &post.tags {
            let tags_entry = archive_global.tags.entry(tag.to_string());
            match tags_entry {
                Entry::Occupied(mut occupied) => {
                    let tag = occupied.get_mut();
                    tag.posts.push(post.clone());
                }
                Entry::Vacant(mut vacant) => {
                    let new_tag = Tag {
                        name: tag.to_string(),
                        posts: vec![post.clone()],
                        source_link: format!("./public/archive/tags/{}/index.html", tag.to_string()),
                        link: format!("/archive/tags/{}/", tag.to_string()),
                    };
                    vacant.insert(new_tag);
                }
            }
        }
    }

    for post in &archive_global.posts{
        for category in &post.categories {
            let categories_entry = archive_global.categories.entry(category.to_string());
            match categories_entry {
                Entry::Occupied(mut occupied) => {
                    let category = occupied.get_mut();
                    category.posts.push(post.clone());
                }
                Entry::Vacant(mut vacant) => {
                    let new_category = Category {
                        name: category.to_string(),
                        posts: vec![post.clone()],
                        source_link: format!("./public/archive/categories/{}/index.html", category.to_string()),
                        link: format!("/archive/categories/{}/", category.to_string()),
                    };
                    vacant.insert(new_category);
                }
            }
        }
    }

    let mut tera = Tera::new("./sources/templates/**/*.html").unwrap();

    // archive post page
    for post in &archive_global.posts{
        let mut context = Context::new();

        let mut page = Page{
            published: true,
            title: post.title.to_string(),
            date: post.date_simp.to_string(),
            template: "".to_string(),
            raw_content: post.raw_content.to_string(),
            content: post.content.to_string(),
            limited_cows: 0,
            total: 0,
            current: 0,
            prev: 0,
            prev_link: "".to_string(),
            next: 0,
            next_link: "".to_string(),
            link: post.link.to_string(),

            posts: vec![post.clone()],
        };
        context.insert("page",&page);
        let rendered = tera.render("post.html", &context).unwrap();
        let folder = PathBuf::from(&post.source_link).parent().unwrap().to_string_lossy().to_string();
        fs::create_dir_all(folder);
        fs::write(&post.source_link, rendered).unwrap();

    }
    // archive tag page
    for tag in archive_global.tags{
        let mut context = Context::new();
        let mut page = Page{
            published: true,
            title: tag.0.to_string(),
            date: "".to_string(),
            template: "".to_string(),
            raw_content: "".to_string(),
            content: "".to_string(),
            limited_cows: 0,
            total: 0,
            current: 0,
            prev: 0,
            prev_link: "".to_string(),
            next: 0,
            next_link: "".to_string(),
            link: tag.1.link,
            posts: tag.1.posts.clone(),
        };
        context.insert("page",&page);
        let rendered = tera.render("archive.html", &context).unwrap();
        let folder = PathBuf::from(&tag.1.source_link).parent().unwrap().to_string_lossy().to_string();
        fs::create_dir_all(folder);
        fs::write(&tag.1.source_link, rendered).unwrap();
    }

    // archive category page
    for category in archive_global.categories{
        let mut context = Context::new();
        let mut page = Page{
            published: true,
            title: category.0.to_string(),
            date: "".to_string(),
            template: "".to_string(),
            raw_content: "".to_string(),
            content: "".to_string(),
            limited_cows: 0,
            total: 0,
            current: 0,
            prev: 0,
            prev_link: "".to_string(),
            next: 0,
            next_link: "".to_string(),
            link: category.1.link,
            posts: category.1.posts.clone(),
        };
        context.insert("page",&page);

        let rendered = tera.render("archive.html", &context).unwrap();
        let folder = PathBuf::from(&category.1.source_link).parent().unwrap().to_string_lossy().to_string();
        fs::create_dir_all(folder);
        fs::write(&category.1.source_link, rendered).unwrap();
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

    let date = Post::parse_date_string(&front_matter.date).unwrap();
    let source_link = format!("./public/archive/posts/{}/{}/{}/{}.html",date.year(),date.month(),date.day(),front_matter.title);
    let link = format!("/archive/posts/{}/{}/{}/{}.html",date.year(),date.month(),date.day(),front_matter.title);
    let parsed_date = Post::parse_date_string(&front_matter.date).unwrap();
    let date_simp = parsed_date.date().to_string();

    Post::new(
        front_matter.title,
        front_matter.date,
        date_simp,
        front_matter.tags.unwrap(),
        front_matter.categories.unwrap(),
        raw_content,content,source_link,link)
}
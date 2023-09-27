mod generate;

use std::{env, fs, process};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use actix_web::{App, HttpServer, Responder};
use actix_files::Files;
use chrono::Utc;
use notify::{ Watcher, RecursiveMode};

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // 禁止直接通过 cargo run 命令运行程序
    // 打印帮助文档
    if args.len() < 2 {
        println!("Usage: cargo run <command>");
        println!("init\t -- Initialize a new blog");
        println!("new\t -- create a new post");
        println!("generate\t-- generate all static files");
        println!("server\t-- Start a local server to preview the blog");
        return;
    }

    let command = args[1].as_str();
    match command{
        "init" =>{
            init();
        },
        "new" => {
            new(args);
        }
        "generate" => {
            generate()
        },
        "server" => {
            if let Err(e) = server().await {
                eprintln!("Error starting server: {}", e);
                process::exit(1);
            }
        }

        &_ => {}
    }
}

async fn server() -> std::io::Result<()> {
    println!("Starting server...");
    generate();
    println!("Running server!");

    let mut watcher = notify::recommended_watcher(|res| {
        match res {
            Ok(event) => {
                generate();
                println!("Restarted server!");
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }).unwrap();
    watcher.watch(Path::new("./sources"), RecursiveMode::Recursive).unwrap();


    // 服务启动，默认在本地 127.0.0.1:8080 运行
    HttpServer::new(|| {
        App::new().service(Files::new("/", "public").index_file("index.html"))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn init() {
    println!("Init...");
    fs::create_dir_all("./public").expect("Failed to create the 'public' directory.");
}

fn generate() {
    println!("Generating...");
    generate::generate_site();
}

fn new(args: Vec<String>){
    if args.len() < 3 {
        println!("Usage: new 'post title'");
        return;
    }

    let local_time = Utc::now().naive_utc();
    let post_title = args[2].as_str();
    let file_name = format!("./sources/content/posts/{}-{}.md", local_time.format("%Y-%m-%d"),post_title.to_lowercase().replace(" ", "-"));

    let file_content = format!(
        "---\ntitle: {}\ndate: {}\ntags:\ncategories:\n---",
        post_title,local_time.format("%Y-%m-%d %H:%M:%S"));

        let mut file = File::create(&file_name).unwrap();
        file.write_all(file_content.as_bytes()).unwrap();
        println!("New post created: {}", &file_name);
}

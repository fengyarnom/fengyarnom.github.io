mod generate;

use std::{env, fs, process};
use actix_web::{App, HttpServer, Responder};
use actix_files::Files;

#[actix_web::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    // 禁止直接通过 cargo run 命令运行程序
    // 打印帮助文档
    if args.len() < 2 {
        println!("Usage: cargo run <command>");
        println!("init\t -- Initialize a new blog");
        println!("generate\t-- generate all static files");
        println!("server\t-- Start a local server to preview the blog");
        println!("clean\t-- Clean up generated files");
        return;
    }

    let command = args[1].as_str();
    match command{
        "init" =>{
            init();
        },
        "generate" => {
            generate();
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
    println!("Running server!");

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
}

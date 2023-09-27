# rustic-pages
A lightweight static blog generator developed using Rust

## Usage

You can compile the source code of this project using the cargo build command.

```shell
cargo build
```

When you run cargo build, it searches for the Cargo.toml file in the current directory and builds the project based on the configuration information specified in that file.
`Cargo.toml` is the configuration file for Rust projects and contains the project's dependencies and other build configurations.

## Quick Start

### Create a new post
```shell
cargo run new "New Post"
```

#### Run server
````shell
cargo run server
````

#### Generate static files
```shell
cargo run generate
```


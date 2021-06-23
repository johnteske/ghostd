use std::path::Path;
use std::process::Command;
use std::{env, fs, str};

use urlencoding::encode;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("html.rs");

    let src_dir = Path::new("src").join("assets");

    let html_content = get_file_content(&src_dir.join("index.html"));

    Command::new("npx")
        .arg("tsc")
        .arg("--strict")
        .arg("src/assets/script.ts")
        .spawn()
        .expect("typescript compiler failed");
    let script_content = get_file_content(&src_dir.join("script.js"));

    let style_content = get_file_content(&src_dir.join("style.css"));

    let icon_content = get_file_content(&src_dir.join("favicon.svg"));
    let icon = format!("data:image/svg+xml,{}", encode(&icon_content));

    let template = r###"const HTML: &str = r##"@@HTML@@"##;"###
        .replace("@@HTML@@", &html_content)
        .replace("@@SCRIPT@@", &script_content)
        .replace("@@STYLE@@", &style_content)
        .replace("@@ICON@@", &icon);
    fs::write(&dest_path, template).unwrap();

    println!("cargo:rerun-if-changed=src/assets/index.html");
    println!("cargo:rerun-if-changed=src/assets/script.ts");
    println!("cargo:rerun-if-changed=src/assets/style.css");
}

fn get_file_content(path: &std::path::Path) -> String {
    let file = fs::read(&path).expect("error reading file");
    let content = str::from_utf8(&file).expect("error parsing file");
    String::from(content)
}

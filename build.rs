use std::env;
use std::fs;
use std::path::Path;
use std::str;

// TODO generate from file
// TODO template this as well
const ICON: &str = "data:image/x-icon;base64,AAABAAEAEBAQAAEABAAoAQAAFgAAACgAAAAQAAAAIAAAAAEABAAAAAAAgAAAAAAAAAAAAAAAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAD//wAAznMAANWrAADb2wAA33sAAN47AADQCwAA3/sAANWrAADb2wAA1asAAN/7AADv9wAA9+8AAPgfAAD//wAA";

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("html.rs");

    let src_dir = Path::new("src");

    let html_content = get_file_content(&src_dir.join("index.html"));
    let script_content = get_file_content(&src_dir.join("script.js"));
    let style_content = get_file_content(&src_dir.join("style.css"));

    let template = r###"const HTML: &str = r##"@@HTML@@"##;"###
        .replace("@@HTML@@", &html_content)
        .replace("@@SCRIPT@@", &script_content)
        .replace("@@STYLE@@", &style_content)
        .replace("@@ICON@@", ICON);
    fs::write(&dest_path, template).unwrap();

    println!("cargo:rerun-if-changed=src/index.html");
    println!("cargo:rerun-if-changed=src/script.js");
    println!("cargo:rerun-if-changed=src/style.css");
}

fn get_file_content(path: &std::path::PathBuf) -> String {
    let path_str = match path.to_str() {
        Some(s) => s,
        None => "path",
    };
    let file = fs::read(&path).expect(&format!("error reading {}", path_str));
    String::from(str::from_utf8(&file).expect(&format!("error parsing {}", path_str)))
}

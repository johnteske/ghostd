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

    let html = fs::read(&src_dir.join("index.html")).expect("could not read index.html");
    let html_content = str::from_utf8(&html).unwrap();

    let script = fs::read(&src_dir.join("script.js")).expect("could not read script.js");
    let script_content = str::from_utf8(&script).unwrap();

    let template = r##"const HTML: &str = r#"{{HTML}}"#;"##
        .replace("{{HTML}}", html_content)
        .replace("{{SCRIPT}}", script_content)
        .replace("{{ICON}}", ICON);
    fs::write(&dest_path, template).unwrap();

    println!("cargo:rerun-if-changed=src/index.html");
    println!("cargo:rerun-if-changed=src/script.js");
}

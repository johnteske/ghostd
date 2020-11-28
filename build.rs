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

    let index_html = Path::new("src").join("index.html");
    let html = fs::read(&index_html).expect("could not read index.html");
    let content = str::from_utf8(&html).unwrap();
    let template = r##"const HTML: &str = r#"{{HTML}}"#;"##
        .replace("{{HTML}}", content)
        .replace("{{ICON}}", ICON);
    fs::write(&dest_path, template).unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}

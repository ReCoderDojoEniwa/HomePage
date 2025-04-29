mod parser;

use std::{env, fs::{read_to_string, File}, io::Write, path::PathBuf};

fn save(path:&PathBuf, inside:&str) {
    match File::create(path) {
        Ok(mut file) => {
            if let Err(e) = writeln!(file, "{}", inside) {
                println!(
                    "Couldn't write onto a file, {} because of the error:\n{}",
                    path.display(),
                    e
                )
            }
        },
        Err(e) => println!(
            "Couldn't make a file, {} because of the error:\n{}",
            path.display(),
            e
        )
    }
}
fn main() {
    let cur_path = env::current_dir().unwrap();
    let get_fullpath_with_check = |f_name:&str| {
        let the_path = cur_path.join(f_name);
        if the_path.exists() {
            the_path
        } else {
            panic!("The path, {} doesn't exist.", the_path.display())
        }
    };
    let blogs_raw = get_fullpath_with_check("blogs_raw");
    let blogs_parsed = get_fullpath_with_check("blogs_parsed");
    let template = get_fullpath_with_check("blog_template.html");
    let index_template = get_fullpath_with_check("blog_index_template.html");

    let mut indexs = Vec::new();
    let blog_folders = blogs_raw
        .read_dir()
        .unwrap()
        .map(|f| f.unwrap())
        .filter(|f| f.path().is_dir());

    for blog_folder in blog_folders {
        let mut contents = blog_folder
            .path()
            .read_dir()
            .unwrap()
            .map(|f| f.unwrap());
        let braw = match contents.find(|f| {
            let path = f.path();
            path.is_file() && (match path.extension() {
                Some(s) => s.to_string_lossy() == "braw",
                None => false
            })
        }) {
            Some(s) => s.path(),
            None => {
                println!("The folder {} has no .braw.", blog_folder.path().display());
                continue;
            }
        };
        let blog = parser::braw_to_blog(&braw).unwrap();
        let parsed_path = blogs_parsed.join(format!("{}.html", blog.title));
        save(&parsed_path, blog.to_html(&blogs_parsed, &blog_folder.path(), &template).unwrap().as_str());
        indexs.push(blog.get_index(&cur_path, &parsed_path));
    }
    save(
        &cur_path.join("blog_index.html"),
        read_to_string(index_template)
            .unwrap()
            .replace("<!-- Insert Content Here -->", &indexs.join("\n"))
            .as_str()
    );
}
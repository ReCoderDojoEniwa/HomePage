use std::fs;
use std::path::PathBuf;
use pathdiff;

pub struct Blog {
    pub title: String,
    pub thumbnail: Option<PathBuf>,
    pub lines: Vec<String>
}

pub fn braw_to_blog(path:&PathBuf) -> Result<Blog, ()> {
    if !(path.is_file() & path.extension().map_or(false, |ext| ext == "braw")) {
        return Err(());
    }
    let inside = fs::read_to_string(path).unwrap();
    let lines = inside
        .lines()
        .filter(|f| !f.is_empty())
        .map(|f| f.to_string())
        .collect::<Vec<String>>();
    let thumbnail = lines.iter().find(|f| f.starts_with("!img"));
    let thumbnail = match thumbnail {
        Some(s) => Some(path.parent().unwrap().join(s.split_once(" ").unwrap().1)),
        None => None
    };
    Ok(Blog {
        title: path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string(),
        thumbnail: thumbnail,
        lines
    })
}

impl Blog {
    pub fn to_html(&self, base_path:&PathBuf, blog_folder:&PathBuf, template:&PathBuf) -> Result<String, std::io::Error> {
        let result = self
            .lines
            .iter()
            .map(|l| parse_a_line(l, base_path, blog_folder).unwrap())
            .collect::<Vec<String>>()
            .join("\n");
        Ok(fs::read_to_string(template)?
            .replace("<!-- Insert Content Here -->", &result))
    }
    fn generate_overview(&self, length:usize) -> String {
        let raw = self
            .lines
            .iter()
            .filter(|t| !t.starts_with("!"))
            .map(|t| t.replace("#", "").trim().to_string())
            .collect::<Vec<String>>()
            .join("\n");
        let mut overview = raw
            .chars()
            .take(std::cmp::min(raw.len(), length))
            .collect::<String>()
            .to_string();
        if length < raw.len() {
            overview.push_str("‥‥");
        }
        overview
    }
    pub fn get_index(&self, base_path:&PathBuf, blog_path:&PathBuf) -> String {
        let thumbnail = match &self.thumbnail {
            Some(s) => format!(
                "<img class=\"blog_thumbnail\" src=\"{}\">\n",
                pathdiff::diff_paths(s, base_path).unwrap().to_string_lossy()
            ),
            None => String::new()
        };
        format!(
            "<div class=\"blog_container\">\n{}<div>\n<a class=\"blog_title\" href=\"{}\">{}</a>\n<p>{}</p>\n</div>\n</div>",
            thumbnail,
            pathdiff::
                diff_paths(blog_path.to_str().unwrap(), base_path)
                .unwrap()
                .to_string_lossy(),
            self.title.clone(),
            self.generate_overview(100)
        )
    }
}

fn parse_a_line(raw:&str, base_path:&PathBuf, blog_folder:&PathBuf) -> Result<String, ()> {
    let raw = raw.trim();
    if raw.starts_with("!") {
        match raw[1..].split_once(" ") {
            Some(s) => {
                match s.0 {
                    "img" => Ok(
                        format!("<img class=\"image\" src=\"{}\">",
                        pathdiff::diff_paths(blog_folder.join(s.1), base_path).unwrap().to_str().unwrap())
                    ),
                    _ => Err(())
                }
            },
            None => return Err(())
        }
    } else if raw.starts_with("#") {
        if let Some(s) = raw.chars().nth(1) {
            if s == '#' {
                Ok(format!("<h3 class=\"subheading\">{}</h3>", raw.split_at(2_usize).1.trim()))
            } else {
                Ok(format!("<h2 class=\"heading\">{}</h2>", raw.split_at(1_usize).1.trim()))
            }
        } else {
            Ok(format!("<h2 class=\"heading\">{}</h2>", raw.split_at(1_usize).1.trim()))
        }
    } else {
        Ok(format!("<p>{}</p>", raw))
    }
}
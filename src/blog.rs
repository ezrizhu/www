use std::fs;
use super::utils;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use super::post::Post;

pub fn init() -> Vec<Post> {
    let mut blog_list = vec![];

    if let Ok(entries) = fs::read_dir("content/blog/") {
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_file() {
                let slug = path.file_stem().unwrap().to_str().unwrap().to_string();
                let slug = slug.replace("_", "-");
                let slug = slug.replace(".md", "");
                let raw = fs::read_to_string(path).unwrap();
                let matter = Matter::<YAML>::new();
                let result = matter.parse(&raw);
                let title = result.data.as_ref().unwrap()["Title"].as_string().unwrap();
                let description = result.data.as_ref().unwrap()["Description"].as_string().unwrap();
                let date = result.data.as_ref().unwrap()["Date"].as_string().unwrap();
                let body = utils::md_to_html(&result.content);
                let blog = Post { slug, title, date, description, body };
                blog_list.push(blog);
            }
        }
    }
    blog_list
}

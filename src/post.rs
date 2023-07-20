use std::fs;
use super::utils;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use chrono::prelude::*;

#[derive(Clone)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: DateTime<FixedOffset>,
    pub description: String,
    pub tags: Vec<String>,
    pub body: String,
}


pub fn get(posts: Vec<Post>, slug: &str) -> Option<Post> {
    for post in posts {
        if post.slug == slug {
            return Some(post);
        }
    }
    None
}

pub fn init(dir: &str) -> Vec<Post> {
    let mut posts_list = vec![];

    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            let path = entry.unwrap().path();
            if path.is_file() {
                // E.g., 20230601-hello_world.md -> date_str: 20230601, slug: hello-world
                let filename = path.file_stem().unwrap().to_str().unwrap().to_string();
                let filename_parts = filename.split("-").collect::<Vec<&str>>();
                assert_eq!(filename_parts.len(), 2);

                // Date processing
                let date_str = filename_parts[0].parse::<String>().unwrap();
                // The date_str will be displayed on the homepage, blogindex, and blog pages.
                // First we parse our text into NaiveDate
                let date = NaiveDate::parse_from_str(&date_str, "%Y%m%d").unwrap();
                // Now we add time to it
                let date = NaiveDateTime::new(date, NaiveTime::from_hms_opt(0, 0, 0).unwrap());
                // Now we make this a Fixed DateTime with Eastern time
                let timezone_east = FixedOffset::east_opt(8 * 60 * 60).unwrap();
                let date = DateTime::<FixedOffset>::from_local(date, timezone_east);
                // Slug is used for the uri
                let slug = filename_parts[1];
                let slug = slug.replace("_", "-");
                let slug = slug.replace(".md", "");
                // Here we read the raw file to be processed
                let raw = fs::read_to_string(path).unwrap();

                // yaml frontmatter parsing
                let matter = Matter::<YAML>::new();
                let result = matter.parse(&raw);

                let Some(result_map) = result.data.as_ref()
                else { panic!("Error parsing YAML") };
                let Ok(result_map) = result_map.as_hashmap()
                else { panic!("Error getting hashmap from Pod") };

                let title = result_map["Title"].as_string().unwrap();
                let description = result_map["Description"].as_string().unwrap();

                // see if tags["Tags"] is exists
                let mut tags: Vec<String> = Vec::new();
                if result_map.contains_key("Tags") {
                    let taglist = result_map["Tags"].as_vec().unwrap();
                    for tag in taglist {
                        tags.push(tag.as_string().unwrap());
                    }
                }

                // the markdown without the frontmatter, parsed to html
                let body = utils::md_to_html(&result.content);

                let post = Post { slug, title, date, description, tags, body };
                posts_list.push(post);
            }
        }
    }
    posts_list.sort_by(|a, b| b.date.cmp(&a.date));
    posts_list
}

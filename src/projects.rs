use std::fs;
use super::utils;
use gray_matter::Matter;
use gray_matter::engine::YAML;

#[derive(Clone)]
pub struct Project {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
}

pub fn init() -> Vec<Project> {
    let mut projects = vec![];

    if let Ok(entries) = fs::read_dir("content/projects/") {
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
                let body = utils::md_to_html(&result.content);
                let project = Project { slug, title, description, body };
                projects.push(project);
            }
        }
    }
    projects
}

pub fn get_title_and_desc(projects: Vec<Project>, slug: &str) -> Option<(String, String)> {
    for project in projects {
        if project.slug == slug {
            return Some((project.title, project.description));
        }
    }
    None
}

pub fn get_all(projects: Vec<Project>, slug: &str) -> Option<Project> {
    for project in projects {
        if project.slug == slug {
            return Some(project);
        }
    }
    None
}

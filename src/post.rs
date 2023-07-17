#[derive(Clone)]
pub struct Post {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub body: String,
}


pub fn get_title_and_desc(posts: Vec<Post>, slug: &str) -> Option<(String, String)> {
    for post in posts {
        if post.slug == slug {
            return Some((post.title, post.description));
        }
    }
    None
}

pub fn get_date_and_title(posts: Vec<Post>, slug: &str) -> Option<(String, String)> {
    for post in posts {
        if post.slug == slug {
            return Some((post.date, post.title));
        }
    }
    None
}

pub fn get_all(posts: Vec<Post>, slug: &str) -> Option<Post> {
    for post in posts {
        if post.slug == slug {
            return Some(post);
        }
    }
    None
}


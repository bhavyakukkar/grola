use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Blog {
    id: usize,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlogPosts {
    posts: Vec<Blog>,
}

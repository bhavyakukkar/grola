use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Blog {
    id: usize,
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
pub struct BlogPosts {
    posts: Vec<Blog>,
    counter: u8,
}

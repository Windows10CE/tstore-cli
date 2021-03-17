use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CommunityListing {
    pub has_nsfw_content: bool,
    pub categories: Vec<String>,
    pub community: String,
}
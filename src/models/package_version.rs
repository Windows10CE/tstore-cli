use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PackageVersion {
    pub namespace: String,
    pub name: String,
    pub version_number: String,
    pub full_name: String,
    pub description: String,
    pub icon: String,
    pub dependencies: Vec<String>,
    pub download_url: String,
    pub downloads: usize,
    pub date_created: String,
    pub website_url: String,
    pub is_active: bool,
}

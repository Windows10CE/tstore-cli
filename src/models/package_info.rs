use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PackageInfo {
    pub namespace: String,
    pub name: String,
    pub full_name: String,
    pub owner: String,
    pub package_url: String,
    pub date_created: String,
    pub date_updated: String,
    pub rating_score: usize,
    pub is_pinned: bool,
    pub is_deprecated: bool,
    pub total_downloads: usize,
    pub latest: super::package_version::PackageVersion,
    pub community_listings: Vec<super::community_listing::CommunityListing>,
}
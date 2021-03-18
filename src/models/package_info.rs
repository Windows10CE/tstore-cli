use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};

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

impl PackageInfo {
    pub fn from_author_and_name(
        author: &str,
        package_name: &str,
        url: &str,
    ) -> Result<PackageInfo, Box<dyn std::error::Error>> {
        let res = Client::new()
            .get(format!(
                "{}/experimental/package/{}/{}/",
                url, author, package_name
            ))
            .send()?;

        if res.status() != StatusCode::OK {
            return Err(From::from("Package not found."));
        }

        let info = serde_json::from_str::<PackageInfo>(res.text()?.as_str())?;

        Ok(info)
    }
}

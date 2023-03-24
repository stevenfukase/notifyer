#[non_exhaustive]
pub struct Repository {
    pub name_with_owner: String,
    pub url: String,
    pub open_graph_image_url: String,
    // "primaryLanguage": {
    //   "name": "Rust",
    //   "color": "#dea584"
    // }
}

impl Repository {
    pub fn new(name_with_owner: &str, url: &str, open_graph_image_url: &str) -> Self {
        Self {
            name_with_owner: name_with_owner.to_string(),
            url: url.to_string(),
            open_graph_image_url: open_graph_image_url.to_string(),
        }
    }
}

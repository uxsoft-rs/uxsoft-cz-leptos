use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArticleMetadata {
    //pub image_path: String,
    pub title: String,
    pub draft: Option<bool>,
    pub date: Option<String>,
    //pub description: String,
    //pub project_link: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Article {
    pub metadata: ArticleMetadata,
    pub content: String,
    pub id: String,
}

impl Article {
    pub fn new(id: String, metadata: ArticleMetadata, content: String) -> Self {
        Self {
            id,
            metadata,
            content,
        }
    }
}

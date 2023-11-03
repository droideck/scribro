use include_dir::*;
use serde::Deserialize;
use gray_matter::Matter;
use gray_matter::engine::YAML;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct BlogPost {
    pub metadata: Metadata,
    pub text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Metadata {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub publish_date: String,
    #[serde(default)]
    pub edit_date: String,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub preview: String,
}

pub fn fetch_all_posts() -> HashMap<String, BlogPost> {
    static POSTS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/posts");
    POSTS_DIR
        .files()
        .map(|post_file| -> (String, BlogPost) {
            let matter = Matter::<YAML>::new();
            let markdown = matter.parse(post_file.contents_utf8().unwrap());
            let metadata: Metadata = markdown.data.unwrap().deserialize().unwrap();

            (
                post_file.path()
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                BlogPost {
                    metadata,
                    text: markdown.content,
                },
            )
        })
        .collect()
}

pub fn fetch_post(id: String) -> BlogPost {
    fetch_all_posts().get(&id).cloned().unwrap()
}

pub fn fetch_all_tags() -> Vec<String> {
    let mut tags: Vec<String> = Vec::new();

    fetch_all_posts()
        .values()
        .for_each(|post| {
            post.metadata.tags.iter().for_each(|tag| {
                if !tags.contains(tag) {
                    tags.push(tag.clone());
                }
            })
    });
    tags.sort();
    tags
}

pub fn fetch_tag_posts(tag: String) -> HashMap<String, BlogPost> {
    fetch_all_posts()
        .into_iter()
        .filter(|(_id, post)| post.metadata.tags.contains(&tag))
        .collect()
}

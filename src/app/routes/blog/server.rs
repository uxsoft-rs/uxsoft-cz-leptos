use super::article::Article;
use leptos::*;

#[server(ListBlogPosts, "/api")]
pub async fn fetch_articles() -> Result<Vec<Article>, ServerFnError> {
    let posts = get_articles("src/app/routes/blog/articles");

    Ok(posts)
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::{
            fs::{self, DirEntry},
            path::{Path, PathBuf},
        };
        use super::article::ArticleMetadata;

        pub fn list_article_files<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
            fs::read_dir(path)
                .unwrap()
                .filter_map(Result::ok)
                .filter(|entry| {
                    if let Ok(file_type) = entry.file_type() {
                        file_type.is_file() && entry.path().extension() == Some("md".as_ref())
                    } else {
                        false
                    }
                })
                .collect()
        }

        pub fn load_article(file: PathBuf) -> Option<Article> {
            let content = fs::read_to_string(&file).ok()?;
            let id = file.file_stem()?.to_os_string().into_string().ok()?;

            use gray_matter::engine::YAML;
            use gray_matter::Matter;
            use pulldown_cmark::{html, Options, Parser};

            let mut options = Options::empty();
            options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
            let matter = Matter::<YAML>::new();

            let post_data = matter
                .parse_with_struct::<ArticleMetadata>(&content)?;
            let post_metadata = post_data.data;

            let content = post_data.content;

            let parser = Parser::new_ext(&content, options);

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            Some(Article::new(id, post_metadata, html_output))
        }

        pub fn get_articles<P: AsRef<Path>>(path: P) -> Vec<Article> {
            let files = list_article_files(path);

            let articles = files.into_iter()
                .filter_map(|f| load_article(f.path()))
                .filter(|p| !p.metadata.draft.is_some_and(|draft| draft))
                .collect();

            articles
        }

    }
}

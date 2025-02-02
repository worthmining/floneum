#![allow(unused)]

use floneum_rust::{plugins::main::imports::get_request, *};
use nipper::Document;

#[export_plugin]
/// Searches wikipedia, fetches the top article from wikipedia, and returns it as text
fn search_engine(query: String) -> String {
    let url = format!(
        "https://en.wikipedia.org/w/index.php?search={}",
        query.replace(" ", "+")
    );
    let html = get_request(&url, &vec![]);

    let document = Document::from(&html);
    let mut results = String::new();
    let mut article_count = 0;

    document.select("a").iter().for_each(|link| {
        if let Some(href) = link.attr("href") {
            if href.starts_with("https://en.wikipedia.org/wiki/") || href.starts_with("/wiki/") {
                if article_count > 5 {
                    return;
                }
                let href = if href.starts_with("/") {
                    format!("https://en.wikipedia.org{}", href)
                } else {
                    href.to_string()
                };
                let request = get_request(&href, &vec![]);

                document.select("p").iter().for_each(|paragragh| {
                    let html = paragragh.text();
                    results += &html;
                    results += "\n";
                });
                article_count += 1;
            }
        }
    });

    results
}

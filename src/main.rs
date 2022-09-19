use regex::Regex;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct NamuWiki {
    title: String,
    text: String,
}

fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    match s.find(p) {
        Some(index) => &s[..index],
        None => s,
    }
}

fn main() {
    let out: Vec<NamuWiki>;
    {
        let data = fs::read_to_string(r"F:\namuwiki210301\namuwiki_20210301.json")
            .expect("Unable to read file");
        out = serde_json::from_str(&data).unwrap();
    }
    let href = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    for data in out.iter() {
        for caps in href.captures_iter(&data.text) {
            if caps[1].contains("파일:")
                || caps[1].contains("분류:")
                || caps[1].contains("틀:")
                || caps[1].starts_with("http")
            {
                continue;
            }
            println!("{}", remove_suffix(&caps[1], "|"));
        }
    }
}

use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
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
        let data =
            fs::read_to_string(r"/home/asm6788/NamuPPuli/test.json").expect("Unable to read file");
        out = serde_json::from_str(&data).unwrap();
    }
    let href = Regex::new(r"\[\[(.*?)\]\]").unwrap();

    let mut counter: HashMap<String, u16> = HashMap::new();
    for data in out.iter() {
        for caps in href.captures_iter(&data.text) {
            if caps[1].starts_with("파일:")
                || caps[1].starts_with("분류:")
                || caps[1].starts_with("틀:")
                || caps[1].starts_with("http")
            {
                continue;
            }
            let href = remove_suffix(remove_suffix(&caps[1], "|"), "#");
            *counter.entry(String::from(href)).or_insert(0) += 1;
        }
    }
    let mut hash_vec: Vec<(&String, &u16)> = counter.iter().collect();
    hash_vec.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", hash_vec);
}

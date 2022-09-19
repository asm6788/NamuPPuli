use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct NamuWiki {
    title: String,
    text: String,
}
fn main() {
    let data = fs::read_to_string(r"F:\namuwiki210301\namuwiki_20210301.json")
        .expect("Unable to read file");
    let out: Vec<NamuWiki> = serde_json::from_str(&data).unwrap();

    for data in out.iter() {
        println!("{:#?} 의 내용\n{:#?}", data.title, data.text);
    }
}

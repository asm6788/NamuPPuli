use rustc_hash::FxHashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{str, thread, time};

fn read_n<R>(reader: R, bytes_to_read: u64) -> Vec<u8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    // Do appropriate error handling for your situation
    // Maybe it's OK if you didn't read enough bytes?
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    assert_eq!(bytes_to_read as usize, n);
    buf
}

fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    match s.find(p) {
        Some(index) => &s[..index],
        None => &s,
    }
}

fn main() {
    let mut reader = BufReader::new(File::open(r"test.json").unwrap());
    let mut counter: FxHashMap<String, u16> = FxHashMap::default();
    let mut record = vec![];
    let mut do_record = false;
    let mut buf = vec![0, 0];
    loop {
        buf.rotate_left(1);
        buf[1] = read_n(&mut reader, 1)[0];

        if buf[0] == b'[' && buf[1] == b'[' {
            do_record = true;
        } else if buf[0] == b']' && buf[1] == b']' {
            do_record = false;
        }

        if do_record {
            record.push(buf[1]);
        } else {
            if record.len() > 0 {
                let mut href = str::from_utf8(&record).unwrap();
                href = &href[1..href.len() - 1];
                if href.starts_with("파일:")
                    || href.starts_with("분류:")
                    || href.starts_with("틀:")
                    || href.starts_with("http")
                {
                    record.clear();
                    continue;
                }

                href = remove_suffix(remove_suffix(&href, "|"), "#");
                *counter.entry(String::from(href)).or_insert(0) += 1;
                println!("{}가 {}번 참고됨", href, counter[&String::from(href)]);
                record.clear();
            }
        }
    }
}

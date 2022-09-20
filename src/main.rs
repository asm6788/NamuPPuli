use dashmap::setref::multiple::RefMulti;
use dashmap::DashMap;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str;

fn read_n<R>(reader: R, bytes_to_read: u64) -> Result<Vec<u8>, i8>
where
    R: Read,
{
    let mut buf = vec![];
    let mut chunk = reader.take(bytes_to_read);
    // Do appropriate error handling for your situation
    // Maybe it's OK if you didn't read enough bytes?
    let n = chunk.read_to_end(&mut buf).expect("Didn't read enough");
    if bytes_to_read as usize == n {
        Ok(buf)
    } else {
        Err(-1)
    }
}

fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    match s.find(p) {
        Some(index) => &s[..index],
        None => &s,
    }
}

fn main() {
    let file = File::open(r"test.json").unwrap();
    let pb = ProgressBar::new(file.metadata().unwrap().len());
    let mut reader = BufReader::new(file);

    let counter: DashMap<String, u16> = DashMap::new();
    let mut record = vec![];
    let mut do_record = false;
    let mut buf = vec![0, 0];
    loop {
        pb.inc(1);
        buf.rotate_left(1);
        buf[1] = match read_n(&mut reader, 1) {
            Ok(v) => v[0],
            Err(_) => break,
        };

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
                record.clear();
            }
        }
    }

    let mut hash_vec: Vec<_> = counter.iter().collect();
    hash_vec.sort_by(|a, b| b.cmp(a));
    for line in hash_vec.into_iter() {
        println!("{}: {}", line.key(), line.value());
    }
}

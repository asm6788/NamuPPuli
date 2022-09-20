use dashmap::DashMap;
use indicatif::ProgressBar;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::str;
use std::sync::Arc;
use std::thread;

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
    let pb = Arc::new(ProgressBar::new(file.metadata().unwrap().len()));
    let counter_d = DashMap::new();
    let counter: Arc<DashMap<String, u16>> = Arc::new(counter_d);

    let thread_count = num_cpus::get() as u64;
    let each_size = file.metadata().unwrap().len() / thread_count;

    for i in 0..thread_count {
        let counter = counter.clone();
        let pb = pb.clone();
        let current_job = thread::spawn(move || {
            let mut reader = BufReader::new(File::open(r"test.json").unwrap());
            reader.seek(SeekFrom::Start(each_size * i)).unwrap();
            let mut buf = vec![0, 0];
            let mut record = vec![];
            let mut do_record = false;
            loop {
                pb.inc(1);
                if reader.seek(SeekFrom::Current(0)).unwrap() == each_size * (i + 1) {
                    break;
                }

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
                } else if record.len() > 0 {
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
        });

        if i == thread_count - 1 {
            current_job.join().unwrap();
        }
    }

    let mut hash_vec: Vec<_> = counter.iter().collect();
    hash_vec.sort_by(|a, b| b.cmp(a));
    for line in hash_vec.into_iter() {
        println!("{}: {}", line.key(), line.value());
    }
}

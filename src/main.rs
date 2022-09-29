use clap::Parser;
use dashmap::DashMap;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use linecount::count_lines;
use petgraph::dot::Config;
use petgraph::dot::Dot;
use petgraph::Graph;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::str;
use std::sync::Arc;
use std::thread;

#[derive(Eq, Hash, Debug)]
struct Link {
    href: (String, String), //시작점 도착점
}

impl PartialEq for Link {
    fn eq(&self, other: &Self) -> bool {
        self.href.0 == other.href.1 || self.href.1 == other.href.0 || self.href == other.href
    }
}

fn remove_suffix<'a>(s: &'a str, p: &str) -> &'a str {
    match s.find(p) {
        Some(index) => &s[..index],
        None => &s,
    }
}

#[derive(Parser)]
#[clap(
    author = "Yuhwan Kim(@asm6788)",
    version,
    about = "나무위키 데이터 파싱 프로그램"
)]
struct Cli {
    #[arg(short, long)]
    namu_db: Option<String>,
    #[arg(short, long)]
    parsed_db: Option<String>,
    #[arg(short, long)]
    csv_export: bool,
    #[arg(short, long)]
    dot_export: bool,
    #[arg(short, long, default_value_t = true)]
    sort: bool,
}

fn main() {
    let args = Cli::parse();
    let mut graph = Graph::<String, u32>::new();
    let mut node_map = HashMap::new();
    if args.namu_db.is_some() {
        let counter_d = DashMap::new();
        let counter: Arc<DashMap<Link, u32>> = Arc::new(counter_d);

        let mut file = File::open(args.namu_db.unwrap()).unwrap();
        let mut buf = Arc::new(Vec::new());
        file.read_to_end(Arc::get_mut(&mut buf).unwrap()).unwrap();

        let pb = Arc::new(ProgressBar::new(buf.len() as u64));
        pb.set_style(
            ProgressStyle::with_template(
                "[{elapsed_precise}] {bar:60.cyan/blue} {pos:>7}/{len:7} {bytes_per_sec} {eta} {msg}",
            )
            .unwrap()
            .progress_chars("##-"),
        );

        let thread_count = num_cpus::get() as u64;
        let each_size = file.metadata().unwrap().len() / thread_count;

        for i in 0..thread_count {
            let counter = counter.clone();
            let pb = pb.clone();
            let mut index = 0;
            let buf = buf.clone();
            let current_job = thread::spawn(move || {
                let mut record = vec![];
                let mut do_record = false;
                let mut is_title = false;
                let mut title: String = String::new();
                loop {
                    pb.inc(1);

                    if (each_size * i + index + 7) as usize >= buf.len()
                        || each_size * i + index + 7 >= each_size * (i + 1)
                    {
                        break;
                    }

                    let buf = &buf
                        [(each_size * i + index) as usize..=(each_size * i + index + 7) as usize];
                    index += 1;

                    if buf[0] == b'[' && buf[1] == b'[' {
                        do_record = true;
                    } else if buf[0] == b']' && buf[1] == b']' {
                        do_record = false;
                    } else if buf == b"\"title\":" {
                        if record.len() > 0 {
                            eprintln!("!!!!에러문서!!!!: {}", title);
                            record.clear();
                        }
                        is_title = true;
                        do_record = true;
                    } else if buf == b",\"text\":" || buf == b",\"contrib" {
                        do_record = false;
                    }

                    if do_record {
                        record.push(buf[1]);
                    } else if record.len() > 0 {
                        if is_title {
                            let temp = str::from_utf8(&record).unwrap();
                            title = (&temp[8..temp.len() - 2]).to_string();
                            is_title = false;
                            pb.set_message(title.to_string());
                            record.clear();
                        } else {
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
                            *counter
                                .entry(Link {
                                    href: (title.to_string(), href.to_string()),
                                })
                                .or_insert(0) += 1;
                            record.clear();
                        }
                    }
                }
            });
            if i == thread_count - 1 {
                current_job.join().unwrap();
            }
        }
        pb.finish();

        let block = [
            Regex::new(r"(\d+)세기").unwrap(),
            Regex::new(r"(\d+)년").unwrap(),
            Regex::new(r"(\d+)월 (\d+)일").unwrap(),
        ];

        let mut hash_vec: Vec<_> = counter.iter().collect();

        if args.sort {
            hash_vec.sort_by(|a, b| b.cmp(a));
        }

        for line in hash_vec.into_iter() {
            if !block
                .iter()
                .map(|v| v.is_match(&*line.key().href.0))
                .collect::<Vec<bool>>()
                .contains(&true)
                && !block
                    .iter()
                    .map(|v| v.is_match(&*line.key().href.1))
                    .collect::<Vec<bool>>()
                    .contains(&true)
                && *line.value() > 1
            {
                if args.csv_export {
                    println!(
                        "\"{}\",\"{}\",{}",
                        line.key().href.0,
                        line.key().href.1,
                        line.value()
                    );
                } else {
                    let origin = *node_map
                        .entry((*line.key().href.0).to_string())
                        .or_insert(graph.add_node((*line.key().href.0).to_string()));
                    let dest = *node_map
                        .entry((*line.key().href.1).to_string())
                        .or_insert(graph.add_node((*line.key().href.1).to_string()));
                    graph.add_edge(origin, dest, *line.value());
                }
            }
        }
    } else if args.parsed_db.is_some() {
        let path = args.parsed_db.unwrap();
        let pb =
            ProgressBar::new(count_lines(File::open(path.to_string()).unwrap()).unwrap() as u64);
        let mut rdr = csv::Reader::from_path(path.to_string()).unwrap();
        let record = rdr.records();

        for result in record {
            pb.inc(1);
            match result {
                Ok(result) => {
                    let origin = *node_map
                        .entry(result[0].to_string())
                        .or_insert(graph.add_node(result[0].to_string()));
                    let dest = *node_map
                        .entry(result[1].to_string())
                        .or_insert(graph.add_node(result[1].to_string()));
                    graph.add_edge(origin, dest, result[2].parse::<u32>().unwrap());
                }
                Err(_) => {
                    continue;
                }
            }
        }
        pb.finish();
    } else {
        eprintln!("데이터 경로가 없습니다.");
        return;
    }

    if args.dot_export {
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeIndexLabel]));
    }
    eprintln!("데이터 처리완료.\n");
    if !args.csv_export && !args.dot_export {
        eprintln!("검색어를 입력하세요.");
        loop {
            let edge = std::io::stdin().lines().next().unwrap().unwrap();

            match node_map.get(&edge) {
                Some(a) => {
                    let mut temp = Vec::new();
                    for node in graph.neighbors_directed(*a, petgraph::Direction::Outgoing) {
                        match graph.edges_connecting(node_map[&edge], node).next() {
                            Some(p) => temp.push((&graph[node], p.weight())),
                            None => continue,
                        }
                    }
                    temp.sort_by(|a, b| b.1.cmp(a.1));
                    for (a, b) in temp {
                        println!("{} -> {} {}", edge, a, b);
                    }

                    println!("-----------------");

                    let mut temp = Vec::new();
                    for node in graph.neighbors_directed(*a, petgraph::Direction::Incoming) {
                        match graph.edges_connecting(node_map[&edge], node).next() {
                            Some(p) => temp.push((&graph[node], p.weight())),
                            None => continue,
                        }
                    }
                    temp.sort_by(|a, b| b.1.cmp(a.1));
                    for (a, b) in temp {
                        println!("{} <- {} {}", edge, a, b);
                    }
                }
                None => {
                    println!("해당 자료 없음!");
                    continue;
                }
            }
        }
    }
}

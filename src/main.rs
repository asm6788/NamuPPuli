use clap::Parser;
use dashmap::DashMap;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use linecount::count_lines;
use petgraph::dot::Config;
use petgraph::dot::Dot;
use petgraph::Graph;
use regex::Regex;
use std::collections::hash_map::Entry;
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
#[clap(about = "나무위키 데이터 파싱 프로그램")]
struct Cli {
    #[arg(short, long)]
    namu_db: Option<String>,
    #[arg(short, long)]
    parsed_db: Option<String>,
    #[arg(short, long)]
    csv_export: bool,
    #[arg(short, long)]
    dot_export: bool,
    #[arg(short = 'D', long)]
    neighbor_dot_export: bool,
    #[arg(long, value_delimiter = ',')]
    stopword: Vec<String>,
    #[arg(short, long)]
    frequency: bool,
    #[arg(short, long)]
    sort: bool,
    #[arg(long, default_value_t = 1)]
    depth: u8,
}

fn main() {
    let args = Cli::parse();
    if args.frequency {
        if !args.csv_export {
            eprintln!("빈도분석이 켜졌지만 csv출력이 켜져있지않습니다. 자동으로 csv로 출력합니다.");
        }

        if args.namu_db.is_none() {
            eprintln!("빈도분석이 켜졌지만 나무위키 덤프경로를 알수가 없습니다. 종료합니다.");
        }
    }

    let mut graph = Graph::<String, u32>::new();
    let mut node_map = HashMap::new();
    if args.namu_db.is_some() {
        let counter_d = DashMap::new();
        let counter: Arc<DashMap<Link, u32>> = Arc::new(counter_d);

        let freq_counter_d = DashMap::new();
        let freq_counter: Arc<DashMap<String, u32>> = Arc::new(freq_counter_d);

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
            let freq_counter = freq_counter.clone();
            let pb = pb.clone();
            let mut index = 0;
            let buf = buf.clone();
            let current_job = thread::spawn(move || {
                let mut record = vec![];
                let mut do_record = false;
                let mut is_title = false;
                let mut title: String = String::new();
                loop {
                    if i == thread_count - 1 {
                        pb.inc(thread_count);
                    }

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
                            if i == thread_count - 1 {
                                //보는맛은 있어야지
                                pb.set_message(title.to_string());
                            }
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

                            //단순 카운팅
                            if args.frequency {
                                *freq_counter.entry(String::from(href)).or_insert(0) += 1;
                            } else {
                                *counter
                                    .entry(Link {
                                        href: (title.to_string(), href.to_string()),
                                    })
                                    .or_insert(0) += 1;
                            }
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

        // 데이터 취합 시작
        let block = [
            Regex::new(r"(\d+)세기").unwrap(),
            Regex::new(r"(\d+)년").unwrap(),
            Regex::new(r"(\d+)월 (\d+)일").unwrap(),
        ];

        let mut hash_vec: Vec<_> = counter.iter().collect();

        if args.frequency {
            let mut hash_vec: Vec<_> = freq_counter.iter().collect();
            hash_vec.sort_by(|a, b| b.cmp(a));
            for line in hash_vec.into_iter() {
                println!("\"{}\",{}", line.key(), line.value());
            }
            return;
        } else if args.sort {
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
                    let origin = *match node_map.entry((*line.key().href.0).to_string()) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => {
                            v.insert(graph.add_node((*line.key().href.0).to_string()))
                        }
                    };

                    let dest = *match node_map.entry((*line.key().href.1).to_string()) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => {
                            v.insert(graph.add_node((*line.key().href.1).to_string()))
                        }
                    };
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
                    let origin = *match node_map.entry(result[0].to_string()) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(graph.add_node(result[0].to_string())),
                    };

                    let dest = *match node_map.entry(result[1].to_string()) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(graph.add_node(result[1].to_string())),
                    };

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
        println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    }

    eprintln!("데이터 처리완료.\n");
    if !args.csv_export && !args.dot_export {
        eprintln!("검색어를 입력하세요.");
        loop {
            let parents = match std::io::stdin().lines().next() {
                Some(a) => a.unwrap(),
                None => {
                    return;
                }
            };

            match node_map.get(&parents) {
                Some(a) => {
                    let mut result = Graph::<String, u32>::new();
                    search_neighbors(
                        &graph,
                        *a,
                        0,
                        args.depth,
                        &args.stopword,
                        &mut HashMap::new(),
                        args.neighbor_dot_export,
                        &mut result,
                    );
                    if args.neighbor_dot_export {
                        println!("{:?}", Dot::with_config(&result, &[Config::EdgeNoLabel]));
                    }
                }
                None => {
                    eprintln!("검색어를 찾을 수 없습니다.");
                }
            }
        }
    }
}

fn search_neighbors(
    graph: &Graph<String, u32>,
    atarget: petgraph::graph::NodeIndex,
    depth: u8,
    max_depth: u8,
    stopword: &Vec<String>,
    map: &mut HashMap<String, petgraph::graph::NodeIndex>,
    neighbor_dot_export: bool,
    result: &mut Graph<String, u32>,
) {
    if depth >= max_depth {
        return;
    }

    if stopword.contains(&graph[atarget]) {
        return;
    }

    let origin = *match map.entry(graph[atarget].to_string()) {
        Entry::Occupied(o) => o.into_mut(),
        Entry::Vacant(v) => v.insert(result.add_node(graph[atarget].to_string())),
    };

    for i in 0..2 {
        let mut neighbors = graph
            .neighbors_directed(
                atarget,
                if i == 0 {
                    petgraph::Direction::Outgoing
                } else {
                    petgraph::Direction::Incoming
                },
            )
            .detach();

        while let Some((edge, target)) = neighbors.next(&graph) {
            if atarget != target {
                if !neighbor_dot_export {
                    if i == 0 {
                        println!("{} -> {} ({})", graph[atarget], graph[target], graph[edge]);
                    } else {
                        println!("{} <- {} ({})", graph[atarget], graph[target], graph[edge]);
                    }
                } else {
                    let dest = *match map.entry(graph[target].to_string()) {
                        Entry::Occupied(o) => o.into_mut(),
                        Entry::Vacant(v) => v.insert(result.add_node(graph[target].to_string())),
                    };

                    if i == 0 {
                        result.add_edge(origin, dest, graph[edge]);
                    } else {
                        result.add_edge(dest, origin, graph[edge]);
                    }

                    search_neighbors(
                        graph,
                        target,
                        depth + 1,
                        max_depth,
                        stopword,
                        map,
                        neighbor_dot_export,
                        result,
                    );
                }
            }
        }
        if !neighbor_dot_export {
            println!("--------------------------------------");
        }
    }
}

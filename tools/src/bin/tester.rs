use std::{
    collections::HashSet,
    fs,
    io::{Read, Write},
    path::PathBuf,
    process::Stdio,
    thread,
};

fn exec(file_path: PathBuf, print_flag: bool, out_flag: bool) -> (i64, u8) {
    let mut file = fs::File::open(&file_path).unwrap();
    let mut buf = vec![];
    file.read_to_end(&mut buf).unwrap_or_else(|e| {
        eprintln!("failed to read {:?}", file);
        eprintln!("{}", e);
        std::process::exit(1)
    });
    let n = (buf[0] - b'0') * 10 + (buf[1] - b'0');
    let command = "cargo";
    let p = std::process::Command::new(command)
        .arg("run")
        .arg("--release")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("failed to execute the command");
            eprintln!("{}", e);
            std::process::exit(1)
        });
    let mut stdin = p.stdin.as_ref().unwrap();
    stdin.write_all(&buf).unwrap();
    let output = p.wait_with_output().unwrap();
    if out_flag {
        let file_path = format!(
            "./tools/out/{}",
            file_path.file_name().unwrap().to_string_lossy()
        );
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(&output.stdout).unwrap();
    }
    let b = String::from_utf8(output.stderr).unwrap();
    let s = b.split('\n').collect::<Vec<_>>();
    let score = s[s.len() - 2].split(':').collect::<Vec<_>>();
    if print_flag {
        println!(
            "{}|{}:{}",
            file_path.display(),
            score[0],
            score[1].parse::<i64>().unwrap()
        );
    }
    if let Ok(score) = score[1].parse::<i64>() {
        (score, n)
    } else {
        eprintln!(
            "failed ./tools/out/{}",
            file_path.file_name().unwrap().to_string_lossy()
        );
        (0, n)
    }
}

fn main() {
    let out_flag = std::env::args().into_iter().any(|s| &s == "out");
    let avg_flag = std::env::args().into_iter().any(|s| &s == "avg");
    let exec_list = std::env::args()
        .into_iter()
        .skip(1)
        .filter(|s| s.parse::<usize>().is_ok())
        .map(|s| {
            // println!("{:04}.txt", s.parse::<usize>().unwrap());
            format!("{:04}.txt", s.parse::<usize>().unwrap())
        })
        .collect::<HashSet<_>>();
    let files = fs::read_dir("./tools/in/").unwrap();
    let mut handles = vec![];
    for file in files {
        let file = file.unwrap();
        if !exec_list.is_empty() && !exec_list.contains(file.file_name().to_str().unwrap()) {
            continue;
        }
        let file_path = file.path();
        let print_flag = !exec_list.is_empty();
        let handle = thread::spawn(move || exec(file_path, print_flag, out_flag));
        handles.push(handle);
    }
    let mut total_score = 0;
    let case_num = handles.len();
    let mut scores = vec![vec![]; 62];
    for handle in handles {
        let (score, n) = handle.join().unwrap();
        total_score += score;
        scores[(n as usize) / 10].push(score);
    }
    const PRETESTNUM: i64 = 50;
    println!("total_score:{}", total_score * PRETESTNUM / case_num as i64);
    if avg_flag {
        for (n, scores) in scores.iter().enumerate() {
            if scores.is_empty() {
                continue;
            }
            println!(
                "n=({}*), avg_score={}",
                n,
                scores.iter().sum::<i64>() * PRETESTNUM / scores.len() as i64
            );
        }
    }
}

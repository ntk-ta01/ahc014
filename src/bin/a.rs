const TIMELIMIT: f64 = 4.8;

const DXY: [Point; 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (!0, 1),
    (!0, 0),
    (!0, !0),
    (0, !0),
    (1, !0),
];

type Output = Vec<[Point; 4]>;

fn main() {
    let input = Input::new();
    let output: Output = vec![];
    println!("{}", output.len());
    for out in output.iter() {
        print!("{} {} ", out[0].0, out[0].1);
        print!("{} {} ", out[1].0, out[1].1);
        print!("{} {} ", out[2].0, out[2].1);
        println!("{} {}", out[3].0, out[3].1);
    }
    let score = compute_score(&input, &output);
    eprintln!("score:{}", score);
}

struct State {
    has_point: Vec<Vec<bool>>,
    used: Vec<Vec<[bool; 8]>>,
}

#[derive(Clone, Debug)]
struct Input {
    n: usize,
    ps: Vec<Point>,
}

impl Input {
    fn new() -> Input {
        use proconio::input;
        input! {
            n: usize,
            m: usize,
            ps: [(usize, usize); m],
        }
        Input { n, ps }
    }
}

type Point = (usize, usize);

fn weight((x, y): Point, n: usize) -> i64 {
    let dx = x as i64 - n as i64 / 2;
    let dy = y as i64 - n as i64 / 2;
    dx * dx + dy * dy + 1
}

fn compute_score(input: &Input, out: &[[Point; 4]]) -> i64 {
    let mut num = 0;
    for &p in &input.ps {
        num += weight(p, input.n);
    }
    for rect in out {
        num += weight(rect[0], input.n);
    }
    let mut den = 0;
    for i in 0..input.n {
        for j in 0..input.n {
            den += weight((i, j), input.n);
        }
    }
    (1e6 * (input.n * input.n) as f64 / input.ps.len() as f64 * num as f64 / den as f64).round()
        as i64
}

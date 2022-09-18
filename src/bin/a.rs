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
    let score_weight = ScoreWeight::new(&input);
    let output: Output = vec![];
    println!("{}", output.len());
    for out in output.iter() {
        print!("{} {} ", out[0].0, out[0].1);
        print!("{} {} ", out[1].0, out[1].1);
        print!("{} {} ", out[2].0, out[2].1);
        println!("{} {}", out[3].0, out[3].1);
    }
    let score = compute_score(&input, &output, &score_weight);
    eprintln!("score:{}", score);
}

struct State {
    has_point: Vec<Vec<bool>>,
    used: Vec<Vec<[bool; 8]>>,
}

impl State {
    fn new(input: &Input) -> Self {
        let mut has_point = vec![vec![false; input.n]; input.n];
        let used = vec![vec![[false; 8]; input.n]; input.n];
        for i in 0..input.ps.len() {
            has_point[input.ps[i].0][input.ps[i].1] = true;
        }
        Self { has_point, used }
    }
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

struct ScoreWeight {
    ps_weight: i64,
    den: f64,
}

impl ScoreWeight {
    fn new(input: &Input) -> Self {
        let mut ps_weight = 0;
        for &p in &input.ps {
            ps_weight += weight(p, input.n);
        }
        let mut den = 0;
        for i in 0..input.n {
            for j in 0..input.n {
                den += weight((i, j), input.n);
            }
        }
        let den = den as f64;
        ScoreWeight { ps_weight, den }
    }
}

fn compute_score(input: &Input, out: &[[Point; 4]], score_weight: &ScoreWeight) -> i64 {
    let mut num = score_weight.ps_weight;
    for rect in out {
        num += weight(rect[0], input.n);
    }
    (1e6 * (input.n * input.n) as f64 / input.ps.len() as f64 * num as f64
        / score_weight.den as f64)
        .round() as i64
}

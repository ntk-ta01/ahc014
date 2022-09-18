use rand::prelude::*;
use std::cmp;

const TIMELIMIT: f64 = 4.2;

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
    let timer = Timer::new();
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);
    let input = Input::new();
    let score_weight = ScoreWeight::new(&input);

    let mut best_output = vec![];
    let mut best_score = 0;
    while timer.get_time() < TIMELIMIT {
        let mut output = vec![];
        greedy(&input, &mut output, &score_weight, &mut rng);
        let score = compute_score(&input, &output, &score_weight);
        if best_score < score {
            best_output = output;
            best_score = score;
        }
    }
    println!("{}", best_output.len());
    for out in best_output.iter() {
        print!("{} {} ", out[0].0, out[0].1);
        print!("{} {} ", out[1].0, out[1].1);
        print!("{} {} ", out[2].0, out[2].1);
        println!("{} {}", out[3].0, out[3].1);
    }
    eprintln!("score:{}", best_score);
}

fn greedy<T: Rng>(input: &Input, out: &mut Output, _score_weight: &ScoreWeight, rng: &mut T) {
    // O(n^3)で印の打点候補を列挙する
    // 打点候補が空になるまで重みのroulette-whell-selectionで打点
    let mut state = State::new(input);
    let mut insertable = construct_insertable(input, &state);
    // insertableをsort
    insertable.sort_by_key(|rect| cmp::Reverse(weight(rect[0], input.n)));
    while !insertable.is_empty() {
        let rect = select_insertable(input, rng, &insertable);
        state.apply_move(&rect);
        out.push(rect);
        insertable = construct_insertable(input, &state);
        insertable.sort_by_key(|rect| cmp::Reverse(weight(rect[0], input.n)));
    }
}

fn construct_insertable(input: &Input, state: &State) -> Vec<[Point; 4]> {
    let mut insertable = vec![];
    for i in 0..input.n {
        for j in 0..input.n {
            if state.has_point[i][j] {
                continue;
            }
            let p0 = (i, j);
            // p0に対してp1, p2, p3を探す
            // p0の周り8点を列挙して、4C2ずつrect[2]が打点可能でcheck_moveを通るかチェック
            let mut even_points = vec![];
            let mut odd_points = vec![];
            for (i, &(dx, dy)) in DXY.iter().enumerate() {
                let (mut x, mut y) = p0;
                let mut inserted = false;
                x += dx;
                y += dy;
                while x < input.n && y < input.n {
                    if state.has_point[x][y] {
                        if i % 2 == 0 {
                            even_points.push((x, y));
                        } else {
                            odd_points.push((x, y));
                        }
                        inserted = true;
                        break;
                    }
                    x += dx;
                    y += dy;
                }
                if !inserted {
                    if i % 2 == 0 {
                        even_points.push((!0, !0));
                    } else {
                        odd_points.push((!0, !0));
                    }
                }
            }
            for (i, &p1) in even_points.iter().enumerate() {
                if p1 == (!0, !0) {
                    continue;
                }
                for &p3 in even_points.iter().skip(i + 1) {
                    if p3 == (!0, !0) {
                        continue;
                    }
                    let dx03 = p3.0 as i64 - p0.0 as i64;
                    let dy03 = p3.1 as i64 - p0.1 as i64;
                    let p2 = ((p1.0 as i64 + dx03) as usize, (p1.1 as i64 + dy03) as usize);
                    let rect = [p0, p1, p2, p3];
                    if p2.0 < input.n && p2.1 < input.n && state.check_move(&rect) {
                        insertable.push(rect);
                    }
                }
            }
            for (i, &p1) in odd_points.iter().enumerate() {
                if p1 == (!0, !0) {
                    continue;
                }
                for &p3 in odd_points.iter().skip(i + 1) {
                    if p3 == (!0, !0) {
                        continue;
                    }
                    let dx03 = p3.0 as i64 - p0.0 as i64;
                    let dy03 = p3.1 as i64 - p0.1 as i64;
                    let p2 = ((p1.0 as i64 + dx03) as usize, (p1.1 as i64 + dy03) as usize);
                    let rect = [p0, p1, p2, p3];
                    if p2.0 < input.n && p2.1 < input.n && state.check_move(&rect) {
                        insertable.push(rect);
                    }
                }
            }
        }
    }
    insertable
}

fn select_insertable<T: Rng>(input: &Input, rng: &mut T, insertable: &[[Point; 4]]) -> [Point; 4] {
    let mut weights = vec![0; insertable.len()];
    for (ws, rect) in weights.iter_mut().zip(insertable.iter()) {
        *ws = weight(rect[0], input.n);
    }
    let sum = weights.iter().sum::<i64>() as f64;
    let mut prob = vec![0.0; insertable.len()];
    for (p, w) in prob.iter_mut().zip(weights) {
        *p = w as f64 / sum;
    }
    let mut accum_prob = 0.0;
    for (&rect, &pr) in insertable.iter().zip(prob.iter()) {
        accum_prob += pr;
        if 1.0 < accum_prob || rng.gen_bool(accum_prob) {
            return rect;
        }
    }
    unreachable!();
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

    fn check_move(&self, rect: &[Point; 4]) -> bool {
        if (1..4).any(|i| !self.has_point[rect[i].0][rect[i].1])
            || self.has_point[rect[0].0][rect[0].1]
        {
            return false;
        }
        let dx01 = rect[1].0 as i64 - rect[0].0 as i64;
        let dy01 = rect[1].1 as i64 - rect[0].1 as i64;
        let dx03 = rect[3].0 as i64 - rect[0].0 as i64;
        let dy03 = rect[3].1 as i64 - rect[0].1 as i64;
        if dx01 * dx03 + dy01 * dy03 != 0
            || dx01 != 0 && dy01 != 0 && dx01.abs() != dy01.abs()
            || (rect[1].0 as i64 + dx03, rect[1].1 as i64 + dy03)
                != (rect[2].0 as i64, rect[2].1 as i64)
        {
            return false;
        }
        // 長方形が外周上に印を持つか、長方形が他の長方形との共通部分を持つかを調べる部分だが、
        // 印をつける点の探し方によっては省略可能
        for i in 0..4 {
            let (mut x, mut y) = rect[i];
            let (tx, ty) = rect[(i + 1) % 4];
            let dx = (tx as i64 - x as i64).signum() as usize;
            let dy = (ty as i64 - y as i64).signum() as usize;
            let dir = (0..8).find(|&dir| DXY[dir] == (dx, dy)).unwrap();
            while (x, y) != (tx, ty) {
                if (x, y) != rect[i] && self.has_point[x as usize][y as usize] {
                    return false;
                }
                if self.used[x as usize][y as usize][dir] {
                    return false;
                }
                x += dx;
                y += dy;
                if self.used[x as usize][y as usize][dir ^ 4] {
                    return false;
                }
            }
        }
        true
    }

    fn apply_move(&mut self, rect: &[Point; 4]) {
        self.has_point[rect[0].0][rect[0].1] = true;
        for i in 0..4 {
            let (mut x, mut y) = rect[i];
            let (tx, ty) = rect[(i + 1) % 4];
            let dx = (tx as i64 - x as i64).signum() as usize;
            let dy = (ty as i64 - y as i64).signum() as usize;
            let dir = (0..8).find(|&dir| DXY[dir] == (dx, dy)).unwrap();
            while (x, y) != (tx, ty) {
                self.used[x][y][dir] = true;
                x += dx;
                y += dy;
                self.used[x][y][dir ^ 4] = true;
            }
        }
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

fn get_time() -> f64 {
    let t = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    t.as_secs() as f64 + t.subsec_nanos() as f64 * 1e-9
}

struct Timer {
    start_time: f64,
}

impl Timer {
    fn new() -> Timer {
        Timer {
            start_time: get_time(),
        }
    }

    fn get_time(&self) -> f64 {
        get_time() - self.start_time
    }
}

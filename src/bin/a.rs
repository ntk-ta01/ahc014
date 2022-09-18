use itertools::Itertools;

const TIMELIMIT: f64 = 4.8;

const DXY: [Point; 8] = [
    Point { x: 1, y: 0 },
    Point { x: 1, y: 1 },
    Point { x: 0, y: 1 },
    Point { x: !0, y: 1 },
    Point { x: !0, y: 0 },
    Point { x: !0, y: !0 },
    Point { x: 0, y: !0 },
    Point { x: 1, y: !0 },
];

type Output = Vec<[Point; 4]>;

fn main() {
    let input = Input::new();
    let output: Output = vec![];
    println!("{}", output.len());
    for out in output {
        println!("{} {} {} {}", out[0], out[1], out[2], out[3]);
    }
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
        let ps = ps.into_iter().map(|(x, y)| Point::new(x, y)).collect_vec();
        Input { n, ps }
    }
}

#[derive(Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.x, self.y)?;
        Ok(())
    }
}

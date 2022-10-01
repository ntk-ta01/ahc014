use rand::prelude::*;
use std::{cmp, collections::VecDeque};

const GREEDYTIMELIMIT: f64 = 0.5;
const TIMELIMIT: f64 = 4.95;

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

// optunaで最適化する用
#[allow(dead_code)]
struct ArgParams {
    // t0: f64,
    // t1: f64,
    select_prob: f64,
    // insert_tabu_tenure: usize,
    // remove_tabu_tenure: usize,
    // ratio_l: f64,
    // ratio_r: f64,
}

impl ArgParams {
    #[allow(dead_code)]
    fn new() -> Self {
        let mut args = std::env::args();
        args.next();
        // let t0 = args.next().unwrap().parse::<f64>().unwrap();
        // let t1 = args.next().unwrap().parse::<f64>().unwrap();
        let select_prob = args.next().unwrap().parse::<f64>().unwrap();
        // let insert_tabu_tenure = args.next().unwrap().parse::<usize>().unwrap();
        // let remove_tabu_tenure = args.next().unwrap().parse::<usize>().unwrap();
        // let ratio_l = args.next().unwrap().parse::<f64>().unwrap();
        // let ratio_r = args.next().unwrap().parse::<f64>().unwrap();
        ArgParams {
            // t0,
            // t1,
            select_prob,
            // insert_tabu_tenure,
            // remove_tabu_tenure,
            // ratio_l,
            // ratio_r,
        }
    }
}

fn main() {
    // let params = ArgParams::new();
    let timer = Timer::new();
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(0);
    let input = Input::new();
    let score_weight = ScoreWeight::new(&input);

    let mut best_output = vec![];
    let mut best_score = 0;
    while timer.get_time() < GREEDYTIMELIMIT {
        let mut output = vec![];
        greedy(&input, &mut output, &mut rng);
        let score = compute_score(&input, &output, &score_weight);
        if best_score < score {
            best_output = output;
            best_score = score;
        }
    }
    best_score = annealing(
        &input,
        &mut best_output,
        &score_weight,
        &mut rng,
        timer,
        // params,
    );
    println!("{}", best_output.len());
    for out in best_output.iter() {
        print!("{} {} ", out[0].0, out[0].1);
        print!("{} {} ", out[1].0, out[1].1);
        print!("{} {} ", out[2].0, out[2].1);
        println!("{} {}", out[3].0, out[3].1);
    }
    eprintln!("score:{}", best_score);
}

fn annealing<T: Rng>(
    input: &Input,
    out: &mut Output,
    score_weight: &ScoreWeight,
    rng: &mut T,
    timer: Timer,
    // params: ArgParams,
) -> i64 {
    const T0: f64 = 7991.026452;
    const T1: f64 = 7911.767400;
    const INSERTTABUTENURE: usize = 4;
    // const REMOVETABUTENURE: usize = 81;
    const BACKTOBEST: usize = 16000;
    let back_to_best = BACKTOBEST / input.n;
    let mut temp = T0;
    // let mut temp = params.t0;
    let mut prob;
    let mut now_score = compute_score(input, out, score_weight);

    let mut best_score = now_score;
    let mut best_output = out.clone();
    let mut count = 0;

    let mut insert_tabu_list = VecDeque::new();
    // let mut remove_tabu_list = VecDeque::new();

    // let mut insert_long_memory = BTreeMap::new();
    // let mut max_inserted_time = 0;
    // const FIXRATIO_L: f64 = 0.740019;
    // const FIXRATIO_R: f64 = 0.801759;
    let mut no_improved = 0;
    loop {
        let passed = timer.get_time() / TIMELIMIT;
        if passed >= 1.0 {
            break;
        }
        if count > 100 {
            temp = T0.powf(1.0 - passed) * T1.powf(passed);
            // temp = params.t0.powf(1.0 - passed) * params.t1.powf(passed);
            count = 0;
        }
        count += 1;

        // while remove_tabu_list.len() > REMOVETABUTENURE {
        //     remove_tabu_list.pop_front();
        // }
        let pos = rng.gen_range(0, out.len());
        // if remove_tabu_list.iter().any(|p| *p == out[pos]) {
        //     continue;
        // }

        let mut new_state = State::new(input);
        let mut new_out = vec![];
        // 近傍解作成
        // randomに1個選んで削除
        if !out.is_empty() {
            if rng.gen_bool(0.95) {
                // 削除近傍
                for (i, &rect) in out.iter().enumerate() {
                    if pos == i {
                        insert_tabu_list.push_back(rect[0]);
                        continue;
                    }
                    if new_state.check_move(&rect) {
                        new_state.apply_move(&rect);
                        new_out.push(rect);
                    }
                }
            } else {
                // 長方形変形近傍（変形できなければ削除）
                for (i, &rect) in out.iter().enumerate() {
                    if pos == i {
                        // 長方形変形
                        let mut modify_rects = construct_modify_rects(input, &new_state, &rect);
                        modify_rects.sort_by_key(|rect| {
                            (area(rect), cmp::Reverse(weight(rect[0], input.n)))
                        });
                        if !modify_rects.is_empty() {
                            let mod_rect = select_insertable(input, rng, &modify_rects);
                            new_state.apply_move(&mod_rect);
                            new_out.push(mod_rect);
                        }
                        continue;
                    }
                    if new_state.check_move(&rect) {
                        new_state.apply_move(&rect);
                        new_out.push(rect);
                    }
                }
            }
        }
        if insert_tabu_list.len() > INSERTTABUTENURE {
            insert_tabu_list.pop_front();
        }
        // let mut inserted_rects = vec![];
        let mut insertable = construct_insertable(input, &new_state, &insert_tabu_list);
        // insertableをsort
        insertable.sort_by_key(|rect| (area(rect), cmp::Reverse(weight(rect[0], input.n))));
        if rng.gen_bool(0.959291824) {
            while !insertable.is_empty() {
                let rect = select_insertable(input, rng, &insertable);
                // let rect = insertable[0];
                new_state.apply_move(&rect);
                out.push(rect);
                // inserted_rects.push(rect);
                update_insertable(
                    input,
                    &new_state,
                    rect[0],
                    &mut insertable,
                    &insert_tabu_list,
                );
                insertable.sort_by_key(|rect| (area(rect), cmp::Reverse(weight(rect[0], input.n))));
            }
        } else {
            while !insertable.is_empty() {
                let rect = select_insertable2(input, &new_state, rng, &insertable);
                // let rect = insertable[0];
                new_state.apply_move(&rect);
                out.push(rect);
                // inserted_rects.push(rect);
                update_insertable(
                    input,
                    &new_state,
                    rect[0],
                    &mut insertable,
                    &insert_tabu_list,
                );
                insertable.sort_by_key(|rect| (area(rect), cmp::Reverse(weight(rect[0], input.n))));
            }
        }

        // 近傍解作成ここまで
        let new_score = compute_score(input, &new_out, score_weight);
        prob = f64::exp((new_score - now_score) as f64 / temp);
        if now_score < new_score || rng.gen_bool(prob) {
            now_score = new_score;
            *out = new_out;
        }

        // for rect in inserted_rects {
        //     let e = insert_long_memory.entry(rect).or_insert(0);
        //     *e += 1;
        //     max_inserted_time = max_inserted_time.max(*e);
        // }

        // for (&rect, insert_time) in insert_long_memory.iter() {
        //     let ratio = *insert_time as f64 / max_inserted_time as f64;
        //     if FIXRATIO_L < ratio && ratio < FIXRATIO_R {
        //         remove_tabu_list.push_back(rect);
        //     }
        // }

        if best_score < now_score {
            // eprintln!("time: {}", timer.get_time());
            // eprintln!("no improved: {} / passed: {:.3}", no_improved, passed);
            no_improved = 0;
            best_score = now_score;
            best_output = out.clone();
        } else {
            no_improved += 1;
        }

        if back_to_best < no_improved {
            now_score = best_score;
            *out = best_output.clone();
            no_improved = 0;
        }
    }
    // eprintln!("no improved: {}", no_improved);
    *out = best_output;
    best_score
}

fn greedy<T: Rng>(input: &Input, out: &mut Output, rng: &mut T) {
    // 始めにO(n^3)で印の打点候補を列挙する
    // 打点候補が空になるまで重みのroulette-wheel-selectionで打点
    // 印の打点候補の更新はO(n^2)
    let mut state = State::new(input);
    let insert_tabu_list = VecDeque::new();
    let mut insertable = construct_insertable(input, &state, &insert_tabu_list);
    // insertableをsort
    insertable.sort_by_key(|rect| (area(rect), cmp::Reverse(weight(rect[0], input.n))));
    while !insertable.is_empty() {
        let rect = select_insertable(input, rng, &insertable);
        // let rect = insertable[0];
        state.apply_move(&rect);
        out.push(rect);
        // insertable = construct_insertable(input, &state);
        update_insertable(input, &state, rect[0], &mut insertable, &insert_tabu_list);
        insertable.sort_by_key(|rect| (area(rect), cmp::Reverse(weight(rect[0], input.n))));
    }
}

fn construct_insertable(
    input: &Input,
    state: &State,
    insert_tabu_list: &VecDeque<Point>,
) -> Vec<[Point; 4]> {
    let mut insertable = vec![];
    for (i, row) in state.has_point.iter().enumerate() {
        for (j, _) in row.iter().enumerate().filter(|(_, has)| !*has) {
            let p0 = (i, j);
            if insert_tabu_list.iter().any(|p| *p == p0) {
                continue;
            }
            // p0に対してp1, p2, p3を探す
            // p0の周り8点を列挙して、4C2ずつrect[2]が打点可能でcheck_moveを通るかチェック
            let mut even_points = vec![];
            let mut odd_points = vec![];
            'construct_p0: for (i, &(dx, dy)) in DXY.iter().enumerate() {
                let (mut x, mut y) = p0;
                x += dx;
                y += dy;
                while x < input.n && y < input.n {
                    if state.has_point[x][y] {
                        if i % 2 == 0 {
                            even_points.push((x, y));
                        } else {
                            odd_points.push((x, y));
                        }
                        continue 'construct_p0;
                    }
                    x += dx;
                    y += dy;
                }
                if i % 2 == 0 {
                    even_points.push((!0, !0));
                } else {
                    odd_points.push((!0, !0));
                }
            }
            for (i, &p1) in even_points.iter().enumerate() {
                if p1 == (!0, !0) {
                    continue;
                }
                for &p3 in even_points.iter().skip(i + 1).step_by(2) {
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
                for &p3 in odd_points.iter().skip(i + 1).step_by(2) {
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

fn construct_modify_rects(
    input: &Input,
    state: &State,
    original_rect: &[Point; 4],
) -> Vec<[Point; 4]> {
    let p0 = original_rect[0];
    let mut insertable = vec![];
    // p0に対してp1, p2, p3を探す
    // p0の周り8点を列挙して、4C2ずつrect[2]が打点可能でcheck_moveを通るかチェック
    let mut even_points = vec![];
    let mut odd_points = vec![];
    'construct_p0: for (i, &(dx, dy)) in DXY.iter().enumerate() {
        let (mut x, mut y) = p0;
        x += dx;
        y += dy;
        while x < input.n && y < input.n {
            if state.has_point[x][y] {
                if i % 2 == 0 {
                    even_points.push((x, y));
                } else {
                    odd_points.push((x, y));
                }
                continue 'construct_p0;
            }
            x += dx;
            y += dy;
        }
        if i % 2 == 0 {
            even_points.push((!0, !0));
        } else {
            odd_points.push((!0, !0));
        }
    }
    for (i, &p1) in even_points.iter().enumerate() {
        if p1 == (!0, !0) {
            continue;
        }
        for &p3 in even_points.iter().skip(i + 1).step_by(2) {
            if p3 == (!0, !0) {
                continue;
            }
            let dx03 = p3.0 as i64 - p0.0 as i64;
            let dy03 = p3.1 as i64 - p0.1 as i64;
            let p2 = ((p1.0 as i64 + dx03) as usize, (p1.1 as i64 + dy03) as usize);
            let rect = [p0, p1, p2, p3];
            if p2.0 < input.n && p2.1 < input.n && rect != *original_rect && state.check_move(&rect)
            {
                insertable.push(rect);
            }
        }
    }
    for (i, &p1) in odd_points.iter().enumerate() {
        if p1 == (!0, !0) {
            continue;
        }
        for &p3 in odd_points.iter().skip(i + 1).step_by(2) {
            if p3 == (!0, !0) {
                continue;
            }
            let dx03 = p3.0 as i64 - p0.0 as i64;
            let dy03 = p3.1 as i64 - p0.1 as i64;
            let p2 = ((p1.0 as i64 + dx03) as usize, (p1.1 as i64 + dy03) as usize);
            let rect = [p0, p1, p2, p3];
            if p2.0 < input.n && p2.1 < input.n && rect != *original_rect && state.check_move(&rect)
            {
                insertable.push(rect);
            }
        }
    }
    insertable
}

fn update_insertable(
    input: &Input,
    state: &State,
    pre_p0: Point,
    insertable: &mut Vec<[Point; 4]>,
    insert_tabu_list: &VecDeque<Point>,
) {
    // 打点によって更新されたstateに合わせてinsertableの各要素をfilter
    *insertable = insertable
        .iter_mut()
        .filter(|rect| state.check_move(rect))
        .map(|rect| *rect)
        .collect();
    // pre_p0から8方向のそれぞれで一番近い点を探す
    let mut near_points = vec![];
    for &(dx, dy) in DXY.iter() {
        let (mut x, mut y) = pre_p0;
        let mut found = false;
        x += dx;
        y += dy;
        while x < input.n && y < input.n {
            if state.has_point[x][y] {
                near_points.push((x, y));
                found = true;
                break;
            }
            x += dx;
            y += dy;
        }
        if !found {
            near_points.push((!0, !0));
        }
    }
    // pre_p0から8方向にp0候補を探しに行く p0候補は!has_pointである
    for (i, &(dx, dy)) in DXY.iter().enumerate() {
        // pre_p0をp1とする方針
        {
            let (mut x, mut y) = pre_p0;
            x += dx;
            y += dy;
            while x < input.n && y < input.n && !state.has_point[x][y] {
                // p0はこれから印を打ちたい点
                let p0 = (x, y);
                if insert_tabu_list.iter().any(|p| *p == p0) {
                    x += dx;
                    y += dy;
                    continue;
                }
                // p1 = pre_p0
                // p2 = (pre_p0から {-2, +2}方向の点)
                // p3を探す
                let p1 = pre_p0;
                let dir = i ^ 4;
                for j in [8 - 2, 2].iter() {
                    let search_dir = (dir + j) % 8;
                    let p2 = near_points[search_dir];
                    if p2 != (!0, !0) {
                        let dx10 = p0.0 as i64 - p1.0 as i64;
                        let dy10 = p0.1 as i64 - p1.1 as i64;
                        let p3 = ((p2.0 as i64 + dx10) as usize, (p2.1 as i64 + dy10) as usize);
                        let rect = [p0, p1, p2, p3];
                        if p3.0 < input.n && p3.1 < input.n && state.check_move(&rect) {
                            insertable.push(rect);
                        }
                    }
                }
                x += dx;
                y += dy;
            }
        }
        {
            // pre_p0をp2とする方針
            // (i+1) % 8方向にp0が存在するか調べる
            // p1 = i方向の点
            // p2 = pre_p0
            // p3 = (i+2) % 8方向の点
            let p1 = near_points[i];
            let p2 = pre_p0;
            let p3 = near_points[(i + 2) % 8];
            if p1 == (!0, !0) || p3 == (!0, !0) {
                continue;
            }
            let dx21 = p1.0 as i64 - p2.0 as i64;
            let dy21 = p1.1 as i64 - p2.1 as i64;
            let p0 = ((p3.0 as i64 + dx21) as usize, (p3.1 as i64 + dy21) as usize);
            if insert_tabu_list.iter().any(|p| *p == p0) {
                continue;
            }
            let rect = [p0, p1, p2, p3];
            if p0.0 < input.n && p0.1 < input.n && state.check_move(&rect) {
                insertable.push(rect);
            }
        }
    }
}

fn select_insertable<T: Rng>(input: &Input, rng: &mut T, insertable: &[[Point; 4]]) -> [Point; 4] {
    let mut weights = vec![0.0; insertable.len()];
    for (ws, rect) in weights.iter_mut().zip(insertable.iter()) {
        let w = weight(rect[0], input.n);
        let area = area(rect);
        *ws = (w * w) as f64 / (area * area * area * area) as f64;
    }
    let mut sum = weights.iter().sum::<f64>();
    if sum < 0.0 || sum.is_nan() || sum.is_infinite() || weights.iter().any(|w| *w < 0.0) {
        for (ws, rect) in weights.iter_mut().zip(insertable.iter()) {
            let w = weight(rect[0], input.n);
            let area = area(rect);
            *ws = (w * w) as f64 / (area * area * area) as f64;
        }
        sum = weights.iter().sum::<f64>();
    }
    let mut prob = vec![0.0; insertable.len()];
    for (p, w) in prob.iter_mut().zip(weights) {
        *p = w / sum;
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

fn select_insertable2<T: Rng>(
    input: &Input,
    state: &State,
    rng: &mut T,
    insertable: &[[Point; 4]],
) -> [Point; 4] {
    let mut weights = vec![0.0; insertable.len()];
    // 各挿入によって増える新規の挿入点の個数を重みとしてroulette-wheel-selection
    for (rect, ws) in insertable.iter().zip(weights.iter_mut()) {
        let mut state = state.clone();
        state.apply_move(rect);
        let inc_num = construct_insertable_one_point(input, &state, rect[0]);
        let w = weight(rect[0], input.n);
        *ws = (w * w * inc_num * inc_num * inc_num) as f64;
    }
    let sum = weights.iter().sum::<f64>();
    let mut prob = vec![0.0; insertable.len()];
    for (p, w) in prob.iter_mut().zip(weights) {
        *p = w / sum;
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

fn construct_insertable_one_point(input: &Input, state: &State, pre_p0: Point) -> i64 {
    // pre_p0を打つと増える打てる点の個数を返す
    // [0, 8]？
    // pre_p0から8方向のそれぞれで一番近い点を探す
    let mut near_points = vec![];
    for &(dx, dy) in DXY.iter() {
        let (mut x, mut y) = pre_p0;
        let mut found = false;
        x += dx;
        y += dy;
        while x < input.n && y < input.n {
            if state.has_point[x][y] {
                near_points.push((x, y));
                found = true;
                break;
            }
            x += dx;
            y += dy;
        }
        if !found {
            near_points.push((!0, !0));
        }
    }
    let mut ret_insertable_num = 0;
    // pre_p0から8方向にp0候補を探しに行く p0候補は!has_pointである
    for (i, &(dx, dy)) in DXY.iter().enumerate() {
        // pre_p0をp1とする方針
        {
            let (mut x, mut y) = pre_p0;
            x += dx;
            y += dy;
            while x < input.n && y < input.n && !state.has_point[x][y] {
                // p0はこれから印を打ちたい点
                let p0 = (x, y);
                // p1 = pre_p0
                // p2 = (pre_p0から {-2, +2}方向の点)
                // p3を探す
                let p1 = pre_p0;
                let dir = i ^ 4;
                for j in [8 - 2, 2].iter() {
                    let search_dir = (dir + j) % 8;
                    let p2 = near_points[search_dir];
                    if p2 != (!0, !0) {
                        let dx10 = p0.0 as i64 - p1.0 as i64;
                        let dy10 = p0.1 as i64 - p1.1 as i64;
                        let p3 = ((p2.0 as i64 + dx10) as usize, (p2.1 as i64 + dy10) as usize);
                        let rect = [p0, p1, p2, p3];
                        if p3.0 < input.n && p3.1 < input.n && state.check_move(&rect) {
                            ret_insertable_num += 1;
                        }
                    }
                }
                x += dx;
                y += dy;
            }
        }
        {
            // pre_p0をp2とする方針
            // (i+1) % 8方向にp0が存在するか調べる
            // p1 = i方向の点
            // p2 = pre_p0
            // p3 = (i+2) % 8方向の点
            let p1 = near_points[i];
            let p2 = pre_p0;
            let p3 = near_points[(i + 2) % 8];
            if p1 == (!0, !0) || p3 == (!0, !0) {
                continue;
            }
            let dx21 = p1.0 as i64 - p2.0 as i64;
            let dy21 = p1.1 as i64 - p2.1 as i64;
            let p0 = ((p3.0 as i64 + dx21) as usize, (p3.1 as i64 + dy21) as usize);
            let rect = [p0, p1, p2, p3];
            if p0.0 < input.n && p0.1 < input.n && state.check_move(&rect) {
                ret_insertable_num += 1;
            }
        }
    }
    ret_insertable_num + 1
}

#[derive(Debug, Clone)]
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
        // 長方形が外周上に印を持つか、長方形が他の長方形との共通部分を持つかを調べる
        // 省略はできない update_insertableで共通部分を持つこともある
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

fn area(rect: &[Point; 4]) -> i64 {
    let dx01 = (rect[1].0 as i64 - rect[0].0 as i64).abs();
    let dy01 = (rect[1].1 as i64 - rect[0].1 as i64).abs();
    let dx03 = (rect[3].0 as i64 - rect[0].0 as i64).abs();
    let dy03 = (rect[3].1 as i64 - rect[0].1 as i64).abs();
    if dx01 == 0 || dy01 == 0 {
        // 軸に平行
        let e01 = dx01.max(dy01);
        let e03 = dx03.max(dy03);
        e01 * e01 * e03 * e03
    } else {
        // 45度傾いている
        let e01 = dx01;
        let e03 = dx03;
        e01 * e03 * 2
    }
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

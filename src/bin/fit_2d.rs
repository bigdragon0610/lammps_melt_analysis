use std::{
    env,
    f64::consts::PI,
    fs::File,
    io::{BufRead, BufReader, Result},
};

use rand::Rng;

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("ファイル名を指定してください");
    let f = File::open(file_name).expect("ファイルが開けませんでした");
    let reader = BufReader::new(f);
    let mut drops: Vec<Vec<f64>> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let line: Vec<f64> = line
            .split_whitespace()
            .map(|val| val.parse::<f64>().unwrap())
            .collect();
        drops.push(line);
    }

    let mut r = 100.0;
    let mut y0 = 0.0;

    let mut min_v = f64::MAX;

    let mut rng = rand::thread_rng();
    'outer: for i in 1..10000 {
        let alpha = 0.15;
        let step_size = 1.0 / (i as f64).powf(alpha);
        let theta = 2.0 * PI * rng.gen::<f64>();
        let tmp_r = r + step_size * theta.sin();
        let tmp_y0 = y0 + step_size * theta.cos();

        for drop in &drops {
            let (x, y) = (drop[0], drop[1]);
            if x * x + (y + tmp_y0) * (y + tmp_y0) > tmp_r * tmp_r {
                continue 'outer;
            }
        }

        let theta = (tmp_y0 / tmp_r).asin();
        let v = PI * tmp_r * tmp_r * ((PI - 2.0 * theta) / (2.0 * PI))
            - tmp_r * tmp_r * (PI - 2.0 * theta).sin() / 2.0;
        if min_v <= v {
            let p: f64 = rng.gen();
            let de = v - min_v;
            let beta = 0.1;
            if p > (-beta * de).exp() {
                continue;
            }
        }

        r = tmp_r;
        y0 = tmp_y0;
        min_v = v;
        println!("{} {} {} {}", i, tmp_r, tmp_y0, min_v);
    }

    Ok(())
}

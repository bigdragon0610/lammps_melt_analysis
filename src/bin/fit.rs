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
    let mut z0 = 0.0;

    let mut min_v = f64::MAX;
    let mut ans = (1, r, z0, min_v);

    let mut beta = 0.01;
    let beta_increase_rate = 0.0001;

    let mut rng = rand::thread_rng();
    'outer: for i in 1..100000 {
        let alpha = 0.15;
        let step_size = 1.0 / (i as f64).powf(alpha);
        let theta = 2.0 * PI * rng.gen::<f64>();
        let tmp_r = r + step_size * theta.sin();
        let tmp_z0 = z0 + step_size * theta.cos();

        for drop in &drops {
            let (x, y, z) = (drop[0], drop[1], drop[2]);
            if x * x + y * y + (z + tmp_z0) * (z + tmp_z0) > tmp_r * tmp_r {
                continue 'outer;
            }
        }

        let h = tmp_r - tmp_z0;
        let v = PI * h * (h * h + 3.0 * (tmp_r * tmp_r - tmp_z0 * tmp_z0)) / 6.0;
        if v < ans.3 {
            ans = (i, tmp_r, tmp_z0, v);
        }

        if min_v <= v {
            let p: f64 = rng.gen();
            let de = v - min_v;
            if p > (-beta * de).exp() {
                continue;
            }
        }

        r = tmp_r;
        z0 = tmp_z0;
        min_v = v;
        println!("{} {} {} {}", i, tmp_r, tmp_z0, min_v);
        beta += beta_increase_rate;
    }

    println!("# {:?}", ans);

    Ok(())
}

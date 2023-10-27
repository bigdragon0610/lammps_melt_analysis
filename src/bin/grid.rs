use std::{
    env,
    f64::consts::PI,
    fs::File,
    io::{BufRead, BufReader, Result},
};

const GRID: usize = 10;
const MAX_R: usize = 100;
const MAX_Z0: usize = 100;

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

    'outer: for i in 0..MAX_R * GRID {
        let tmp_r = (i / GRID) as f64;
        for j in 0..MAX_Z0 * GRID {
            let tmp_z0 = (j / GRID) as f64;
            for drop in &drops {
                let (x, y, z) = (drop[0], drop[1], drop[2]);
                if x * x + y * y + (z + tmp_z0) * (z + tmp_z0) > tmp_r * tmp_r {
                    continue 'outer;
                }
            }

            let h = tmp_r - tmp_z0;
            let v = PI * h * (h * h + 3.0 * (tmp_r * tmp_r - tmp_z0 * tmp_z0)) / 6.0;
            println!("{} {} {}", tmp_r, tmp_z0, v);
        }
    }

    Ok(())
}

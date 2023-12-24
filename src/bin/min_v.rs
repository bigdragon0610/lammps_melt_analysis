use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Result},
};

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("ファイル名を指定してください");
    let f = File::open(file_name).expect("ファイルが開けませんでした");
    let reader = BufReader::new(f);

    let mut min = (0, f64::MAX, f64::MAX, f64::MAX);

    for line in reader.lines() {
        let line: Vec<f64> = line?
            .split_whitespace()
            .map(|val| val.parse::<f64>().unwrap())
            .collect();

        if line[3] < min.3 {
            min = (line[0] as usize, line[1], line[2], line[3])
        }
    }

    println!("{} {} {} {}", min.0, min.1, min.2, min.3);

    Ok(())
}

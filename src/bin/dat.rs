use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
};

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("ファイル名を指定してください");
    let timestep = env::args()
        .nth(2)
        .expect("タイムステップを指定してください")
        .parse::<usize>()
        .unwrap();
    let f = File::open(file_name).expect("ファイルが開けませんでした");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    skip_timestep(timestep, &mut lines)?;
    let number_of_atoms = get_number_of_atoms(&mut lines)?;
    let box_size = get_box_size(&mut lines)?;

    lines.next(); // ITEM: ATOMS id type xs ys zs
    for _ in 0..number_of_atoms {
        let line = lines.next().unwrap()?;
        let line = line.split_whitespace().collect::<Vec<&str>>();
        let atom_type: usize = line[1].parse().unwrap();
        if atom_type != 2 {
            continue;
        }
        let x: f64 = line[2].parse().unwrap();
        let y: f64 = line[3].parse().unwrap();
        let z: f64 = line[4].parse().unwrap();
        println!(
            "{} {} {}",
            x * box_size[0],
            y * box_size[1],
            z * box_size[2]
        );
    }

    Ok(())
}

fn skip_timestep(timestep: usize, lines: &mut Lines<BufReader<File>>) -> Result<()> {
    let mut found_timestep = false;
    loop {
        let line = lines.next().unwrap()?;
        if found_timestep {
            if timestep == line.parse().unwrap() {
                break;
            }
            found_timestep = false;
        }
        if line == "ITEM: TIMESTEP" {
            found_timestep = true;
        }
    }

    Ok(())
}

fn get_number_of_atoms(lines: &mut Lines<BufReader<File>>) -> Result<usize> {
    lines.next(); // ITEM: NUMBER OF ATOMS

    let line = lines.next().unwrap()?;

    Ok(line.parse().unwrap())
}

fn get_box_size(lines: &mut Lines<BufReader<File>>) -> Result<[f64; 3]> {
    lines.next(); // ITEM: BOX BOUNDS pp pp pp

    let mut box_size = [0.0, 0.0, 0.0];
    for size in &mut box_size {
        let next_line = lines
            .next()
            .unwrap()?
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<f64>>();
        *size = next_line[1] - next_line[0];
    }

    Ok(box_size)
}

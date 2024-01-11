use std::{
    fs::File,
    io::{BufReader, Lines, Result},
};

pub fn skip_timestep(timestep: usize, lines: &mut Lines<BufReader<File>>) -> Result<()> {
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

pub fn get_number_of_atoms(lines: &mut Lines<BufReader<File>>) -> Result<usize> {
    lines.next(); // ITEM: NUMBER OF ATOMS

    let line = lines.next().unwrap()?;

    Ok(line.parse().unwrap())
}

pub fn get_box_size(lines: &mut Lines<BufReader<File>>) -> Result<[f64; 3]> {
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

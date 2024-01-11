use clap::Parser;
use lammps_melt_analysis::utility::{
    dat::{get_box_size, get_number_of_atoms, skip_timestep},
    drop::get_drop,
    fit::fit,
    min_v::get_min_v,
};
use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
};

#[derive(Parser)]
struct Args {
    /// Path to the LAMMPS data file
    #[arg(short, long)]
    file_path: String,

    /// The last timestep
    #[arg(short, long)]
    last_timestep: usize,

    /// The interval of timesteps
    #[arg(short, long)]
    interval: usize,

    #[arg(short, long)]
    cutoff: f64,

    #[arg(short, long)]
    start_r: f64,

    #[arg(short, long)]
    start_z0: f64,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let file_path = args.file_path;
    let last_timestep = args.last_timestep;
    let interval = args.interval;
    let cutoff = args.cutoff;
    let start_r = args.start_r;
    let start_z0 = args.start_z0;

    let f = File::open(file_path).expect("Could not open the file");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    for timestep in (0..=last_timestep).step_by(interval) {
        skip_timestep(timestep, &mut lines)?;
        let number_of_atoms = get_number_of_atoms(&mut lines)?;
        let box_size = get_box_size(&mut lines)?;

        lines.next(); // ITEM: ATOMS id type xs ys zs
        let mut atoms = Vec::new();
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
            atoms.push([x * box_size[0], y * box_size[1], z * box_size[2]]);
        }

        let droplet = get_drop(cutoff, atoms);

        let fitting_data = fit(start_r, start_z0, droplet);

        let min_v = get_min_v(fitting_data);

        println!(
            "{} {} {} {} {}",
            timestep, min_v.0, min_v.1, min_v.2, min_v.3
        );
    }

    Ok(())
}

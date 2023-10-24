use std::io::Result;

use rand::Rng;

const R: f64 = 50.0;
const Z0: f64 = 10.0;

fn main() -> Result<()> {
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let x = R * rng.gen_range(-1.0..=1.0);
        let y = R * rng.gen_range(-1.0..=1.0);
        let z = R * rng.gen_range(0.0..=1.0);

        if x * x + y * y + z * z <= R * R && z >= Z0 {
            println!("{} {} {}", x, y, z - Z0);
        }
    }
    Ok(())
}

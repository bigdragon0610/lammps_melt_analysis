use rand::Rng;

const R: f64 = 50.0;
const Y0: f64 = 10.0;
// v: 2900程度

fn main() {
    let mut rng = rand::thread_rng();
    for _ in 0..10000 {
        let x = R * rng.gen_range(-1.0..=1.0);
        let y = R * rng.gen_range(0.0..=1.0);

        if x * x + y * y <= R * R && y >= Y0 {
            println!("{} {}", x, y - Y0);
        }
    }
}

use std::f64::consts::PI;

use rand::Rng;

pub fn fit(mut r: f64, mut z0: f64, drops: Vec<[f64; 3]>) -> Vec<[f64; 4]> {
    let mut min_v = f64::MAX;

    let mut beta = 0.01;
    let beta_increase_rate = 0.0001;

    let mut rng = rand::thread_rng();
    let mut fitting_data = Vec::new();
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
        fitting_data.push([i as f64, tmp_r, tmp_z0, min_v]);
        beta += beta_increase_rate;
    }

    fitting_data
}

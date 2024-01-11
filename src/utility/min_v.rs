pub fn get_min_v(fitting_data: Vec<[f64; 4]>) -> (usize, f64, f64, f64) {
    let mut min = (0, f64::MAX, f64::MAX, f64::MAX);

    for line in fitting_data {
        if line[3] < min.3 {
            min = (line[0] as usize, line[1], line[2], line[3])
        }
    }

    min
}

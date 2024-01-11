use std::collections::HashMap;

pub fn get_drop(cutoff: f64, list: Vec<[f64; 3]>) -> Vec<[f64; 3]> {
    let mut cluster: Vec<usize> = (0..list.len()).collect();
    for i in 0..list.len() {
        for j in i + 1..list.len() {
            let dx = list[i][0] - list[j][0];
            let dy = list[i][1] - list[j][1];
            let dz = list[i][2] - list[j][2];
            let r2 = dx * dx + dy * dy + dz * dz;
            if r2 <= cutoff * cutoff {
                union(i, j, &mut cluster);
            }
        }
    }

    let cluster = (0..list.len())
        .map(|i| find(i, &cluster))
        .collect::<Vec<usize>>();

    let mut occurrences: HashMap<usize, usize> = HashMap::new();
    for parent in &cluster {
        occurrences
            .entry(*parent)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let max_index = *occurrences.iter().max_by_key(|x| x.1).unwrap().0;

    let droplet = cluster
        .iter()
        .zip(list.iter())
        .filter(|x| *x.0 == max_index)
        .map(|x| x.1)
        .cloned()
        .collect::<Vec<_>>();

    let (x_sum, y_sum) = droplet
        .iter()
        .fold((0.0, 0.0), |acc, r| (acc.0 + r[0], acc.1 + r[1]));
    let z_min = droplet
        .iter()
        .min_by(|a, b| a[2].partial_cmp(&b[2]).unwrap())
        .unwrap()[2];
    let center = (
        x_sum / droplet.len() as f64,
        y_sum / droplet.len() as f64,
        z_min,
    );

    let droplet = droplet
        .iter()
        .map(|r| [r[0] - center.0, r[1] - center.1, r[2] - center.2])
        .collect::<Vec<[f64; 3]>>();

    droplet
}

fn union(i: usize, j: usize, cluster: &mut Vec<usize>) {
    let i = find(i, cluster);
    let j = find(j, cluster);
    if i < j {
        cluster[j] = i
    } else {
        cluster[i] = j
    }
}

fn find(index: usize, cluster: &Vec<usize>) -> usize {
    if index == cluster[index] {
        index
    } else {
        find(cluster[index], cluster)
    }
}

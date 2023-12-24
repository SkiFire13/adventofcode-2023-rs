#[allow(unused_imports)]
use super::prelude::*;
type Input = Vec<(i128, i128, i128, i128, i128, i128)>;

pub fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(" @ ").unwrap();
            let (x, yz) = p.split_once(", ").unwrap();
            let (y, z) = yz.split_once(", ").unwrap();
            let (vx, vyz) = v.split_once(", ").unwrap();
            let (vy, vz) = vyz.split_once(", ").unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            let z = z.parse().unwrap();
            let vx = vx.parse().unwrap();
            let vy = vy.parse().unwrap();
            let vz = vz.parse().unwrap();
            (x, y, z, vx, vy, vz)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut count = 0;

    for (&p1, &p2) in input.iter().tuple_combinations() {
        let (x1, y1, _, vx1, vy1, _) = p1;
        let (x2, y2, _, vx2, vy2, _) = p2;

        let dx = x2 - x1;
        let dy = y2 - y1;

        let (t2_up, t2_down) = (vy1 * dx - dy * vx1, vy2 * vx1 - vy1 * vx2);
        let (t1_up, t1_down) = (dy * vx2 - vy2 * dx, vy1 * vx2 - vy2 * vx1);

        if t2_up.signum() != t2_down.signum() || t1_up.signum() != t1_down.signum() {
            continue;
        }

        const MIN: i128 = 200000000000000;
        const MAX: i128 = 400000000000000;

        let (t2_up, t2_down) = (t2_up.abs() as i128, t2_down.abs() as i128);
        let (x2, y2, vx2, vy2) = (x2 as i128, y2 as i128, vx2 as i128, vy2 as i128);

        if ((MIN - x2) * t2_down <= t2_up * vx2 && t2_up * vx2 <= (MAX - x2) * t2_down)
            && ((MIN - y2) * t2_down <= t2_up * vy2 && t2_up * vy2 <= (MAX - y2) * t2_down)
        {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &Input) -> u64 {
    let (x1, y1, z1, vx1, vy1, vz1) = input[0];
    let (x2, y2, z2, vx2, vy2, vz2) = input[1];
    let (x3, y3, z3, vx3, vy3, vz3) = input[2];

    let [x1, y1, z1, vx1, vy1, vz1] = [x1, y1, z1, vx1, vy1, vz1].map(|c| c as f64);
    let [x2, y2, z2, vx2, vy2, vz2] = [x2, y2, z2, vx2, vy2, vz2].map(|c| c as f64);
    let [x3, y3, z3, vx3, vy3, vz3] = [x3, y3, z3, vx3, vy3, vz3].map(|c| c as f64);

    #[rustfmt::skip]
    let mut coeffs = [
        [0., -vz1 + vz2, vy1 - vy2, 0., z1 - z2, -y1 + y2, vy1 * z1 - vy2 * z2 - vz1 * y1 + vz2 * y2],
        [vz1 - vz2, 0., -vx1 + vx2, -z1 + z2, 0., x1 - x2, -vx1 * z1 + vx2 * z2 + vz1 * x1 - vz2 * x2 ],
        [-vy1 + vy2, vx1 - vx2, 0., y1 - y2, -x1 + x2, 0., vx1 * y1 - vx2 * y2 - vy1 * x1 + vy2 * x2],
        [0., -vz2 + vz3, vy2 - vy3, 0., z2 - z3, -y2 + y3, vy2 * z2 - vy3 * z3 - vz2 * y2 + vz3 * y3],
        [vz2 - vz3, 0., -vx2 + vx3, -z2 + z3, 0., x2 - x3, -vx2 * z2 + vx3 * z3 + vz2 * x2 - vz3 * x3 ],
        [-vy2 + vy3, vx2 - vx3, 0., y2 - y3, -x2 + x3, 0., vx2 * y2 - vx3 * y3 - vy2 * x2 + vy3 * x3],
    ];

    for i in 0..6 {
        let j = (i..6).max_by(|&j, &k| f64::total_cmp(&coeffs[j][i].abs(), &coeffs[k][i].abs()));
        coeffs.swap(i, j.unwrap());
        (i..7).rev().for_each(|j| coeffs[i][j] /= coeffs[i][i]);
        for j in i + 1..6 {
            for k in (i..7).rev() {
                coeffs[j][k] -= coeffs[i][k] * coeffs[j][i];
            }
        }
    }

    for i in (1..6).rev() {
        for j in 0..i {
            coeffs[j][6] -= coeffs[j][i] * coeffs[i][6];
            coeffs[j][i] = 0.;
        }
    }

    (coeffs[0][6] + coeffs[1][6] + coeffs[2][6]) as u64
}

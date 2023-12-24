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

        let t2_up = vy1 * dx - dy * vx1;
        let t2_down = vy2 * vx1 - vy1 * vx2;

        let t1_up = dy * vx2 - vy2 * dx;
        let t1_down = vy1 * vx2 - vy2 * vx1;

        if t2_down == 0 || t2_up.signum() * t2_down.signum() == -1 {
            continue;
        }
        if t1_down == 0 || t1_up.signum() * t1_down.signum() == -1 {
            continue;
        }

        let (t2_up, t2_down) = (t2_up.abs(), t2_down.abs());

        const MIN: i128 = 200000000000000;
        const MAX: i128 = 400000000000000;

        if ((MIN - x2) * t2_down <= t2_up * vx2 && t2_up * vx2 <= (MAX - x2) * t2_down)
            && ((MIN - y2) * t2_down <= t2_up * vy2 && t2_up * vy2 <= (MAX - y2) * t2_down)
        {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &Input) -> usize {
    use std::fmt::Write as _;
    let mut s = String::new();
    _ = writeln!(s, "from z3 import *");
    _ = writeln!(s, "x = Real('x')");
    _ = writeln!(s, "y = Real('y')");
    _ = writeln!(s, "z = Real('z')");
    _ = writeln!(s, "vx = Real('vx')");
    _ = writeln!(s, "vy = Real('vy')");
    _ = writeln!(s, "vz = Real('vz')");
    _ = writeln!(s, "s = Solver()");
    for i in 0..input.len() {
        let (x, y, z, vx, vy, vz) = input[i];
        let i = i + 1;
        _ = writeln!(s, "t{i} = Real('t{i}')");
        _ = writeln!(s, "s.add({x} + {vx} * t{i} == x + vx * t{i})");
        _ = writeln!(s, "s.add({y} + {vy} * t{i} == y + vy * t{i})");
        _ = writeln!(s, "s.add({z} + {vz} * t{i} == z + vz * t{i})");
    }
    _ = writeln!(s, "s.check()");
    _ = writeln!(s, "print(sum(int(str(s.model()[v])) for v in [x, y, z]))");

    let mut p = std::process::Command::new("python3")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = p.stdin.take().unwrap();
    use std::io::Write as _;
    stdin.write_all(s.as_bytes()).unwrap();
    stdin.flush().unwrap();
    drop(stdin);
    String::from_utf8(p.wait_with_output().unwrap().stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}

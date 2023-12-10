#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |char, _, _| char as u8)
}

pub fn part1(input: &Input) -> usize {
    let ((x, y), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();
    let pos = (x as isize, y as isize);

    let mut poss = [(pos, (-1, 0)), (pos, (0, 1))];
    let mut steps = 0;

    while poss[0].0 != poss[1].0 || steps == 0 {
        poss = poss.map(|((x, y), (dx, dy))| {
            let (x, y) = (x + dx, y + dy);
            let (dx, dy) = match input[(x, y)] {
                b'-' | b'|' => (dx, dy),
                b'L' | b'7' => (dy, dx),
                b'F' | b'J' => (-dy, -dx),
                _ => unreachable!("{:?} {}", (x, y), input[(x, y)] as char),
            };
            ((x, y), (dx, dy))
        });
        steps += 1;
    }

    steps
}

pub fn part2(input: &Input) -> usize {
    let ((x, y), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();
    let pos = (x as isize, y as isize);

    let mut poss = [(pos, (-1, 0)), (pos, (0, 1))];
    let mut first = true;

    let mut seen = HashSet::new();
    seen.insert(pos);

    while poss[0].0 != poss[1].0 || first {
        first = false;
        poss = poss.map(|((x, y), (dx, dy))| {
            let (x, y) = (x + dx, y + dy);
            let (dx, dy) = match input[(x, y)] {
                b'-' | b'|' => (dx, dy),
                b'L' | b'7' => (dy, dx),
                b'F' | b'J' => (-dy, -dx),
                _ => unreachable!("{:?} {}", (x, y), input[(x, y)] as char),
            };
            seen.insert((x, y));
            ((x, y), (dx, dy))
        });
    }

    let mut inside = 0;
    for y in 0..input.h() {
        let mut crossed = 0;
        let mut crossing_dy = 0;
        for x in 0..input.w() {
            if seen.contains(&(x as isize, y as isize)) {
                match input[(x, y)] {
                    b'|' => {
                        crossed += 1;
                        crossing_dy = 0;
                    }
                    b'-' => {}
                    b'F' => crossing_dy = 1,
                    b'L' => crossing_dy = -1,
                    b'7' | b'S' => {
                        if crossing_dy == -1 {
                            crossed += 1;
                        }
                        crossing_dy = 0;
                    }
                    b'J' => {
                        if crossing_dy == 1 {
                            crossed += 1;
                        }
                        crossing_dy = 0;
                    }
                    _ => unreachable!(),
                }
            } else {
                crossing_dy = 0;
                if crossed % 2 == 1 {
                    inside += 1;
                }
            }
        }
    }
    inside
}

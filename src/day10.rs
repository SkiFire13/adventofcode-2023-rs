#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |char, _, _| char as u8)
}

fn visit_loop(input: &Input, mut f: impl FnMut(usize, usize)) -> u8 {
    let ((x, y), _) = input.iter().find(|&(_, &c)| c == b'S').unwrap();
    let (mut x, mut y) = (x as isize, y as isize);

    let left = matches!(input.iget((x - 1, y)), Some(b'-' | b'L' | b'F'));
    let right = matches!(input.iget((x + 1, y)), Some(b'-' | b'7' | b'J'));
    let top = matches!(input.iget((x, y - 1)), Some(b'|' | b'7' | b'F'));
    let bottom = matches!(input.iget((x, y + 1)), Some(b'|' | b'L' | b'J'));

    let (mut dx, mut dy) = match () {
        _ if left => (-1, 0),
        _ if right => (1, 0),
        _ if top => (0, -1),
        _ if bottom => (0, 1),
        _ => panic!("Invalid input"),
    };

    loop {
        f(x as usize, y as usize);
        (x, y) = (x + dx, y + dy);
        (dx, dy) = match input[(x, y)] {
            b'-' | b'|' => (dx, dy),
            b'L' | b'7' => (dy, dx),
            b'F' | b'J' => (-dy, -dx),
            b'S' => break,
            _ => panic!("Invalid input"),
        }
    }

    match (left, right, top, bottom) {
        (true, true, _, _) => b'-',
        (true, _, true, _) => b'J',
        (true, _, _, true) => b'7',
        (_, true, true, _) => b'L',
        (_, true, _, true) => b'F',
        (_, _, true, true) => b'|',
        _ => panic!("Invalid input"),
    }
}

pub fn part1(input: &Input) -> usize {
    let mut steps = 0;
    visit_loop(input, |_, _| steps += 1);
    steps / 2
}

pub fn part2(input: &Input) -> usize {
    let mut seen = Grid::with_dimensions(input.w(), input.h()).into_set();
    let start_symb = visit_loop(input, |x, y| seen[(x, y)] = true);

    let mut count = 0;
    for y in 0..input.h() {
        let mut inside = false;
        for x in 0..input.w() {
            if seen.contains((x, y)) {
                let symb = input[(x, y)];
                let symb = if symb == b'S' { start_symb } else { symb };
                inside = inside ^ matches!(symb, b'|' | b'L' | b'J');
            } else if inside {
                count += 1;
            }
        }
    }
    count
}

#[allow(unused_imports)]
use super::prelude::*;
type Input = (Box<[u128]>, Box<[u128]>);

pub fn input_generator(input: &str) -> Input {
    let (mut rocks, mut walls) = (vec![0], vec![!0]);
    let (mut rock_layer, mut wall_layer) = (0, 1);

    for b in input.bytes() {
        (rock_layer, wall_layer) = (rock_layer << 1, wall_layer << 1);
        match b {
            b'\n' => {
                rocks.push(rock_layer);
                walls.push(wall_layer | 1);
                (rock_layer, wall_layer) = (0, 1);
            }
            b'O' => rock_layer |= 1,
            b'#' => wall_layer |= 1,
            b'.' => {}
            _ => panic!(),
        }
    }

    rocks.push(0);
    walls.push(!0);

    (rocks.into_boxed_slice(), walls.into_boxed_slice())
}

fn move_north(rocks: &mut [u128], walls: &[u128]) {
    for i in 1..rocks.len() - 1 {
        let mut curr_rockl = take(&mut rocks[i]);
        let mut j = i;
        while curr_rockl != 0 {
            let stuck_rockl = curr_rockl & (walls[j - 1] | rocks[j - 1]);
            rocks[j] |= stuck_rockl;
            curr_rockl ^= stuck_rockl;
            j -= 1;
        }
    }
}

fn move_south(rocks: &mut [u128], walls: &[u128]) {
    for i in (1..rocks.len() - 1).rev() {
        let mut curr_rockl = take(&mut rocks[i]);
        let mut j = i;
        while curr_rockl != 0 {
            let stuck_rockl = curr_rockl & (walls[j + 1] | rocks[j + 1]);
            rocks[j] |= stuck_rockl;
            curr_rockl ^= stuck_rockl;
            j += 1;
        }
    }
}

fn move_west(rocks: &mut [u128], walls: &[u128]) {
    for i in 1..rocks.len() - 1 {
        let (mut curr_rockl, mut new_rockl) = (rocks[i], 0);
        let mut curr_stuck = walls[i];
        while curr_rockl != 0 {
            let mut new_stuck = curr_rockl & (curr_stuck >> 1);
            while new_stuck != 0 {
                curr_stuck |= new_stuck;
                new_stuck = (curr_rockl & (curr_stuck >> 1)) & !curr_stuck;
            }
            new_rockl |= curr_stuck & !walls[i];
            curr_rockl = (curr_rockl & !new_rockl) << 1;
        }
        rocks[i] = new_rockl;
    }
}

fn move_east(rocks: &mut [u128], walls: &[u128]) {
    for i in 1..rocks.len() - 1 {
        let (mut curr_rockl, mut new_rockl) = (rocks[i], 0);
        let mut curr_stuck = walls[i];
        while curr_rockl != 0 {
            let mut new_stuck = curr_rockl & (curr_stuck << 1);
            while new_stuck != 0 {
                curr_stuck |= new_stuck;
                new_stuck = (curr_rockl & (curr_stuck << 1)) & !curr_stuck;
            }
            new_rockl |= curr_stuck & !walls[i];
            curr_rockl = (curr_rockl & !new_rockl) >> 1;
        }
        rocks[i] = new_rockl;
    }
}

fn load_of(rocks: &[u128]) -> usize {
    rocks
        .iter()
        .enumerate()
        .map(|(i, rockl)| rockl.count_ones() as usize * (rocks.len() - i - 1))
        .sum()
}

pub fn part1(input: &Input) -> usize {
    let (rocks, walls) = input;
    let mut rocks = rocks.clone();
    move_north(&mut rocks, walls);
    load_of(&rocks)
}

pub fn part2(input: &Input) -> usize {
    let (rocks, walls) = input;
    let mut rocks = rocks.clone();

    let mut seen = FxIndexMap::default();
    let mut curr = 0;
    while curr < 1000000000 {
        seen.insert(rocks.clone(), curr);

        move_north(&mut rocks, walls);
        move_west(&mut rocks, walls);
        move_south(&mut rocks, walls);
        move_east(&mut rocks, walls);

        curr += 1;

        if let Some(&old) = seen.get(&rocks) {
            let cycle_len = curr - old;
            let cycle_offset = old;
            let answer = (1000000000 - cycle_offset) % cycle_len + cycle_offset;

            rocks = seen
                .drain(..)
                .find(|&(_, i)| i == answer)
                .map(|(rocks, _)| rocks)
                .unwrap();
            break;
        }
    }

    load_of(&rocks)
}

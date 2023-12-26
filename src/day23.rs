#[allow(unused_imports)]
use super::prelude::*;
type Input = Grid<u8>;

pub fn input_generator(input: &str) -> Input {
    Grid::from_input_chars(input, |c, _, _| c as u8)
}

fn simplify_graph<const PART2: bool>(input: &Input) -> Vec<ArrayVec<(usize, usize), 4>> {
    let start = (0..input.w()).find(|&x| input[(x, 0)] == b'.');
    let end = (0..input.w()).find(|&x| input[(x, input.h() - 1)] == b'.');
    let (start, end) = (start.unwrap(), end.unwrap());

    let mut node_poss = vec![(start, 0), (end, input.h() - 1)];
    let mut node_edges = vec![ArrayVec::new(), ArrayVec::new()];
    let mut pos_to_node = FxHashMap::from_iter([(node_poss[0], 0), (node_poss[1], 1)]);
    let mut explored = FxHashSet::default();
    let mut to_visit = vec![0];

    explored.reserve(128);
    pos_to_node.reserve(64);

    while let Some(node) = to_visit.pop() {
        for (mut x, mut y) in input.plus_neighbours(node_poss[node]) {
            if input[(x, y)] == b'#' || !explored.insert((x, y)) {
                continue;
            }

            let mut d = 1;
            let (mut px, mut py) = node_poss[node];
            let (mut forward, mut backward) = (true, true);

            loop {
                let (dx, dy) = (x as isize - px as isize, y as isize - py as isize);
                match (if PART2 { b'.' } else { input[(x, y)] }, dx, dy) {
                    (b'<', 1, 0) | (b'>', -1, 0) | (b'^', 0, 1) | (b'v', 0, -1) => forward = false,
                    (b'<', -1, 0) | (b'>', 1, 0) | (b'^', 0, -1) | (b'v', 0, 1) => backward = false,
                    _ => {}
                }

                if (x, y) == (end, input.h() - 1) {
                    node_edges[node].push((1, d));
                    node_edges[1].push((node, d));
                    break;
                }

                let mut iter = input
                    .plus_neighbours((x, y))
                    .filter(|&(nx, ny)| input[(nx, ny)] != b'#')
                    .filter(|&(nx, ny)| (nx, ny) != (px, py));

                let Some((nx, ny)) = iter.next() else { break };

                if iter.next().is_some() {
                    let end_node = *pos_to_node.entry((x, y)).or_insert_with(|| {
                        let new_node = node_poss.len();
                        node_poss.push((x, y));
                        node_edges.push(ArrayVec::new());
                        to_visit.push(new_node);
                        new_node
                    });
                    explored.insert((px, py));
                    forward.then(|| node_edges[node].push((end_node, d)));
                    backward.then(|| node_edges[end_node].push((node, d)));

                    break;
                }

                (px, py, x, y, d) = (x, y, nx, ny, d + 1);
            }
        }
    }

    node_edges
}

pub fn part1(input: &Input) -> usize {
    fn solve(edges: &[ArrayVec<(usize, usize), 4>], seen: u64, node: usize, d: usize) -> usize {
        edges[node]
            .iter()
            .filter(|&&(next, _)| seen & (1 << next) == 0)
            .map(|&(next, cost)| solve(edges, seen | (1 << next), next, d + cost))
            .max()
            .unwrap_or(if node == 1 { d } else { 0 })
    }
    solve(&simplify_graph::<false>(input), 0, 0, 0)
}

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    from: u64,
    to: u64,
}

const MASK: u64 = 0x0101010101010101;
impl State {
    fn index_of(&self, b: u8) -> usize {
        let from = self.from ^ b as u64 * MASK;
        let from = from.wrapping_sub(MASK) & !from;
        let to = self.to ^ b as u64 * MASK;
        let to = to.wrapping_sub(MASK) & !to;
        ((from | to) & (MASK * 0x80)).trailing_zeros() as usize / 8
    }
    fn insert(&mut self, from: u8, to: u8) {
        let masked = self.from & (MASK * 0x7F);
        let pos = (!(masked - MASK * from as u64) & (MASK * 0x80)).trailing_zeros() / 8;
        let m1 = (1 << pos * 8) - 1;
        self.from = (self.from & m1) | (self.from & !m1) << 8 | ((from as u64) << pos * 8);
        self.to = (self.to & m1) | (self.to & !m1) << 8 | ((to as u64) << pos * 8);
    }
    fn remove(&mut self, pos: usize) {
        let m1 = (1 << pos * 8) - 1;
        let m2 = !((1 << (pos + 1) * 8) - 1);
        self.from = (self.from & m1) | (self.from & m2) >> 8 | (0xFF << 56);
        self.to = (self.to & m1) | (self.to & m2) >> 8 | (0xFF << 56);
    }
    fn get(&self, pos: usize) -> (u8, u8) {
        let from = (self.from >> pos * 8) & 0xFF;
        let to = (self.to >> pos * 8) & 0xFF;
        (from as u8, to as u8)
    }
}

pub fn part2(input: &Input) -> usize {
    let mut graph = simplify_graph::<true>(input);
    let counts = graph.iter().map(|edges| edges.len()).collect::<Vec<_>>();
    for edges in &mut graph {
        edges.sort_unstable_by_key(|&(n, _)| counts[n]);
    }

    let mut states = FxHashMap::default();
    let mut new_states = FxHashMap::default();
    let mut to_add = Vec::new();

    let mut queue = VecDeque::new();
    let mut unexplored_edges = graph.iter().map(|edges| edges.len()).collect::<Vec<_>>();
    let mut added = vec![false; graph.len()];
    let mut enqueued = vec![false; graph.len()];

    enqueued[0] = true;
    added[0] = true;
    for &(next, cost) in &graph[0] {
        enqueued[next] = true;
        added[next] = true;
        unexplored_edges[next] -= 1;
        states.insert(State { from: !0 << 8 | 0, to: !0 << 8 | next as u64 }, cost);
        for &(nextnext, _) in &graph[next] {
            if nextnext != 0 {
                enqueued[nextnext] = true;
                queue.push_back(nextnext);
            }
        }
    }
    unexplored_edges[1] += 1;

    while let Some(curr) = queue.pop_front() {
        added[curr] = true;

        new_states.clear();
        for (mut state, cost) in states.drain() {
            state.insert(curr as u8, curr as u8);
            new_states.insert(state, cost);
        }
        swap(&mut states, &mut new_states);

        for &(next, cost) in &graph[curr] {
            if added[next] {
                unexplored_edges[curr] -= 1;
                unexplored_edges[next] -= 1;

                for (&(mut state), &w) in &states {
                    let (edge1, edge2) = (state.index_of(curr as u8), state.index_of(next as u8));
                    if edge1 < 8 && edge2 < 8 && edge1 != edge2 {
                        let ((n1, m1), (n2, m2)) = (state.get(edge1), state.get(edge2));
                        let (other1, other2) = (n1 ^ m1 ^ (curr as u8), n2 ^ m2 ^ (next as u8));

                        state.remove(max(edge1, edge2));
                        state.remove(min(edge1, edge2));
                        state.insert(min(other1, other2), max(other1, other2));

                        to_add.push((state, w + cost));
                    }
                }
                for (state, cost) in to_add.drain(..) {
                    let best = states.entry(state).or_default();
                    *best = max(*best, cost);
                }

                for node in [curr, next] {
                    if unexplored_edges[node] == 0 {
                        let node = node as u8;
                        new_states.clear();
                        for (mut state, cost) in states.drain() {
                            if let idx @ ..=7 = state.index_of(node) {
                                if state.get(idx) != (node, node) {
                                    continue;
                                }
                                state.remove(idx);
                            }
                            let best = new_states.entry(state).or_default();
                            *best = max(*best, cost);
                        }
                        swap(&mut states, &mut new_states);
                    }
                }
            } else if !enqueued[next] {
                enqueued[next] = true;
                queue.push_back(next);
            }
        }
    }

    states[&State { from: !0 << 8 | 0, to: !0 << 8 | 1 }]
}

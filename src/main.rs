use std::collections::BTreeSet;

use defaultmap::DefaultHashMap;
use pathfinding::prelude::bfs;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Item {
    Wall,
    Air,
    Door(char),
    Key(char),
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct State {
    pos: (usize, usize),
    all_keys: BTreeSet<char>,
    unlocked: BTreeSet<char>,
}

fn main() {
    let input = include_str!("input.txt").trim();

    let mut grid = DefaultHashMap::new(Item::Wall);
    let mut pos = (0, 0);

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.trim().chars().enumerate() {
            grid[(i, j)] = match c {
                '#' => Item::Wall,
                '.' | '@' => Item::Air,
                'A'..='Z' => Item::Door(c.to_ascii_lowercase()),
                'a'..='z' => Item::Key(c.to_ascii_lowercase()),
                _ => unreachable!(),
            };
            if c == '@' {
                pos = (i, j);
            }
        }
    }

    dbg!(&input);

    let initial_state = State {
        pos,
        all_keys: input.chars().filter(|x| ('a'..='z').contains(x)).collect(),
        unlocked: Default::default(),
    };

    let path = bfs(
        &initial_state,
        |state| {
            let (i, j) = state.pos;
            let mut result = vec![];
            for next_pos in [(i + 1, j), (i, j + 1), (i - 1, j), (i, j - 1)] {
                match grid[next_pos] {
                    Item::Air => result.push(State {
                        pos: next_pos,
                        ..state.clone()
                    }),
                    Item::Key(c) => {
                        let mut next_state = state.clone();
                        next_state.pos = next_pos;
                        next_state.unlocked.insert(c);
                        result.push(next_state);
                    }
                    Item::Door(c) => {
                        if state.unlocked.contains(&c) {
                            result.push(State {
                                pos: next_pos,
                                ..state.clone()
                            })
                        }
                    }
                    _ => {}
                }
            }
            result
        },
        |state| state.unlocked.len() == state.all_keys.len(),
    );

    dbg!(&path.unwrap().len() - 1);
}

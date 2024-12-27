use arrayvec::ArrayVec;
use itertools::iproduct;
use crate::sudoku_test::board::Possibilities;
use crate::sudoku_test::solution::Solution;

pub fn solve_backtracking(mut solution: Solution) -> Option<Solution> {
    // Dumb thing due to possible invalid inputs
    if !solution.is_valid() {
        return None;
    }

    let possibilities = Possibilities::from_solution(&solution);

    recursively_attempt(possibilities, &mut solution);
    if solution.solved() {
        Some(solution)
    } else {
        None
    }
}

pub fn recursively_attempt(mut possibilities: Possibilities, solution: &mut Solution) {
    if solution.solved() {
        return;
    }

    // let mut to_revert: Vec<(u8, u8)> = Vec::with_capacity(16);
    let mut to_revert: ArrayVec<(u8, u8), 128> = ArrayVec::new();

    let mut change = true;
    let mut first_pass = true;

    // 0-8 Number appeared
    // 9 - Hasn't appeared | -1
    // 10 - Has appeared more than once | -2
    let mut singles_data = [9u8; 9];

    let mut lowest_pos = None;

    while change {
        if solution.solved() {
            return;
        }

        change = false;
        let mut lowest = 10;

        for y in 0..9 {
            if first_pass {
                singles_data = [9; 9];
            }

            for x in 0..9 {
                let sg = solution.get(x, y);
                if sg != 9 {
                    if first_pass {
                        singles_data[sg as usize] = 10;
                    }
                    continue;
                }

                let cell_possibilities = possibilities.get(x, y);
                let count = cell_possibilities.bits_set();

                if count == 0 {
                    // println!("A");
                    solution.undo(&to_revert);
                    return;
                }

                if count == 1 {
                    let val = cell_possibilities.find_single_bit();
                    to_revert.push((x as u8, y as u8));
                    solution.set(x, y, val);
                    possibilities.update_found(x, y, val);
                    if first_pass {
                        singles_data[val as usize] = 10;
                    }

                    change = true;
                    continue;
                }

                if count < lowest {
                    lowest = count;
                    lowest_pos = Some((x, y));
                }

                if first_pass {
                    for n in 0..9 {
                        if cell_possibilities.has(n) {
                            if singles_data[n as usize] == 9 {
                                singles_data[n as usize] = x as u8;
                            } else {
                                singles_data[n as usize] = 10;
                            }
                        }
                    }
                }
            }

            if first_pass {
                for (n, x) in singles_data.iter().enumerate() {
                    if *x <= 8 {
                        if solution.get(*x as usize, y) != 9 {
                            solution.undo(&to_revert);
                            return;
                        }

                        solution.set(*x as usize, y, n as u8);
                        change = true;
                        to_revert.push((*x, y as u8));
                        possibilities.update_found(*x as usize, y, n as u8);
                    }
                }
            }
        }

        if !first_pass {
            if change {
                continue;
            }
            break;
        }
        first_pass = false;

        for x in 0..9 {
            singles_data = [9; 9];

            for y in 0..9 {
                let sg = solution.get(x, y);
                if sg != 9 {
                    singles_data[sg as usize] = 10;
                    continue;
                }

                let cell_possibilities = possibilities.get(x, y);

                for n in 0..9 {
                    if cell_possibilities.has(n) {
                        if singles_data[n as usize] == 9 {
                            singles_data[n as usize] = y as u8;
                        } else {
                            singles_data[n as usize] = 10;
                        }
                    }
                }
            }

            for (n, y) in singles_data.iter().enumerate() {
                if *y <= 8 {
                    if solution.get(x, *y as usize) != 9 {
                        solution.undo(&to_revert);
                        return;
                    }

                    solution.set(x, *y as usize, n as u8);
                    change = true;
                    to_revert.push((x as u8, *y));
                    possibilities.update_found(x, *y as usize, n as u8);
                }
            }
        }

        for (sy, sx) in iproduct!(0..3, 0..3) {
            singles_data = [9; 9];

            for (cy, cx) in iproduct!(0..3, 0..3) {
                let (x, y) = (sx * 3 + cx, sy * 3 + cy);

                let sg = solution.get(x, y);
                if sg != 9 {
                    singles_data[sg as usize] = 10;
                    continue;
                }

                let cell_possibilities = possibilities.get(x, y);

                for n in 0..9 {
                    if cell_possibilities.has(n) {
                        if singles_data[n as usize] == 9 {
                            singles_data[n as usize] = cy as u8 * 3 + cx as u8;
                        } else {
                            singles_data[n as usize] = 10;
                        }
                    }
                }
            }

            for (n, sc) in singles_data.iter().enumerate() {
                if *sc <= 8 {
                    let (x, y) = (
                        (sx as u8 * 3 + (sc % 3)) as usize,
                        (sy as u8 * 3 + (*sc / 3)) as usize,
                    );

                    if solution.get(x, y) != 9 {
                        solution.undo(&to_revert);
                        return;
                    }

                    solution.set(x, y, n as u8);
                    change = true;
                    to_revert.push((x as u8, y as u8));
                    possibilities.update_found(x, y, n as u8);
                }
            }
        }
    }

    if solution.solved() {
        return;
    }

    let (x, y) = lowest_pos.unwrap();
    // println!("{x} {y}");
    // return;

    to_revert.push((x as u8, y as u8));
    let cell_possibilites = possibilities.get(x, y);
    for n in 0..9 {
        if !cell_possibilites.has(n) {
            continue;
        }

        solution.set(x, y, n);

        let mut new_possibilites = possibilities.clone();
        new_possibilites.update_found(x, y, n);

        recursively_attempt(new_possibilites, solution);

        if solution.solved() {
            return;
        }
    }

    solution.undo(&to_revert);
}

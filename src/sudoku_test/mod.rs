use solution::Solution;

pub mod board;
pub mod solution;
pub mod solver;

// fn load_puzzles(name: &str) -> Vec<(Solution, Option<Solution>)> {
//     let mut puzzles = Vec::new();
//     let mut i: usize = 0;
//     loop {
//         let Ok(ps) = fs::read_to_string(format!("converted/{name}_{i}_puzzle.txt")) else { break; };
//         let Ok(ss) = fs::read_to_string(format!("converted/{name}_{i}_solution.txt")) else { break; };
//
//         if ss.chars().next().unwrap() != '!' {
//             puzzles.push(
//                 (Solution::load_string(ps), Some(Solution::load_string(ss)))
//             );
//         }
//         else {
//             puzzles.push(
//                 (Solution::load_string(ps), None)
//             );
//         }
//
//
//         i += 1;
//     }
//
//     println!("Loaded {i} {name} puzzles");
//
//     puzzles
// }

// fn title_case(name: &str) -> String {
//     let mut sb = String::new();
//     let mut prev_space = true;
//     for c in name.chars() {
//         let c = if c == '_' { ' ' } else { c };
//         if c == ' ' {
//             prev_space = true;
//             sb.push(c);
//             continue;
//         }
//
//         if prev_space {
//             sb.push(c.to_ascii_uppercase());
//             prev_space = false;
//             continue;
//         }
//
//         sb.push(c);
//     }
//     sb
// }

// fn main() {
//     let mut puzzles = Vec::new();
//
//     cprintln!("<b, bold>Loading puzzles...");
//     for name in ["very_easy", "easy", "medium", "hard"] {
//         puzzles.push((name, load_puzzles(name)))
//     }
//     cprintln!("<g, bold>Done");
//
//     cprint!("<b, bold>Validating solver... ");
//
//     for (name, puzzles) in &puzzles {
//         let name = title_case(name);
//         for (i, (puzzle, solution)) in puzzles.iter().enumerate() {
//             let s = solve_backtracking(puzzle.clone());
//             let solution = solution.clone();
//             if s != solution {
//                 cprintln!("<r, bold>\nFailed to solve {name} - {i}:");
//                 if let Some(s) = s {
//                     println!("Given solution:\n{s}")
//                 }
//                 else {
//                     println!("No solution found")
//                 }
//
//                 if let Some(solution) = solution {
//                     println!("Actual solution:\n{solution}")
//                 }
//                 else {
//                     println!("No actual solution")
//                 }
//             }
//         }
//     }
//
//     cprintln!("<g, bold>Done");
//
//     cprintln!("<b, bold>Measuring performance...");
//
//     const INDIVIDUAL_RUNS: usize = 100_000;
//     for (name, puzzles) in &puzzles {
//         let name = title_case(name);
//         let puzzles = puzzles.iter().map(|(s, _)| s.clone()).collect_vec();
//         let total_solved = INDIVIDUAL_RUNS * puzzles.len();
//         cprint!("<b, bold>Timing {name} [{} runs | {} puzzles]", INDIVIDUAL_RUNS.separate_with_commas(), total_solved.separate_with_commas());
//         stdout().flush().ok();
//         let start = Instant::now();
//         for _ in 0..INDIVIDUAL_RUNS {
//             for puzzle in &puzzles {
//                 let s = solve_backtracking(puzzle.clone());
//                 black_box(s);
//             }
//         }
//         let duration = start.elapsed();
//
//         cprintln!("\r<g, bold>{name} $ Total solved: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
//             total_solved.separate_with_commas(),
//             duration,
//             duration / total_solved as u32,
//             (1f64 / (duration / total_solved as u32).as_secs_f64()) as usize,
//         );
//     }
//
//     const MIXED_RUNS: usize = 100_000;
//     let puzzles = puzzles.into_iter().map(|(_, v)| v).flatten().map(|(p, _)| p).collect_vec();
//     let total_solved = MIXED_RUNS * puzzles.len();
//     cprint!("<b, bold>Timing mixed [{} runs | {} puzzles]", MIXED_RUNS.separate_with_commas(), total_solved.separate_with_commas());
//     stdout().flush().ok();
//     let start = Instant::now();
//     for _ in 0..MIXED_RUNS {
//         for puzzle in &puzzles {
//             let s = solve_backtracking(puzzle.clone());
//             black_box(s);
//         }
//     }
//     let duration = start.elapsed();
//
//     cprintln!("\r<g, bold>All $ Total solved: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
//         total_solved.separate_with_commas(),
//         duration,
//         duration / total_solved as u32,
//         (1f64 / (duration / total_solved as u32).as_secs_f64()) as usize,
//     );
//
//     // let sample = Solution::load("data/sudoku.txt");
//     //
//     // // println!("{:?}", Possibilities::from_solution(&sample));
//     //
//     // let start = Instant::now();
//     // let runs = 1_000_000;
//     // for _ in 0..runs {
//     //     let solution = solve_backtracking(sample.clone());
//     //     black_box(solution);
//     // }
//     // let duration = start.elapsed();
//     //
//     // if let Some(solution) = solve_backtracking(sample) {
//     //     println!("{}", solution);
//     // } else {
//     //     println!("No solution found");
//     // }
//     //
//     // println!(
//     //     "Runs: {} | Duration: {:?} | Time per: {:?} | Per second: {}",
//     //     runs.separate_with_commas(),
//     //     duration,
//     //     duration / runs,
//     //     (1f64 / (duration / runs).as_secs_f64()) as usize,
//     // );
// }

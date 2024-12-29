use crate::sudoku_test::board::Possibilities;
use core::fmt::{Display, Formatter};

#[derive(Clone)]
pub struct Solution {
    inner: [[u8; 9]; 9],
    remaining: u8,
}

impl Solution {
    // pub fn load<P: AsRef<Path>>(path: P) -> Self {
    //     let contents = fs::read_to_string(path).expect("File read error");
    //     Self::load_string(contents)
    // }

    pub fn load_string(contents: &str) -> Self {
        let mut board = [[9; 9]; 9];
        let mut remaining = 81;

        for (i, line) in contents.lines().enumerate() {
            if i > 8 && !line.trim().is_empty() {
                panic!("Too many lines");
            } else if i > 8 {
                continue;
            }
            for (j, c) in line.chars().enumerate() {
                if j > 8 {
                    panic!("Line too long in input");
                } else if j > 8 {
                    continue;
                }
                if c == '_' || c == '0' {
                    continue;
                } else {
                    let n = c.to_digit(10);
                    if let Some(n) = n {
                        board[i][j] = n as u8 - 1;
                        remaining -= 1;
                    } else {
                        panic!("Expected number, found {c}")
                    }
                }
            }
        }

        Solution {
            inner: board,
            remaining,
        }
    }

    pub fn is_valid(&self) -> bool {
        let mut possibilites = Possibilities::new_full();

        for y in 0..9 {
            for x in 0..9 {
                let cell = self.inner[y][x];
                if cell == 9 {
                    continue;
                }
                if !possibilites.get(x, y).has(cell) {
                    return false;
                }
                possibilites.update_found(x, y, cell);
            }
        }

        true
    }

    pub fn undo(&mut self, revert: &[(u8, u8)]) {
        for (x, y) in revert {
            self.set(*x as usize, *y as usize, 9);
        }
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        let cur = self.inner[y][x];
        if cur != 9 && val == 9 {
            self.remaining += 1;
        } else if cur == 9 && val != 9 {
            self.remaining -= 1;
        }
        self.inner[y][x] = val;
    }

    #[inline]
    pub fn solved(&self) -> bool {
        self.remaining == 0
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize) -> u8 {
        self.inner[y][x]
    }
}

impl PartialEq for Solution {
    fn eq(&self, other: &Self) -> bool {
        #[cfg(not(debug_assertions))]
        return self.inner == other.inner;
        #[cfg(debug_assertions)]
        {
            let result = self.inner == other.inner;
            if result {
                debug_assert!(self.remaining == other.remaining)
            }
            result
        }
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        for row in 0..9 {
            write!(f, "│ ")?;
            for col in 0..9 {
                let n = self.get(col, row);
                if n != 9 {
                    write!(f, "{}", n + 1)?;
                } else {
                    write!(f, "-")?;
                }

                if (col + 1) % 3 == 0 && col != 8 {
                    write!(f, " │ ")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "│")?;

            if (row + 1) % 3 == 0 && row != 8 {
                writeln!(f, "├───────┼───────┼───────┤")?;
            }
        }
        writeln!(f, "└───────┴───────┴───────┘")?;

        Ok(())
    }
}

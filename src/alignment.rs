use core::fmt;
use std::cmp::max;

use crate::cell;
pub struct Alignment<'a, 'b> {
    pub matrix: Box<Vec<Vec<cell::Cell>>>,
    first: &'a str,
    second: &'b str,
}

impl<'a, 'b> Alignment<'a, 'b> {
    pub fn new(first: &'a str, second: &'b str) -> Alignment<'a, 'b> {
        let mut initialized_matrix = Box::new(Vec::new());

        for i in 0..first.chars().count() as i64 + 1 {
            let mut row = Vec::new();
            row.push(cell::Cell{value: i * -1, direction: cell::Direction::Up, value_type: cell::ValueType::Unknown});
            initialized_matrix.push(row);
        }

        for j in 1..second.chars().count() as i64 + 1 {
            initialized_matrix[0].push(cell::Cell{value: j * -1, direction: cell::Direction::Left, value_type: cell::ValueType::Unknown});
        }

        for i in 1..first.chars().count() + 1 {
            for _ in 1..second.chars().count() + 1 {
                initialized_matrix[i].push(cell::Cell{value: 0, direction: cell::Direction::Unset, value_type: cell::ValueType::Unknown});
            }
        }

        Alignment{
            matrix: initialized_matrix,
            first,
            second,
        }
    }


    pub fn calculate_score(&mut self) -> Result<(), &'static str> {
        // weights for scores
        const MATCH: i64 = 1;
        const MISMATCH: i64 = -1;
        const INDEL: i64 = -1;

        for i in 1..self.first.chars().count() + 1 {
            for j in 1..self.second.chars().count() + 1 {
                // check if correlating letters are a match
                if self.first.chars().nth(i - 1) == None {
                    println!("i: {}", i);
                    return Err("Out of bounds");
                } else if self.second.chars().nth(j - 1) == None {
                    println!("j: {}", j);
                    return Err("Out of bounds");
                }
                let match_value = match self.first.chars().nth(i - 1).unwrap() == self.second.chars().nth(j - 1).unwrap() {
                    true => {
                        self.matrix[i][j].value_type = cell::ValueType::Static;
                        MATCH
                    },
                    false => {
                        self.matrix[i][j].value_type = cell::ValueType::Dynamic;
                        MISMATCH
                    },
                };

                let diagonal = self.matrix[i-1][j-1].value + match_value;
                let up = self.matrix[i-1][j].value + INDEL;
                let left = self.matrix[i][j-1].value + INDEL;

                // store as value
                let max_val = max(max(diagonal, up), left);
                self.matrix[i][j].value = max_val;

                // set Direction
                if left == max_val {
                    self.matrix[i][j].direction = cell::Direction::Left;
                } else if up == max_val {
                    self.matrix[i][j].direction = cell::Direction::Up;
                } else {
                    self.matrix[i][j].direction = cell::Direction::Diagonal;
                }

                // if i == 2 && j == 3 {
                //     println!("diagonal: {}", diagonal);
                //     println!("left: {}", left);
                //     println!("up: {}", up);
                //     println!("first: {}", self.first.chars().nth(i).unwr);
                //     println!("second: {}", self.second.chars().nth(j));
                // } 
            }
        }

        Ok(())
    }

    pub fn align(&self) -> (String, String) {
        let mut first = String::from("");
        let mut second = String::from("");
        let first_len = self.first.chars().count();
        let second_len = self.second.chars().count();

        // set starting index to bottom right corner
        let mut i = first_len;
        let mut j = second_len;

        while i != 0 && j != 0 {
            let current_cell = &self.matrix[i][j];
            match current_cell.direction {
                cell::Direction::Diagonal => {
                    first = self.first.chars().nth(i - 1).unwrap().to_string() + &first;
                    second = self.second.chars().nth(j - 1).unwrap().to_string() + &second;
                    i = i - 1;
                    j = j - 1;
                },
                cell::Direction::Left => {
                    first = "_".to_owned() + &first;
                    second = self.second.chars().nth(j - 1).unwrap().to_string() + &second;
                    j = j - 1;
                },
                cell::Direction::Up => {
                    first = self.first.chars().nth(i - 1).unwrap().to_string() + &first;
                    second = "_".to_owned() + &second;
                    i = i - 1;
                },
                cell::Direction::Unset => panic!("Didnt get set"),
            };
        }

        return (first, second);
    }
}

impl<'a, 'b> fmt::Debug for Alignment<'a, 'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut r = std::result::Result::Ok(());
        for i in 0..self.matrix.len() {
            let mut s = String::from("{ ");
            for j in 0..self.matrix[0].len() {
                s = s + match self.matrix[i][j].direction {
                    cell::Direction::Left => "\u{2190}",
                    cell::Direction::Up => "\u{2191}",
                    cell::Direction::Diagonal => "\u{2B09}",
                    cell::Direction::Unset => " ",
                };
                if self.matrix[i][j].value > -1 {
                    s = s + "+";
                }
                s = s + &self.matrix[i][j].value.to_string() + " ";
            }
            s = s + "}\n";
            r = f.write_str(&s);
        }
        r
    }
}
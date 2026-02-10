use std::collections::HashSet;

struct ValidSudoku {}

impl ValidSudoku {
    pub fn is_valid_set(set: &Vec<char>) -> bool {
        let mut traveled: HashSet<char> = HashSet::new();
        for c in set {
            if c.eq(&'.') {
                continue;
            }
            if traveled.contains(c) {
                return false;
            }
            traveled.insert(*c);
        }
        return true;
    }

    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let size = 9;
        for i in 0..size {
            // scan rows
            let mut is_valid = Self::is_valid_set(&board[i]);
            if !is_valid {
                return false;
            }

            // scan columns
            is_valid = Self::is_valid_set(&vec![
                board[0][i],
                board[1][i],
                board[2][i],
                board[3][i],
                board[4][i],
                board[5][i],
                board[6][i],
                board[7][i],
                board[8][i],
            ]);
            if !is_valid {
                return false;
            }
            // scan sub-boxes
            if i == 0 || i == 3 || i == 6 {
                let set = vec![
                    board[i][0],
                    board[i][1],
                    board[i][2],
                    board[i + 1][0],
                    board[i + 1][1],
                    board[i + 1][2],
                    board[i + 2][0],
                    board[i + 2][1],
                    board[i + 2][2],
                ];
                is_valid = Self::is_valid_set(&set);
                if !is_valid {
                    return false;
                }

                let set2 = vec![
                    board[i][3],
                    board[i][4],
                    board[i][5],
                    board[i + 1][3],
                    board[i + 1][4],
                    board[i + 1][5],
                    board[i + 2][3],
                    board[i + 2][4],
                    board[i + 2][5],
                ];
                is_valid = Self::is_valid_set(&set2);
                if !is_valid {
                    return false;
                }

                let set3 = vec![
                    board[i][6],
                    board[i][7],
                    board[i][8],
                    board[i + 1][6],
                    board[i + 1][7],
                    board[i + 1][8],
                    board[i + 2][6],
                    board[i + 2][7],
                    board[i + 2][8],
                ];
                is_valid = Self::is_valid_set(&set3);
                if !is_valid {
                    return false;
                }
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let input: Vec<Vec<char>> = vec![
            vec!['.', '.', '4', '.', '.', '.', '6', '3', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['5', '.', '.', '.', '.', '.', '.', '9', '.'],
            vec!['.', '.', '.', '5', '6', '.', '.', '.', '.'],
            vec!['4', '.', '3', '.', '.', '.', '.', '.', '1'],
            vec!['.', '.', '.', '7', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '5', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
            vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
        ];
        let result = ValidSudoku::is_valid_sudoku(input);
        println!("Result: {}", result)
    }
}
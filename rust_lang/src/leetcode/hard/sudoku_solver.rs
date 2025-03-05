// const N: &[char] = &['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const CHARS: &str = "123456789";
const N: usize = 9;
const DOT: char = '.';

#[derive(Debug, Clone)]
pub struct BitMasksSudoKu {
    pub row: Vec<u16>,
    pub col: Vec<u16>,
    pub boxz: Vec<u16>,
    pub seted: bool,
}

impl BitMasksSudoKu {
    pub fn get_box(i: usize, j: usize) -> usize {
        return i / 3 * 3 + j / 3;
    }

    pub fn is_safe(&self, i: usize, j: usize, number: char) -> bool {
        return (self.row[i].rotate_right(number as u32) & 1) == 0
            && (self.col[j].rotate_right(number as u32) & 1) == 0
            && (self.boxz[Self::get_box(i, j)].rotate_right(number as u32) & 1) == 0;
    }

    pub fn set_initial_values(&mut self, board: &Vec<Vec<char>>) {
        for i in 0..board.len() {
            for j in 0..board.len() {
                self.row[i] |= 1_u16.rotate_left(board[i][j] as u32);
                self.col[j] |= 1_u16.rotate_left(board[i][j] as u32);
                self.boxz[Self::get_box(i, j)] |= 1_u16.rotate_left(board[i][j] as u32);
            }
        }
    }
    pub fn run(&mut self, board: &mut Vec<Vec<char>>, mut i: usize, mut j: usize) -> bool {
        if i == N - 1 && j == N {
            return true;
        }
        if j == N {
            j = 0;
            i += 1;
        }

        if board[i][j] != DOT {
            return self.run(board, i, j + 1);
        }

        for nr in CHARS.chars() {
            if self.is_safe(i, j, nr) {
                board[i][j] = nr;
                self.row[i] |= 1_u16.rotate_left(nr as u32);
                self.col[j] |= 1_u16.rotate_left(nr as u32);
                self.boxz[Self::get_box(i, j)] |= 1_u16.rotate_left(nr as u32);

                if self.run(board, i, j + 1) { return true; }

                self.row[i] &= !1_u16.rotate_left(nr as u32);
                self.col[j] &= !1_u16.rotate_left(nr as u32);
                self.boxz[Self::get_box(i, j)] &= !1_u16.rotate_left(nr as u32);
            }
            board[i][j] = DOT;
        }
        return false;
    }
    /// Create 3 arrays of size N (one for rows, columns, and boxes)
    /// The boxes are indexed 0 to 8. (in order to find the box index of an element: int(row/3*3+col/3)
    /// Map the initial values of the grid first
    /// Each time add/remove an element to/from the grid set the bit to 1/0 to corresponding bitmasks
    pub fn solve_sudoku(&mut self, board: &mut Vec<Vec<char>>) {
        if !self.seted {
            self.seted = true;
            self.set_initial_values(board);
        }
        self.run(board, 0, 0);
        Self::print(board);
    }

    pub fn print(board: &Vec<Vec<char>>) {
        for i in 0..N {
            for j in 0..N {
                print!("{} ", board[i][j]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::leetcode::hard::sudoku_solver::BitMasksSudoKu;

    #[test]
    fn run() {
        let mut input: Vec<Vec<char>> = vec![
            vec!['1', '9', '6', '.', '5', '3', '.', '.', '.'],
            vec!['7', '.', '.', '2', '1', '.', '3', '.', '.'],
            vec!['.', '3', '.', '.', '.', '6', '.', '.', '7'],
            vec!['.', '7', '.', '.', '.', '2', '4', '1', '.'],
            vec!['5', '6', '1', '3', '7', '4', '.', '.', '.'],
            vec!['4', '.', '.', '.', '.', '.', '7', '.', '.'],
            vec!['6', '1', '.', '.', '2', '.', '5', '.', '9'],
            vec!['2', '.', '.', '6', '.', '.', '8', '3', '.'],
            vec!['.', '8', '.', '5', '3', '.', '.', '7', '2'],
        ];
        let mut bitmasks = BitMasksSudoKu {
            row: vec![0; 9],
            col: vec![0; 9],
            boxz: vec![0; 9],
            seted: false,
        };
        bitmasks.solve_sudoku(&mut input);
    }
}

// struct CrossHatchingSudoku {}
//
// impl CrossHatchingSudoku {
//     pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {}
// }
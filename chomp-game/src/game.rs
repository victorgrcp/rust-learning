#[derive(Debug)]
pub struct Game {
    board: Vec<Vec<bool>>,
}

impl Game {
    pub fn new(rows: usize, cols: usize) -> Game {
        Game {
            board: vec![vec![false; cols]; rows],
        }
    }
    pub fn show_board(&self) {
        for row in &self.board {
            for col in row {
                if *col {
                    print!("X ");
                } else {
                    print!("O ");
                }
            }
            println!("");
        }
    }
    fn valid_move(&self, n: usize, m: usize) -> bool {
        if n > self.board.len() || m > self.board[0].len() {
            return false;
        }
        else if self.board[n][m] == true {
            return false;
        }
        true
    }
    pub fn fill_board(&mut self, n: usize, m: usize) -> bool {
        if valid_move {
            for i in (0..n).rev() {
                for j in 0..self.board.len() {
                    self.board[i][j] = true;
                }
            }
        }
    }
}

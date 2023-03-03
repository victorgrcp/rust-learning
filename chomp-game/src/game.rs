pub enum Move {
    Valid,
    Invalid,
    Final
}
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
        for i in 0..self.board[0].len() {
            print!("{i} ");
        }
        println!("");
        for (i, row) in self.board.iter().enumerate() {
            for col in row {
                if *col {
                    print!("X ");
                } else {
                    print!("O ");
                }
            }
            print!("<-- {} \n", i);
        }
    }
    fn valid_move(&self, n: usize, m: usize) -> bool {
        if n >= self.board.len() || m >= self.board[0].len() {
            return false;
        }
        else if self.board[n][m] == true {
            return false;
        }
        true
    }
    pub fn fill_board(&mut self, n: usize, m: usize) -> Move {
        if self.valid_move(n, m) {
            for i in (0..n+1).rev() {
                for j in m..self.board.len() {
                    self.board[i][j] = true;
                }
            }
            if n==self.board.len()-1 && m==0 {
                return Move::Final;
            } 
            return Move::Valid;
        } Move::Invalid
    }
}

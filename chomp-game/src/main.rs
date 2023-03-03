
use game::*;
mod game;

fn main() {
    let mut game = Game::new(10, 10);
    let (n, m) = (9, 0);
    let step: Move = game.fill_board(n, m);
    match step {
        Move::Valid => println!("Valid move!"),
        Move::Invalid => println!("Invalid move!"),
        Move::Final => println!("Final move!"),
    }
    game.show_board();
}


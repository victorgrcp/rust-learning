
use game::Game;
mod game;

fn main() {
    let mut game = Game::new(10, 10);
    let (n, m) = (3, 3);
    game.fill_board(n, m);
    game.show_board();
}


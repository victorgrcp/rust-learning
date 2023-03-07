use game::*;
use player::*;

mod game;
mod player;

fn get_move() -> (usize, usize) {
    let mut n = String::new();
    let mut m = String::new();
    println!("Enter row and column: ");
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    std::io::stdin().read_line(&mut m).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please type a number!");
    let m: usize = m.trim().parse().expect("Please type a number!");
    (n, m)
}

fn main() {
    let mut game = Game::new(10, 10);
    let mut n: usize;
    let mut m: usize;
    let mut step: Move = Move::Valid;

    let mut players: Vec<Player> = vec![
        Player::new("Player 1".to_string()),
        Player::new("Player 2".to_string()),
    ];

    loop {
        for p in &mut players {
            println!("{}'s turn", p.get_name());
            game.show_board();
            loop { // Loop for invalid move
                (n, m) = get_move();
                step = game.fill_board(n, m);
                if step != Move::Invalid { break; };
                println!("Invalid move!");
            }
            match step {
                Move::Valid => println!("\n[*] Valid move! \n"),
                Move::Final => {
                    println!("\n[*] Final move! \n");
                    p.increment_score();
                    break;
                }
                _ => (),
            }
        }
        if step == Move::Final { 
            break;
        }
    }
    game.show_board();
    for p in players {
        println!("{} score: {}", p.get_name(), p.get_score());
    }
}


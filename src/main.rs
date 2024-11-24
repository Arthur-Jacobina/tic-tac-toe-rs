mod player;
mod game;
use game::game::Game;
use player::player::{HumanPlayer, ComputerPlayer, Player};

fn play(game: &mut Game, x_player: &dyn Player, o_player: &dyn Player, print_game: bool) {
    if print_game {
        println!("| 0 | 1 | 2 |\n| 3 | 4 | 5 |\n| 6 | 7 | 8 |");
    }

    let mut letter = 'X';

    while game.empty_squares() {
        let square = if letter == 'O' {
            o_player.get_move(game)
        } else {
            x_player.get_move(game)
        };

        if game.make_move(square, letter) {
            if print_game {
                println!("{} makes a move to square {}", letter, square);
                for row in game.board.chunks(3) {
                    println!("| {} | {} | {} |", row[0], row[1], row[2]);
                }
            }

            if game.current_winner.is_some() {
                if print_game {
                    println!("{} wins!", letter);
                }
                return;
            }

            letter = if letter == 'X' { 'O' } else { 'X' };
        }
    }

    if print_game {
        println!("It's a tie!");
    }
}

fn main() {
    let mut game = Game::new();
    let x_player = ComputerPlayer::new('X');
    let o_player = HumanPlayer::new('O');

    play(&mut game, &x_player, &o_player, true);
}

pub mod player {
    use std::io;
    use crate::game::game::Game;
    pub trait Player {
        fn get_move(&self, game: &Game) -> usize;
    }

    pub struct HumanPlayer {
        pub letter: char,
    }

    impl HumanPlayer {
        pub fn new(letter: char) -> Self {
            Self { letter }
        }
    }

    impl Player for HumanPlayer {
        fn get_move(&self, game: &Game) -> usize {
            loop {
                println!("{}'s turn. Input move (0-8): ", self.letter);
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if let Ok(square) = input.trim().parse::<usize>() {
                    if game.available_moves().contains(&square) {
                        return square;
                    }
                }
                println!("Invalid square. Try again.");
            }
        }
    }

    pub struct ComputerPlayer {
        pub letter: char,
    }

    impl ComputerPlayer {
        pub fn new(letter: char) -> Self {
            Self { letter }
        }

        fn minimax(&self, game: &mut Game, player: char) -> (Option<usize>, i32) {
            let max_player = self.letter;
            let other_player = if player == 'X' { 'O' } else { 'X' };

            if let Some(winner) = game.current_winner {
                if winner == max_player {
                    return (None, 1 * (game.num_empty_squares() as i32 + 1));
                } else {
                    return (None, -1 * (game.num_empty_squares() as i32 + 1));
                }
            } else if !game.empty_squares() {
                return (None, 0);
            }

            let mut best = if player == max_player {
                (None, i32::MIN)
            } else {
                (None, i32::MAX)
            };

            for &possible_move in &game.available_moves() {
                game.make_move(possible_move, player);
                let (_, sim_score) = self.minimax(game, other_player);

                game.board[possible_move] = ' ';
                game.current_winner = None;

                if player == max_player {
                    if sim_score > best.1 {
                        best = (Some(possible_move), sim_score);
                    }
                } else {
                    if sim_score < best.1 {
                        best = (Some(possible_move), sim_score);
                    }
                }
            }

            best
        }
    }

    impl Player for ComputerPlayer {
        fn get_move(&self, game: &Game) -> usize {
            let mut game_clone = game.clone();
            if game_clone.available_moves().len() == 9 {
                rand::random::<usize>() % 9
            } else {
                self.minimax(&mut game_clone, self.letter).0.unwrap()
            }
        }
    }
}

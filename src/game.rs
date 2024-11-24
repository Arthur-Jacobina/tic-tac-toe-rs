pub mod game {
    #[derive(Clone)]
    pub struct Game {
        pub board: Vec<char>,
        pub current_winner: Option<char>,
    }

    impl Game {
        // Create a new game
        pub fn new() -> Self {
            Self {
                board: vec![' '; 9],
                current_winner: None,
            }
        }

        pub fn available_moves(&self) -> Vec<usize> {
            self.board
                .iter()
                .enumerate()
                .filter(|(_, &value)| value == ' ')
                .map(|(index, _)| index)
                .collect()
        }

        pub fn make_move(&mut self, square: usize, letter: char) -> bool {
            if self.board[square] == ' ' {
                self.board[square] = letter;
                if self.winner(square, letter) {
                    self.current_winner = Some(letter);
                }
                true
            } else {
                false
            }
        }

        pub fn empty_squares(&self) -> bool {
            self.board.contains(&' ')
        }

        pub fn num_empty_squares(&self) -> usize {
            self.board.iter().filter(|&&c| c == ' ').count()
        }

        // Check if the player has won
        fn winner(&self, square: usize, letter: char) -> bool {
            let row_index = square / 3;
            let col_index = square % 3;

            // rows
            if self.board[row_index * 3..(row_index * 3 + 3)]
                .iter()
                .all(|&value| value == letter)
            {
                return true;
            }

            // columns
            if (0..3).all(|i| self.board[col_index + i * 3] == letter) {
                return true;
            }

            // diagonals
            if square % 2 == 0 {
                if (0..3).all(|i| self.board[i * 3 + i] == letter) {
                    return true;
                }
                if (0..3).all(|i| self.board[(i + 1) * 2] == letter) {
                    return true;
                }
            }
            false
        }
    }
}
mod player;
mod game;
use game::game::Game;
use player::player::{ComputerPlayer, HumanPlayer, RandomComputerPlayer};


fn main() {
    let mut game = Game::new();
    let x_player = ComputerPlayer::new('X');
    let o_player = RandomComputerPlayer::new('O');

    Game::play(&mut game, &x_player, &o_player, true);
}

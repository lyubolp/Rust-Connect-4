mod human_player;
mod game;
mod bot;

use game::game::GameState;
fn main()
{
    println!("Hello, world!");

    let mut gs: GameState = GameState::init();
    gs.create_and_add_player('+');
    gs.create_and_add_bot('O', 3);

    gs.turn();
}

pub mod human_player{
    use crate::game::game::{Player, GameState, GAME_BOARD_SIZE};
    use std::io::{self, Read};

    pub struct HumanPlayer{
        board_value: u8,
        board_symbol: char
    }

    pub fn read_from_keyboard() -> u8
    {
        let default_value: u8 = 0;
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer){
            Ok(n) =>{
                match buffer.parse::<u8>(){
                    Ok(data) => data,
                    Err(err) => {
                        println!("Error: {}\n Setting it to default value {}", err, default_value);
                        default_value
                    }
                }
            }
            Err(error) => {
                println!("error: {}\n Setting it to default value: {}", error, default_value);
                default_value
            }
        }
    }
    impl HumanPlayer{
        pub fn new(board_value: u8, board_symbol: char) -> HumanPlayer{
            HumanPlayer {
                board_value,
                board_symbol
            }
        }
    }
    impl Player for HumanPlayer{

        fn play(&self, game_state: Option<GameState>) -> u8 {
            println!("Please enter a column number (1-{}): ", GAME_BOARD_SIZE.0);
            read_from_keyboard()
        }

        fn get_board_value(&self) -> u8
        {
            self.board_value
        }
        fn get_board_symbol(&self) -> char
        {
            self.board_symbol
        }
    }
}
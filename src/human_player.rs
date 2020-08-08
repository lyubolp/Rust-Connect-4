pub mod human_player{
    use crate::game::game::{Player, GameState, GAME_BOARD_SIZE, PlayerType};
    use std::io::{self};


    #[derive(Copy, Clone)]
    pub struct HumanPlayer{
        board_value: u8,
        board_symbol: char
    }

    pub fn read_from_keyboard(filled_columns: [u8; GAME_BOARD_SIZE.1]) -> Option<u8>
    {
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer){
            Ok(_) =>{
                let first_symbol = buffer.chars().next().unwrap();
                match first_symbol.to_digit(10){
                    Some(data) => {
                        if (1 <= data && data <= GAME_BOARD_SIZE.1 as u32) && filled_columns[(data - 1) as usize] < GAME_BOARD_SIZE.0 as u8 {
                            Some(data as u8)
                        }
                        else{
                            None
                        }
                    },
                    None => None
                }
            },
            Err(_) => None
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

        fn play(&self, game_state: &GameState) -> u8 {
            println!("Please enter a column number (1-{}): ", GAME_BOARD_SIZE.1);
            loop{
                if let Some(value) = read_from_keyboard(game_state.get_filled_columns()){
                    return value
                }
            }
        }

        fn get_board_value(&self) -> u8
        {
            self.board_value
        }
        fn get_board_symbol(&self) -> char
        {
            self.board_symbol
        }

        fn get_type(&self) -> PlayerType { PlayerType::Human}
    }

    #[cfg(test)]
    mod tests{
        use super::*;

        #[test]
        fn test_getters(){
            let player: HumanPlayer = HumanPlayer::new(1, '+');

            assert_eq!(player.get_board_symbol(), '+');
            assert_eq!(player.get_board_value(), 1);
        }
    }
}
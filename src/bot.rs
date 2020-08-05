mod bot{
    use crate::game::game::{GameState, GAME_BOARD_SIZE, PLAYER_ONE_VALUE, MoveType, Player};
    struct Bot{

    }

    fn does_player_have_winning_move(game_state: &GameState) -> Option<(u8, u8, MoveType)>{
        for (column_index, column) in game_state.get_field().iter().enumerate(){
            for (row_index, value) in column.iter().enumerate(){
                if *value == PLAYER_ONE_VALUE{
                    let move_check = game_state.is_there_winning_move_from((column_index as u8, row_index as u8));
                    match move_check.0{
                        true => return Some((column_index as u8, row_index as u8, move_check.1.unwrap())),
                        false => ()
                    };
                }
            }
        };
        None
    }
    impl Bot{
        fn evaluate_board_with_move(&self, game_state: &GameState, column_to_place: u8) -> u32 {
            //TODO
            3
        }
        fn take_decision(&self, game_state: &GameState) -> u8{
            //TODO
            3
        }

        fn block_player_move(&self, player_move: (u8, u8, MoveType)) -> u8{
            match player_move.2{
                MoveType::Vertical => {
                    player_move.0
                },
                MoveType::Horizontal => {
                    player_move.0 + 3
                },
                MoveType::LeftDiagonal => {
                    if player_move.0 - 3 < 0{
                        panic!("Invalid move")
                    }
                    player_move.0 - 3
                },
                MoveType::RightDiagonal => {
                    player_move.0 + 3
                }
            }
        }

        /*pub fn play(&self, game_state: &GameState) -> u8{
            match does_player_have_winning_move(game_state){
                Some(where_to) => {
                    self.block_player_move(where_to)
                }
                None => self.take_decision(game_state)
            }
        }*/
    }

    impl Player for Bot{
        fn init() -> Bot{
            Bot{}
        }
        fn play(&self, game_state: Option<GameState>) -> u8{
            match game_state{
                Some(game_state_object) => match does_player_have_winning_move(&game_state_object){
                    Some(where_to) => {
                        self.block_player_move(where_to)
                    }
                    None => self.take_decision(&game_state_object)
                }
                None => panic!("Please pass the GameState object to the bot")
            }
        }
    }
}
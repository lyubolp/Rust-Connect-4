pub mod bot{
    use crate::game::game::{GameState, GAME_BOARD_SIZE, MoveType, Player, PlayerType, EMPTY_PLACE_VALUE};
    use std::borrow::BorrowMut;


    #[derive(Copy, Clone)]
    pub struct Bot{
        board_value: u8,
        board_symbol: char,
        level: u8
    }

    fn does_player_have_winning_move(game_state: &GameState) -> Option<(u8, u8, MoveType)>{
        //TOOD - Refactor
        let human_player_id: Vec<u8> = game_state.get_human_players_ids();
        for new_move_column in 1..=GAME_BOARD_SIZE.1{
            let mut new_game_state= game_state.clone();

            if human_player_id.len() == 1{
                new_game_state.place_on_board(new_move_column as u8, human_player_id[0]);
            }
            else{
                panic!("Not yet implemented");
            }

            let winning_move_tuple: (bool, Option<MoveType>) = new_game_state.check_for_win();
            if winning_move_tuple.0{
                return Some((GAME_BOARD_SIZE.0 as u8 - new_game_state.get_filled_columns()[new_move_column - 1], (new_move_column) as u8, winning_move_tuple.1.unwrap()));
            }
        }
        None
    }
    impl Bot{
        fn generate_game_states(game_state: &GameState, id_to_play: u8) -> Vec<GameState>{
            let mut game_states: Vec<GameState> = Vec::new();
            for new_move_column in 1..=GAME_BOARD_SIZE.1{
                let mut new_game_state= game_state.clone();
                new_game_state.place_on_board(new_move_column as u8, id_to_play);
                game_states.push(new_game_state);
            };
            game_states
        }
        fn evaluate_board(game_state: &GameState) -> i16{
            let mut placed = 0;
            for row in game_state.get_field().iter(){
                for cell in row.iter(){
                    if cell != EMPTY_PLACE_VALUE{
                        placed+= 1;
                    }
                }
            }
            let our_win: bool = (placed % 2 == 0);

            if game_state.check_for_win().0{
                if our_win{
                    1000
                }
                else{
                    -1000
                }
            }
            else{
                0
            }
        }
        fn take_decision(&self, game_state: &GameState) -> u8{
            //TODO
            3
        }

        fn block_player_move(&self, player_move: (u8, u8, MoveType)) -> u8{
            match player_move.2{
                MoveType::Vertical => {
                    player_move.1
                },
                MoveType::Horizontal => {
                    player_move.1
                },
                MoveType::LeftDiagonal => {
                    if player_move.1 - 3 < 0{
                        panic!("Invalid move")
                    }
                    player_move.1
                },
                MoveType::RightDiagonal => {
                    player_move.1
                }
            }
        }

        pub fn new(board_value: u8, board_symbol: char, level: u8) -> Bot{
            Bot{
                board_value,
                board_symbol,
                level
            }
        }
    }

    impl Player for Bot{
        fn play(&self, game_state: &GameState) -> u8{
            if let Some(winning_move) = does_player_have_winning_move(game_state){
                self.block_player_move(winning_move)
            }
            else{
                self.take_decision(game_state)
            }
        }
        fn get_board_value(&self) -> u8{
            self.board_value
        }
        fn get_board_symbol(&self) -> char {
            self.board_symbol
        }

        fn get_type(&self) -> PlayerType { PlayerType::Bot}
    }
}
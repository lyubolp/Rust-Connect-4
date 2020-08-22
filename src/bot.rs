pub mod bot{
    use crate::game::game::{GameState, GAME_BOARD_SIZE, MoveType, Player, PlayerType, EMPTY_PLACE_VALUE};
    use std::borrow::BorrowMut;
    use std::f32::{INFINITY, NEG_INFINITY};
    use std::i32::{MAX, MIN};
    use std::cmp::max;


    #[derive(Copy, Clone)]
    pub struct Bot{
        board_value: u8,
        board_symbol: char,
        level: u8
    }

    fn does_player_have_winning_move(game_state: &GameState) -> Option<(u8, u8, MoveType)>{
        //TODO - Refactor
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
        fn generate_game_states(&self, game_state: &GameState, id_to_play: u8) -> Vec<GameState>{
            let mut game_states: Vec<GameState> = Vec::new();
            for new_move_column in 1..=GAME_BOARD_SIZE.1{
                let mut new_game_state= game_state.clone();
                new_game_state.place_on_board(new_move_column as u8, id_to_play);
                game_states.push(new_game_state);
            };
            game_states
        }
        fn evaluate_board(&self, game_state: &GameState) -> i32{
            let is_bot_next_move: bool = (game_state.get_next_player_to_move_id() == self.board_value); //If the next move is for the bot

            if game_state.check_for_win().0{
                if is_bot_next_move {
                    MIN
                }
                else{
                    MAX
                }
            }
            else{
                let mut max_game_value = game_state.get_max_connected_from(GAME_BOARD_SIZE.0 - 1, 0).0;

                for (row_index, row) in game_state.get_field().iter().enumerate(){
                    for (column_index, _) in row.iter().enumerate(){
                        let current_game_state_score = game_state.get_max_connected_from(GAME_BOARD_SIZE.0 - 1 - row_index, column_index);
                        max_game_value = max(max_game_value, current_game_state_score.0);
                    }
                }

                if is_bot_next_move {
                    -1 * max_game_value as i32
                }
                else{
                    max_game_value as i32
                }
            }
        }
        fn take_decision(&self, game_state: &GameState) -> u8{
            let mut current_game_states: Vec<(GameState, i32)> = vec!((game_state.clone(), 0));
            /*for depth_level in 0..self.level{
                for current_game in current_game_states{
                    self.generate_game_states(&current_game.0, current_game.0.get_next_player_to_move_id());
                }
            };*/
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
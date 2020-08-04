pub mod game {
    use std::collections::HashMap;

    use crate::human_player::human_player::HumanPlayer;
    use std::borrow::{Borrow, BorrowMut};

    pub const EMPTY_PLACE_VALUE: u8 = 0;
    pub const EMPTY_PLACE_SYMBOL: char = '_';

    pub const WINNING_LENGTH: u8 = 4;

    pub const GAME_BOARD_SIZE: (usize, usize) = (6, 7); //(rows, cols)

    pub trait Player {
        fn play(&self, game_state: &GameState) -> u8;

        fn get_board_value(&self) -> u8;
        fn get_board_symbol(&self) -> char;
    }

    pub struct GameState {
        field: [[u8; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
        last_filled_for_column: [u8; GAME_BOARD_SIZE.1],
        values_players: HashMap<u8, Box<dyn Player>>,
        is_game_playing: bool,
    }

    pub enum MoveType {
        Horizontal,
        Vertical,
        LeftDiagonal,
        RightDiagonal,
    }

    impl GameState {
        //Constructor
        pub fn init() -> GameState {
            let temp = GameState {
                field: [[EMPTY_PLACE_VALUE; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
                last_filled_for_column: [0; GAME_BOARD_SIZE.1],
                values_players: HashMap::new(),
                is_game_playing: true,
            };
            temp
        }

        //Getters
        pub fn get_field(&self) -> [[u8; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0] {
            self.field
        }
        pub fn get_filled_columns(&self) -> [u8; GAME_BOARD_SIZE.1] {
            self.last_filled_for_column
        }
        fn get_values_players(&self) -> &HashMap<u8, Box<dyn Player>>{
            &self.values_players
        }

        //Public interactions
        pub fn create_and_add_player(&mut self, symbol: char) {
            let temp_value = (self.values_players.len() + 1) as u8;
            let temp_player = HumanPlayer::new(temp_value, symbol);
            self.values_players.insert(temp_value, Box::new(temp_player));
        }
        pub fn place_on_board(&mut self, column: u8, played_id: u8) {
            let row = GAME_BOARD_SIZE.0 as u8 - self.last_filled_for_column[(column - 1) as usize] - 1;

            let board_value = match self.values_players.get(&played_id) {
                Some(player) => player.get_board_value(),
                None => panic!("Invalid played_id")
            };
            self.field[row as usize][(column - 1) as usize] = board_value;
            self.last_filled_for_column[(column - 1) as usize] += 1;
        } //TODO Maybe it should be private ?
        pub fn turn(&mut self) {
            while self.is_game_playing {
                for player_id in 1..self.values_players.len() + 1 {
                    self.draw_board();
                    println!("Player {}'s turn", player_id);
                    let player = self.values_players.get(&((player_id) as u8));
                    match player {
                        Some(player) => {
                            let column = player.play(self);
                            self.place_on_board(column, player_id as u8);
                            if self.check_for_win(){
                                println!("Player {} won !", player_id);
                                self.is_game_playing = false;
                                break;
                            }
                        }
                        None => panic!("Player not found")
                    }
                }
            }
        }
        pub fn check_for_win(&self) -> bool {
            for row_iter_index in 0..GAME_BOARD_SIZE.0 {
                for column_index in 0..GAME_BOARD_SIZE.1 {
                    let row_index = GAME_BOARD_SIZE.0 - row_iter_index - 1;

                    if self.field[row_index][column_index] != EMPTY_PLACE_VALUE &&
                        self.is_there_winning_move_from((row_index, column_index)).0 {
                        return true;
                    }
                }
            }
            false
        }
        pub fn is_there_winning_move_from(&self, (row, column): (usize, usize)) -> (bool, Option<MoveType>) {
            let mut vertical_win: bool = column <= (GAME_BOARD_SIZE.1 - WINNING_LENGTH as usize);
            let mut horizontal_win: bool = row <= (GAME_BOARD_SIZE.0 - WINNING_LENGTH as usize);
            let mut right_diagonal_win: bool = vertical_win && horizontal_win;
            let mut left_diagonal_win: bool = WINNING_LENGTH as usize <= column && (GAME_BOARD_SIZE.0 - WINNING_LENGTH as usize) <= row;

            for offset in 1..WINNING_LENGTH as usize {
                if vertical_win && self.field[row][column] != self.field[row][column + offset] {
                    vertical_win = false;
                }
                if horizontal_win && self.field[row][column] != self.field[row + offset][column] {
                    horizontal_win = false;
                }
                if right_diagonal_win && self.field[row][column] != self.field[row + offset][column + offset] {
                    right_diagonal_win = false;
                }
                if left_diagonal_win && self.field[row][column] != self.field[row + offset][column - offset] {
                    left_diagonal_win = false;
                }
            }

            let win_type: Option<MoveType> = {
                if vertical_win {
                    Some(MoveType::Vertical)
                } else if horizontal_win {
                    Some(MoveType::Horizontal)
                } else if left_diagonal_win {
                    Some(MoveType::LeftDiagonal)
                } else if right_diagonal_win {
                    Some(MoveType::RightDiagonal)
                } else {
                    None
                }
            };

            if vertical_win || horizontal_win || left_diagonal_win || right_diagonal_win {
                (true, win_type)
            } else {
                (false, win_type)
            }
        }

        //Private interactions
        fn draw_board(&self) {
            for column_name in 1..=GAME_BOARD_SIZE.1 {
                print!("{} ", column_name);
            }
            print!("\n");
            for column in self.field.iter() {
                for cell in column.iter() {
                    match self.values_players.get(cell) {
                        Some(player) => print!("{} ", player.get_board_symbol()),
                        None => print!("{} ", EMPTY_PLACE_SYMBOL)
                    }
                }
                print!("\n");
            }
        }


    }
    //Tests
    #[cfg(test)]
    mod unit_tests{
        use super::*;
        use crate::human_player::human_player::HumanPlayer;

        #[test]
        fn test_init(){
            let mut gs = GameState::init();
            assert_eq!(gs.get_field(), [[EMPTY_PLACE_VALUE; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0]);
            assert_eq!(gs.get_filled_columns(), [0; GAME_BOARD_SIZE.1])
        }

        #[test]
        fn test_create_two_players(){
            let mut gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            assert_eq!(gs.get_values_players().get(&1u8).unwrap().get_board_symbol(), '+');
            assert_eq!(gs.get_values_players().get(&2u8).unwrap().get_board_symbol(), 'O');
        }

        #[test]
        fn test_place_on_board(){
            let mut gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            gs.place_on_board(1, 1);

            assert_eq!(gs.get_field()[GAME_BOARD_SIZE.0 - 1][0], 1);
            assert_eq!(gs.get_filled_columns()[0], 1);

            gs.place_on_board(1, 2);

            assert_eq!(gs.get_field()[GAME_BOARD_SIZE.0 - 2][0], 2);
            assert_eq!(gs.get_filled_columns()[0], 2);
        }

        #[test]
        fn test_check_for_win(){
            let mut gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            //Horizontal win
            gs.place_on_board(1, 1);
            assert_eq!(gs.check_for_win(), false);

            gs.place_on_board(1, 1);
            gs.place_on_board(1, 1);
            gs.place_on_board(1, 1);

            assert_eq!(gs.check_for_win(), true);

            //Vertical win
            gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            gs.place_on_board(1, 1);
            assert_eq!(gs.check_for_win(), false);

            gs.place_on_board(2,1);
            gs.place_on_board(3,1);
            gs.place_on_board(4,1);

            assert_eq!(gs.check_for_win(), true);

            //Right diagonal win
            gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            gs.place_on_board(1, 1);
            assert_eq!(gs.check_for_win(), false);

            gs.place_on_board(2,2);

            gs.place_on_board(2,1);
            gs.place_on_board(3,2);

            gs.place_on_board(3,1);
            gs.place_on_board(4,2);

            gs.place_on_board(3,1);
            gs.place_on_board(4,2);

            gs.place_on_board(4,1);
            gs.place_on_board(6,2);

            gs.place_on_board(4,1);

            assert_eq!(gs.check_for_win(), true);
        }
    }
}
pub mod game{

    /*pub const PLAYER_ONE_VALUE: u8 = 1;
    pub const COMPUTER_VALUE: u8 = 2;

    pub const PLAYER_ONE_SYMBOL: char = '+';
    pub const COMPUTER_SYMBOL: char = '-';*/
    use std::collections::HashMap;

    use crate::human_player::human_player::HumanPlayer;
    use std::borrow::{Borrow, BorrowMut};

    pub const EMPTY_PLACE_VALUE: u8 = 0;
    pub const EMPTY_PLACE_SYMBOL: char = '_';

    pub const WINNING_LENGTH: u8 = 4;

    pub const GAME_BOARD_SIZE: (usize, usize) = (7, 6); //(cols, rows)

    pub trait Player{
        //fn new(board_value: u8, board_symbol: char) -> Self;
        fn play(&self, game_state: Option<GameState>) -> u8;

        fn get_board_value(&self) -> u8;
        fn get_board_symbol(&self) -> char;
    }

    pub struct GameState {
        field: [[u8; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
        last_filled_for_column: [u8; GAME_BOARD_SIZE.0],
        values_players: HashMap<u8, Box<dyn Player>>
    }

    pub enum MoveType{
        Horizontal,
        Vertical,
        LeftDiagonal,
        RightDiagonal
    }

    impl GameState{
        pub fn init() -> GameState{
            let temp = GameState{
                field: [[EMPTY_PLACE_VALUE; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
                last_filled_for_column: [0;GAME_BOARD_SIZE.0],
                values_players: HashMap::new()
            };
            temp
        }

        pub fn create_and_add_player(&mut self, symbol: char){
            let temp_value = (self.values_players.len() + 1) as u8;
            let temp_player = HumanPlayer::new(temp_value, symbol);
            self.values_players.insert(temp_value, Box::new(temp_player));
        }

        pub fn place_on_board(&mut self, column: u8, played_id: u8)
        {
            let row = self.last_filled_for_column[column as usize] + 1;

            let board_value = match self.values_players.get(&played_id){
                Some(player) => player.get_board_value(),
                None => panic!("Invalid played_id")
            };
            self.field[column as usize][row as usize] = board_value;
            self.last_filled_for_column[column as usize] += 1;
        }

        fn draw_board(&self)
        {
            for column_name in 1..GAME_BOARD_SIZE.0{
                print!("{} ", column_name);
            }
            print!("\n");
            for column in self.field.iter(){
                for cell in column.iter(){
                    match self.values_players.get(cell){
                        Some(player) => println!("{} ", player.get_board_symbol()),
                        None => print!("{} ", EMPTY_PLACE_SYMBOL)
                    }
                }
                print!("\n");
            }
        }

        pub fn turn(&mut self){
            for player_id in 0..self.values_players.len(){
                self.draw_board();
                let player = self.values_players.get(&(player_id as u8));
                match player{
                    Some(player) =>self.place_on_board(player.play(None), player_id as u8),
                    None => panic!("Player not found")
                }
            }
        }

        pub fn get_field(&self) -> [[u8; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0]{
            self.field
        }
        pub fn get_filled_columns(&self) -> [u8; GAME_BOARD_SIZE.0]{
            self.last_filled_for_column
        }
        pub fn is_there_winning_move_from(&self, (column, row): (u8, u8)) -> (bool, Option<MoveType>){
            let mut column_check: bool = true;
            let mut row_check: bool = true;
            let mut right_diagonal_check: bool = true; //upward
            let mut left_diagonal_check: bool = true; //downward
            for offset in 1..WINNING_LENGTH{
                let target_column = column + offset;
                let target_row = row + offset;
                let target_column_left = column - offset;

                if !(target_column < GAME_BOARD_SIZE.0 as u8 || target_row < GAME_BOARD_SIZE.1 as u8 || 0 <= target_column_left)
                {
                    return (false, None);
                }
                if column + WINNING_LENGTH < GAME_BOARD_SIZE.0 as u8 &&
                    self.field[column as usize][row as usize] != self.field[(column + offset) as usize][row as usize]
                {
                    column_check = false;
                }
                if self.field[column as usize][row as usize] != self.field[column as usize][(row + offset) as usize]
                {
                    row_check = false;
                }
                if self.field[column as usize][row as usize] != self.field[(column + offset) as usize][(row + offset) as usize]
                {
                    right_diagonal_check = false;
                }
                if self.field[column as usize][row as usize] != self.field[(column - offset) as usize][(row + offset) as usize]
                {
                    left_diagonal_check = false;
                }
            };
            if column_check{
                (true, Some(MoveType::Vertical))
            }
            else if row_check{
                (true, Some(MoveType::Horizontal))
            }
            else if right_diagonal_check{
                (true, Some(MoveType::RightDiagonal))
            }
            else if left_diagonal_check{
                (true, Some(MoveType::LeftDiagonal))
            }
            else{
                (false, None)
            }
        }
    }
}
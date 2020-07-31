pub mod game{

    /*pub const PLAYER_ONE_VALUE: u8 = 1;
    pub const COMPUTER_VALUE: u8 = 2;

    pub const PLAYER_ONE_SYMBOL: char = '+';
    pub const COMPUTER_SYMBOL: char = '-';*/


    use std::collections::HashMap;

    pub const EMPTY_PLACE_VALUE: u8 = 0;
    pub const EMPTY_PLACE_SYMBOL: char = '_';

    pub const WINNING_LENGTH: u8 = 4;

    pub const GAME_BOARD_SIZE: (usize, usize) = (7, 6); //(cols, rows)

    pub trait Player{
        fn init(board_value: u8, board_symbol: char) -> Self;
        fn play(&self: Self, game_state: Option<GameState>) -> u8;

        fn get_board_value(&self) -> u8;
        fn get_board_symbol(&self) -> char;
    }

    pub struct GameState {
        field: [[u8; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
        last_filled_for_column: [u8; GAME_BOARD_SIZE.0],
        values_players: HashMap<u8, dyn Player>
    }

    pub enum MoveType{
        Horizontal,
        Vertical,
        LeftDiagonal,
        RightDiagonal
    }

    impl GameState{
        pub fn init() -> GameState{
            //TODO: Refactor it
            let temp = GameState{
                field: [[EMPTY_PLACE_VALUE; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
                last_filled_for_column: [0;GAME_BOARD_SIZE.0],
                values_players: HashMap(),
            };
            temp
        }

        pub fn place_on_board(&mut self, column: u8, played_id: usize)
        {
            let row = self.last_filled_for_column[column as usize] + 1;
            self.field[column as usize][row as usize] = self.players[played_id].get_board_value();
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
                        None() => print!("{} ", EMPTY_PLACE_SYMBOL)
                    }
                }
                print!("\n");
            }
        }

        pub fn turn(&mut self){
            for player in self.values_players.values().enumerate(){
                self.draw_board();
                self.place_on_board(player.1.play(), player.0);
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
pub mod game{
    use std::io::{self, Read};

    pub const PLAYER_ONE_VALUE: u8 = 1;
    pub const COMPUTER_VALUE: u8 = 2;
    pub const EMPTY_PLACE_VALUE: u8 = 0;

    pub const PLAYER_ONE_SYMBOL: char = '+';
    pub const COMPUTER_SYMBOL: char = '-';
    pub const EMPTY_PLACE_SYMBOL: char = '_';

    pub const WINNING_LENGTH: u8 = 4;

    pub const GAME_BOARD_SIZE: (usize, usize) = (7, 6); //(cols, rows)

    pub trait Player{
        fn init() -> Self;
        fn play(&self: Self, game_state: Option<GameState>) -> u8;
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

    pub struct GameState {
        field: [[u8; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
        last_filled_for_column: [u8; GAME_BOARD_SIZE.0],
        players: Vec<dyn Player>
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
                last_filled_for_column: [0;GAME_BOARD_SIZE.0]
            };
            temp
        }

        pub fn player_play(&mut self, column: u8)
        {
            let row = self.last_filled_for_column[column as usize] + 1;
            self.field[column as usize][row as usize] = PLAYER_ONE_VALUE;

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
                    match *cell{
                        EMPTY_PLACE_VALUE => print!("{} ", EMPTY_PLACE_SYMBOL),
                        PLAYER_ONE_VALUE => print!("{} ", PLAYER_ONE_SYMBOL),
                        COMPUTER_VALUE => print!("{} ", COMPUTER_SYMBOL),
                        _ => panic!("Invalid value"),
                    };
                }
                print!("\n");
            }
        }

        pub fn computer_play(&mut self)
        {

        }

        pub fn turn(&mut self){
            self.draw_board();

            println!("Please enter a column number (1-{}): ", GAME_BOARD_SIZE.0);

            let user_input: u8 = read_from_keyboard();
            self.player_play(user_input);
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
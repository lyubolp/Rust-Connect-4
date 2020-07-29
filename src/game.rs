mod game{
    use std::io::{self, Read};

    pub static PLAYER_ONE_VALUE: u8 = 1;
    pub static COMPUTER_VALUE: u8 = 2;
    pub static EMPTY_PLACE_VALUE: u8 = 0;

    pub static PLAYER_ONE_SYMBOL: char = '+';
    pub static COMPUTER_SYMBOL: char = '-';
    pub static EMPTY_PLACE_SYMBOL: char = '_';

    pub fn read_from_keyboard() -> u8
    {
        let default_value: u8 = 0;
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer){
            Ok(n) =>{
                match buffer.parse::<u8>(){
                    Ok(data) => data,
                    Err(err) => {
                        println!("Error: {}\n Setting it to default value {}", err, default_value)
                    }
                }
            }
            Err(error) => {
                println!("error: {}\n Setting it to default value: {}", error, default_value);
                default_valuelet
            }
        }
    }

    pub struct GameState {
        field: [[u8; 6]; 7],
        last_filled_for_column: [u8; 7]
    }

    impl GameState{
        pub fn init() -> GameState{
            let temp = GameState{
                field: [[EMPTY_PLACE_VALUE; 6]; 7],
                last_filled_for_column: [0;7]
            };
            temp
        }

        pub fn player_play(&mut self, column: u8)
        {
            let row = self.last_filled_for_column[column] + 1;
            self.field[column][row] = PLAYER_ONE_VALUE;

            self.last_filled_for_column[column] += 1;
        }

        pub fn computer_play(&mut self)
        {

        }

        fn draw_board(&self)
        {
            for column_name in 1..7{
                print("{} ", column_name);
            }
            println!("\n");
            for column in self.field.iter(){
                for cell in column.iter(){
                    match *cell{
                        EMPTY_PLACE_VALUE => print!("{}", EMPTY_PLACE_SYMBOL),
                        PLAYER_ONE_VALUE => print!("{}", PLAYER_ONE_SYMBOL),
                        COMPUTER_VALUE => print!("{}", COMPUTER_VALUE),
                        _ => panic!("Invalid value"),
                    };
                }
                println!("\n");
            }
        }

        pub fn turn(&mut self){
            self.draw_board();

            println!("Please enter a column number (1-7): ");

            let user_input: u8 = read_from_keyboard();
            self.player_play(user_input);


        }
    }
}
pub mod game {
    use std::collections::HashMap;

    use crate::human_player::human_player::HumanPlayer;
    use crate::bot::bot::Bot;
    use std::cmp::max;
    use std::env::current_exe;

    pub const EMPTY_PLACE_VALUE: u8 = 0;
    pub const EMPTY_PLACE_SYMBOL: char = '_';

    pub const WINNING_LENGTH: u8 = 4;

    pub const GAME_BOARD_SIZE: (usize, usize) = (6, 7); //(rows, cols)


    #[derive(Copy, Clone)]
    pub enum PlayerType{
        Human,
        Bot
    }
    pub trait Player {
        fn play(&self, game_state: &GameState) -> u8;

        fn get_board_value(&self) -> u8;
        fn get_board_symbol(&self) -> char;

        fn get_type(&self) -> PlayerType;
    }

    pub struct GameState {
        field: [[u8; GAME_BOARD_SIZE.1]; GAME_BOARD_SIZE.0],
        last_filled_for_column: [u8; GAME_BOARD_SIZE.1],
        values_players: HashMap<u8, Box<dyn Player>>,
        is_game_playing: bool,
    }

    #[derive(Copy, Clone)]
    pub enum MoveType {
        Horizontal,
        Vertical,
        LeftDiagonal,
        RightDiagonal,
    }

    impl Clone for GameState{
        fn clone(&self) -> Self {
            GameState{
                field: self.field.clone(),
                last_filled_for_column: self.last_filled_for_column.clone(),
                values_players: {
                    let mut temp_map: HashMap<u8, Box<dyn Player>> = HashMap::new();
                    for (k,v) in self.values_players.iter(){
                        match v.get_type(){
                            PlayerType::Human => temp_map.insert(*k, Box::new(HumanPlayer::new(v.get_board_value(), v.get_board_symbol()))),
                            PlayerType::Bot => temp_map.insert(*k, Box::new(Bot::new(v.get_board_value(), v.get_board_symbol(), 1)))
                        };
                    };
                    temp_map
                },
                is_game_playing: self.is_game_playing.clone()
            }
        }

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
        pub fn get_next_player_to_move_id(&self) -> u8{
            let mut placed = 0;
            for row in self.field.iter(){
                for cell in row.iter(){
                    if *cell != EMPTY_PLACE_VALUE{
                        placed+= 1;
                    }
                }
            }
            placed % self.values_players.len() as u8
        }

        //Public interactions
        pub fn create_and_add_player(&mut self, symbol: char) {
            let temp_value = (self.values_players.len() + 1) as u8;
            let temp_player = HumanPlayer::new(temp_value, symbol);
            self.values_players.insert(temp_value, Box::new(temp_player));
        }
        pub fn create_and_add_bot(&mut self, symbol: char, level: u8){
            let temp_value = (self.values_players.len() + 1) as u8;
            let temp_player: Bot = Bot::new(temp_value, symbol, level);
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
                            if self.check_for_win().0{
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
        pub fn check_for_win(&self) -> (bool, Option<MoveType>) {
            for row_iter_index in 0..GAME_BOARD_SIZE.0 {
                for column_index in 0..GAME_BOARD_SIZE.1 {
                    let row_index = GAME_BOARD_SIZE.0 - row_iter_index - 1;
                    if self.field[row_index][column_index] != self::EMPTY_PLACE_VALUE{
                        let winning_move_tuple: (bool, Option<MoveType>) = self.is_there_winning_move_from((row_index, column_index));
                        if self.field[row_index][column_index] != EMPTY_PLACE_VALUE &&
                            winning_move_tuple.0 {
                            return winning_move_tuple;
                        }
                    }
                }
            }
            (false, None)
        }

        pub fn get_max_connected_from(& self, row: usize, column: usize) -> (u8, MoveType){
            let mut current_vertical: (u8, MoveType) = (1, MoveType::Vertical);
            let mut current_horizontal: (u8, MoveType) = (1, MoveType::Horizontal);
            let mut current_right_diagonal: (u8, MoveType) = (1, MoveType::RightDiagonal);
            let mut current_left_diagonal: (u8, MoveType) = (1, MoveType::LeftDiagonal);

            let mut holder: Vec<(u8, MoveType)> = vec!(current_vertical, current_horizontal, current_left_diagonal, current_right_diagonal);

            for offset in 1..WINNING_LENGTH as usize{
                if column + offset < GAME_BOARD_SIZE.1 && self.field[row][column] == self.field[row][column + offset] {
                    holder[0].0 += 1;
                }
                if offset <= row && self.field[row][column] == self.field[row - offset][column] {
                    holder[1].0 += 1;
                }
                if offset <= row && column + offset < GAME_BOARD_SIZE.1 && self.field[row][column] == self.field[row - offset][column + offset] {
                    holder[2].0 += 1;
                }
                if offset <= row && offset <= column && self.field[row][column] == self.field[row - offset][column - offset] {
                    holder[3].0 += 1;
                }
            };
            
            let mut min = holder[0].clone();
            for win in holder{
                if win.0 > min.0{
                    min = win;
                }
            }
            (min.0, match min.1{
                MoveType::Horizontal => MoveType::Horizontal,
                MoveType::Vertical => MoveType::Vertical,
                MoveType::LeftDiagonal => MoveType::LeftDiagonal,
                MoveType::RightDiagonal => MoveType::RightDiagonal
            })
        }
        pub fn get_max_connected_for_player(&self, player_id: u8) -> (u8, MoveType){
            let mut max: (u8, MoveType) = (0,MoveType::Horizontal);
            for row in self.field.iter().enumerate(){
                for cell in row.1.iter().enumerate(){
                    if *cell.1 == player_id{
                        let current = self.get_max_connected_from(row.0, cell.0);
                        if current.0 > max.0{
                            max = current;
                        }
                    }
                }
            };
            max
        }
        pub fn is_there_winning_move_from(&self, (row, column): (usize, usize)) -> (bool, Option<MoveType>) {
            let is_vertical_win_possible: bool = column <= (GAME_BOARD_SIZE.1 - WINNING_LENGTH as usize);
            let is_horizontal_win_possible: bool = row > (GAME_BOARD_SIZE.0 - WINNING_LENGTH as usize);
            let is_right_diagonal_win_possible: bool = is_vertical_win_possible && is_horizontal_win_possible;
            let is_left_diagonal_win_possible: bool = WINNING_LENGTH as usize <= column && is_horizontal_win_possible;

            if is_vertical_win_possible || is_horizontal_win_possible || is_right_diagonal_win_possible || is_left_diagonal_win_possible {
                let max_connected_from: (u8, MoveType) = self.get_max_connected_from(row, column);
                if max_connected_from.0 == WINNING_LENGTH {
                    (true, Some(max_connected_from.1))
                }
                else{
                    (false, None)
                }
            }
            else{
                (false, None)
            }
        }
        pub fn get_human_players_ids(&self) -> Vec<u8>{
            let mut result: Vec<u8> = Vec::new();
            for player in self.values_players.iter() {
                match player.1.get_type(){
                    PlayerType::Human => result.push(player.0.clone()),
                    _ => {}
                }
            }
            result
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

        #[test]
        fn test_init(){
            let gs = GameState::init();
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
        fn test_check_for_horizontal_win(){
            let mut gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            //Horizontal win
            gs.place_on_board(1, 1);
            assert_eq!(gs.check_for_win().0, false);

            gs.place_on_board(1, 1);
            gs.place_on_board(1, 1);
            gs.place_on_board(1, 1);

            assert_eq!(gs.check_for_win().0, true);
        }

        #[test]
        fn test_check_for_vertical_win(){
            let mut gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            gs.place_on_board(1, 1);
            assert_eq!(gs.check_for_win().0, false);

            gs.place_on_board(2,1);
            gs.place_on_board(3,1);
            gs.place_on_board(4,1);

            assert_eq!(gs.check_for_win().0, true);

        }

        #[test]
        fn test_check_for_right_diagonal_win(){
            let mut gs = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            gs.place_on_board(1, 1);
            assert_eq!(gs.check_for_win().0, false);

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

            assert_eq!(gs.check_for_win().0, true);
        }

        #[test]
        fn test_check_for_left_diagonal_win(){
            let mut gs: GameState = GameState::init();

            gs.create_and_add_player('+');
            gs.create_and_add_player('O');

            gs.place_on_board(5, 1);
            assert_eq!(gs.check_for_win().0, false);
            gs.place_on_board(4, 2);

            gs.place_on_board(4, 1);
            gs.place_on_board(3, 2);

            gs.place_on_board(2, 1);
            gs.place_on_board(3, 2);

            gs.place_on_board(3, 1);
            gs.place_on_board(2, 2);

            gs.place_on_board(2, 1);
            gs.place_on_board(3, 2);

            gs.place_on_board(2, 1);

            assert_eq!(gs.check_for_win().0, true);
        }

        #[test]
        fn test_check_for_connected_number(){
            let mut gs: GameState = GameState::init();

            gs.create_and_add_player('+');

            gs.place_on_board(1, 1);
            assert_eq!(gs.get_max_connected_from(5, 0).0, 1);

            gs.place_on_board(1, 1);
            assert_eq!(gs.get_max_connected_from(5, 0).0, 2);

            gs.place_on_board(1, 1);
            assert_eq!(gs.get_max_connected_from(5, 0).0, 3);
        }
    }
}

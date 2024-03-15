// Create a battleship game in rust

// 1. implement human vs. computer 
// 2. implement human vs human

// 2 players
// each has a 2D array representing their board
// ^ is a ship * is a hit and 0 is empty water
// player wins when all ships are destroyed
    // have a parrell array that contains total ship pieces and when it hits taht number the other player wins

// player struct containing all language

// 
use std::io;
use rand::Rng;

const SIZE: usize = 7;

struct GameBoard {
    board: [[char; SIZE]; SIZE],
}

struct Player {
    board: GameBoard,
    identifier: u64,
    hits: u64,
    misses: u64,
    casualties: u64,
    winner: bool,
    human: bool,
    turn: bool
}

struct BattleshipGame {
    player1: Player,
    player2: Player
}


// impl keyword used to define methods associated with the struct that contains a constructor
impl GameBoard {
    fn new() -> Self {
        GameBoard {
            board: [['0'; SIZE]; SIZE], // Initialize the board with value '0'
        }
    }

    fn place_ships(&mut self) {
        let ship_sizes = [3, 3, 2, 4]; // Ship sizes

        let mut rng = rand::thread_rng();

        for &size in ship_sizes.iter() {
            loop {
                let row = rng.gen_range(0..SIZE);
                let col = rng.gen_range(0..SIZE);
                let dir = rng.gen_range(0..2); // 0 for horizontal, 1 for vertical

                if self.can_place_ship(row, col, size, dir) {
                    self.place_ship(row, col, size, dir);
                    break;
                }
            }
        }
    }

    fn can_place_ship(&self, row: usize, col: usize, size: usize, dir: usize) -> bool {
        // Simplified check; enhance for full collision detection
        for i in 0..size {
            let (r, c) = if dir == 0 { (row, col + i) } else { (row + i, col) };
            
            if r >= SIZE || c >= SIZE || self.board[r][c] != '0' {
                return false; // Out of bounds or collision
            }
        }
        true
    }

    fn place_ship(&mut self, row: usize, col: usize, size: usize, dir: usize) {
        for i in 0..size {
            let (r, c) = if dir == 0 { (row, col + i) } else { (row + i, col) };
            self.board[r][c] = '^'; // Mark as ship
        }
    }

    fn display_board(&self) {
        // Display column headers
        print!("   "); // Spacing for row labels
        for n in 1..=SIZE {
            print!("{} ", n);
        }
        print!("\n   ");
        for n in 1..=(SIZE*2 - 1) {
            print!("-");
        }
        println!();

        // ASCII value for 'A'
        let mut row_label = 'A' as u8;

        for row in self.board.iter() {
            // Convert ASCII to char and print as row label
            print!("{}| ", row_label as char);
            row_label += 1; // Move to the next letter

            // Print each cell in the row
            for &cell in row.iter() {
                print!("{} ", cell);
            }
            println!(); // Newline after each row
        }
    }
}

impl BattleshipGame {
    fn new() -> Self {
        BattleshipGame {
            player1: Player {
                board: GameBoard { board: [['0'; SIZE]; SIZE] },
                identifier: 1,
                hits: 0,
                misses: 0,
                casualties: 0,
                winner: false,
                human: true,
                turn: false
            },        
            player2: Player {
                board: GameBoard { board: [['0'; SIZE]; SIZE] },
                identifier: 2,
                hits: 0,
                misses: 0,
                casualties: 0,
                winner: false,
                human: false,
                turn: false
            },
        }
    }

    fn initialize_boards(&mut self) {
        self.player1.board.place_ships();
        self.player2.board.place_ships();
    }

    fn human_game(&self) {
        println!("human game");
    }

    fn execute_turn(&mut self) {
        if self.player1.turn == true {
            println!("Your turn");
        }
        else if self.player2.turn == true {
            // TODO need to change when human_game calls this function
            println!("Computer's turn");
        } else{
            println!("\n\nError: No turn recgonized");
        }

    }

    fn computer_game(&mut self) {
        println!("computer game");
        println!("There will be random selection to decide which player goes first.");

        let mut rng = rand::thread_rng();
        let decicion = rng.gen_range(1..=2);

        println!("{}", decicion);
        if decicion == 1  {
            self.player1.turn = true;
        } else {
            self.player2.turn = true;
        }

        // only perform this once outside of loop
        self.initialize_boards();

        // iterate through turns
        self.execute_turn();        

        // add loop to iterate through turns
        // loop { true } {
        //     break;
        // }

    }

    fn load_screen(&mut self) {
        // let val: i8;
        loop {
            let mut input = String::new();

            println!("\n\n****************************************************\n\n\tWelcome to Battle Ship\n
            __   ____
            __|_|__|_|__
        _|____________|__
        |o o o o o o o o /
    ~'`~'`~'`~'`~'`~'`~'`~    \n\n\tSelect an option below.\n\n****************************************************\n\n1. vs Computer\n2. vs Human");
            io::stdin().read_line(&mut input).expect("Failed to read line");

            let input: i32 = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please enter a valid number.");
                    continue; // If the input is not a valid number, start the loop over
                },
            };

            match input {
                1 => {
                    break;
                },
                2 => {
                    self.player2.human = true;
                    break;
                },
                _ => println!("Try again. Please enter 1 or 2."),
            }
        }
    }
}

fn main() {

    let mut game = BattleshipGame::new(); // state may need to change when winner is found; or change check player struct
    game.load_screen();

    if game.player2.human {
        game.human_game();
    } else {
        game.computer_game();
    }
}

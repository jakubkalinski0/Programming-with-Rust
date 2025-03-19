use std::io;

// Define an enumeration for the possible states of a board field
#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    Empty, // Represents an empty field, meaning no move has been made
    X,     // Represents a field occupied by player X
    O,     // Represents a field occupied by player O
}

// Define a struct for the Tic-Tac-Toe board
struct Board {
    fields: [Field; 9], // A fixed-size array representing the board fields (3x3 grid)
}

impl Board {
    // Constructor to initialize a new empty board
    fn new() -> Board {
        Board { fields: [Field::Empty; 9] } // All fields are initially empty
    }

    // Check if a specific board field is empty
    fn is_field_empty(&self, position: usize) -> bool {
        self.fields[position] == Field::Empty // Returns true if the field is empty
    }

    // Make a move on the board if the field is empty
    fn make_move(&mut self, position: usize, player: Field) -> bool {
        if !self.is_field_empty(position) {
            return false; // Move is not valid since the field is already occupied
        }
        self.fields[position] = player; // Assign the field to the current player
        true // Move was successful
    }

    // Check if the board is full (i.e., no empty fields left)
    fn is_full(&self) -> bool {
        for i in 0..9 {
            if self.fields[i] == Field::Empty {
                return false; // At least one empty field remains
            }
        }
        true // No empty fields left, meaning the board is full
    }

    // Check if there is a winner and return the winning player if found
    fn check_winner(&self) -> Option<Field> {
        // Check rows for a win (three consecutive marks in a row)
        for i in 0..3 {
            let row_start = i * 3;
            if self.fields[row_start] != Field::Empty
                && self.fields[row_start] == self.fields[row_start + 1]
                && self.fields[row_start] == self.fields[row_start + 2]
            {
                return Some(self.fields[row_start]); // Return the winning player
            }
        }

        // Check columns for a win (three consecutive marks in a column)
        for column_start in 0..3 {
            if self.fields[column_start] != Field::Empty
                && self.fields[column_start] == self.fields[column_start + 3]
                && self.fields[column_start] == self.fields[column_start + 6]
            {
                return Some(self.fields[column_start]); // Return the winning player
            }
        }

        // Check diagonal from top-left to bottom-right
        if self.fields[0] != Field::Empty && self.fields[0] == self.fields[4] && self.fields[0] == self.fields[8] {
            return Some(self.fields[0]); // Return the winning player
        }

        // Check diagonal from top-right to bottom-left
        if self.fields[2] != Field::Empty && self.fields[2] == self.fields[4] && self.fields[2] == self.fields[6] {
            return Some(self.fields[2]); // Return the winning player
        }

        None // No winner found
    }

    // Display the board in a human-readable format
    fn display(&self) {
        println!("Board:");
        for i in 0..3 {
            for j in 0..3 {
                let index = i * 3 + j; // Calculate the current index in the 1D array
                match self.fields[index] {
                    Field::Empty => print!("{} ", index + 1), // Show the field number if it's empty
                    Field::X => print!("X "), // Show X for player X
                    Field::O => print!("O "), // Show O for player O
                }
                if j < 2 {
                    print!("| "); // Column separator for better readability
                }
            }
            println!();
            if i < 2 {
                println!("---------"); // Row separator for better visibility
            }
        }
    }
}

fn main() {
    let mut board = Board::new(); // Initialize a new game board
    let mut current_player = Field::X; // Set the starting player as X
    let mut game_over = false; // Track whether the game has ended

    println!("Welcome to Tic-Tac-Toe!");
    board.display(); // Display the initial empty board

    while !game_over {
        // Determine the current player's name for display
        let player_name = match current_player {
            Field::X => "Player 1 (X)",
            Field::O => "Player 2 (O)",
            _ => unreachable!(),
        };

        // Loop until the player makes a valid move
        let position = loop {
            println!("{}, your turn (enter a field number from 1 to 9):", player_name);

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Error reading input");

            // Parse user input and validate the move
            match user_input.trim().parse::<usize>() {
                Ok(pos) if pos >= 1 && pos <= 9 => {
                    let index = pos - 1; // Convert user input (1-9) to array index (0-8)
                    if board.is_field_empty(index) {
                        break index; // Valid move, exit loop
                    } else {
                        println!("Field is already occupied! Choose another field.");
                    }
                }
                _ => println!("Invalid field number. Enter a number from 1 to 9."),
            }
        };

        // Apply the move and update the board
        board.make_move(position, current_player);
        board.display(); // Show the updated board

        // Check if there is a winner after the move
        if let Some(winner) = board.check_winner() {
            match winner {
                Field::X => println!("Player 1 (X) wins!"),
                Field::O => println!("Player 2 (O) wins!"),
                _ => unreachable!(),
            }
            game_over = true;
        } else if board.is_full() {
            println!("It's a draw! The board is full."); // The game ends in a draw
            game_over = true;
        }

        // Switch to the next player for the next round
        current_player = match current_player {
            Field::X => Field::O,
            Field::O => Field::X,
            _ => unreachable!(),
        };
    }

    println!("Game over! Thanks for playing."); // Final message after the game ends
}
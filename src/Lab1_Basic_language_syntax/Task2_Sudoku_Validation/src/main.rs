/// Function to check if a given Sudoku board is valid.
/// The function verifies that each row, each column, and each 3x3 subgrid
/// contains only unique digits from 1 to 9 (or 0 for empty spaces).
fn check_sudoku_board(board: [[u8; 9]; 9]) -> bool {
    // Check all rows
    for row in &board {
        if !check_segment(row) {
            return false; // If any row is invalid, return false immediately
        }
    }

    // Check all columns
    for col in 0..9 {
        let mut column = [0; 9];
        for row in 0..9 {
            column[row] = board[row][col];
        }
        if !check_segment(&column) {
            return false; // If any column is invalid, return false
        }
    }

    // Check all 3x3 subgrids
    for starting_row in 0..3 {
        for starting_column in 0..3 {
            let mut square = [0; 9];
            let mut index = 0;
            for row in 0..3 {
                for column in 0..3 {
                    square[index] = board[starting_row * 3 + row][starting_column * 3 + column];
                    index += 1;
                }
            }
            if !check_segment(&square) {
                return false; // If any 3x3 subgrid is invalid, return false
            }
        }
    }

    true // If all checks pass, the board is valid
}

/// Function to check whether a given segment (row, column, or subgrid) contains unique numbers
fn check_segment(segment: &[u8; 9]) -> bool {
    let mut used_digits = [false; 9]; // Array to track used numbers
    for &value in segment {
        if value > 9 {
            return false; // Sudoku numbers must be in range 1-9
        }
        if value == 0 {
            continue; // Empty cells are allowed (0 represents an empty cell)
        }
        if used_digits[value as usize - 1] {
            return false; // Duplicate found, segment is invalid
        } else {
            used_digits[value as usize - 1] = true; // Mark digit as used
        }
    }
    true // The segment is valid
}

fn main() {
    // Example Sudoku board with some empty cells (0s)
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9]
    ];

    let result = check_sudoku_board(board);

    if result {
        println!("The Sudoku board is valid!");
    } else {
        println!("The Sudoku board contains errors!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test with a valid but incomplete Sudoku board
    #[test]
    fn test_valid_incomplete_board() {
        let board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(check_sudoku_board(board));
    }

    /// Test with an invalid row (duplicate number in the same row)
    #[test]
    fn test_invalid_row() {
        let mut board = [[0; 9]; 9];
        board[0][0] = 5;
        board[0][1] = 5; // Duplicate value in the same row
        assert!(!check_sudoku_board(board));
    }

    /// Test with an invalid column (duplicate number in the same column)
    #[test]
    fn test_invalid_column() {
        let mut board = [[0; 9]; 9];
        board[0][0] = 3;
        board[1][0] = 3; // Duplicate value in the same column
        assert!(!check_sudoku_board(board));
    }

    /// Test with an invalid 3x3 subgrid (duplicate number in the same subgrid)
    #[test]
    fn test_invalid_square() {
        let mut board = [[0; 9]; 9];
        board[0][0] = 7;
        board[1][1] = 7; // Duplicate value in the same subgrid
        assert!(!check_sudoku_board(board));
    }

    /// Test with an invalid value outside the allowed range (1-9)
    #[test]
    fn test_invalid_value() {
        let mut board = [[0; 9]; 9];
        board[0][0] = 10; // Invalid value
        assert!(!check_sudoku_board(board));
    }
}

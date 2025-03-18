fn check_sudoku_board(board: [[u8; 9]; 9]) -> bool {

    for row in &board {
        if check_segment(row) == false {
            return false;
        }
    }

    for col in 0..9 {
        let mut column = [0;9];
        for row in 0..9 {
            column[row] = board[row][col];
        }
        if check_segment(&column) == false {
            return false;
        }
    }

    for starting_row in 0..3 {
        for starting_column in 0..3 {
            let mut square = [0;9];
            let mut index = 0;
            for row in 0..3 {
                for column in 0..3 {
                    square[index] = board[starting_row*3 + row][starting_column*3 + column];
                    index += 1;
                }
            }
            if check_segment(&square) == false {
                return false;
            }
        }
    }
    true
}


fn check_segment(segment: &[u8; 9]) -> bool {
    let mut used_digits = [false; 9];
    for &value in segment {
        if value > 9 {
            return false;
        }

        if value == 0 {
            continue;
        }

        if used_digits[value as usize - 1] == true {
            return false;
        }
        else {
            used_digits[value as usize - 1] = true
        }
    }
    true
}

fn main() {
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
        println!("Plansza Sudoku jest poprawna!");
    } else {
        println!("Plansza Sudoku zawiera błędy!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_invalid_row() {
        let mut board = [
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

        board[0][2] = 5;

        assert!(!check_sudoku_board(board));
    }

    #[test]
    fn test_invalid_column() {
        let mut board = [
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

        board[2][0] = 5;

        assert!(!check_sudoku_board(board));
    }

    #[test]
    fn test_invalid_square() {
        let mut board = [
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

        board[2][2] = 5;

        assert!(!check_sudoku_board(board));
    }

    #[test]
    fn test_invalid_value() {
        let mut board = [
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

        board[0][0] = 10;

        assert!(!check_sudoku_board(board));
    }
}
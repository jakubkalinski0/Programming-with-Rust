use std::io;


#[derive(Clone, Copy, PartialEq, Debug)]
enum Field {
    Empty,
    X,
    O,
}


struct Board{
    fields: [Field; 9],
}


impl Board{
    fn new() -> Board{
        Board{fields: [Field::Empty; 9]}
    }

    fn is_field_empty(&self, position: usize)-> bool{
        self.fields[position] == Field::Empty
    }

    fn make_move(&mut self, position: usize, player: Field) -> bool {
        if !self.is_field_empty(position) {
            return false;
        }
        self.fields[position] = player;
        true
    }

    fn is_full(&self)-> bool {
        for i in 0..9{
            if self.fields[i] == Field::Empty {
                return false;
            }
        }
        true
    }

    fn check_winner(&self)-> Option<Field> {
        for i in 0..3{
            let row_start = i*3;
            if self.fields[row_start] != Field::Empty
                && self.fields[row_start] == self.fields[row_start+1]
                && self.fields[row_start] == self.fields[row_start+2]
            {
                return Some(self.fields[row_start]);
            }
        }

        for column_start in 0..3{
            if self.fields[column_start] != Field::Empty
                && self.fields[column_start] == self.fields[column_start+3]
                && self.fields[column_start] == self.fields[column_start+6]
            {
                return Some(self.fields[column_start]);
            }
        }

        if self.fields[0] != Field::Empty && self.fields[0] == self.fields[4] && self.fields[0] == self.fields[8] {
            return Some(self.fields[0]);
        }

        if self.fields[2] != Field::Empty && self.fields[2] == self.fields[4] && self.fields[2] == self.fields[6] {
            return Some(self.fields[2]);
        }

        None
    }

    fn display(&self) {
        println!("Plansza:");
        for i in 0..3 {
            for j in 0..3 {
                let index = i * 3 + j;
                match self.fields[index] {
                    Field::Empty => print!("{} ", index + 1),
                    Field::X => print!("X "),
                    Field::O => print!("O "),
                }
                if j < 2 {
                    print!("| ");
                }
            }
            println!();
            if i < 2 {
                println!("---------");
            }
        }
    }
}


fn main() {
    let mut board = Board::new();
    let mut current_player = Field::X;
    let mut game_over = false;

    println!("Witaj w grze Kółko i Krzyżyk!");
    board.display();

    while !game_over {
        let player_name = match current_player {
            Field::X => "Gracz 1 (X)",
            Field::O => "Gracz 2 (O)",
            _ => unreachable!(),
        };

        let position = loop {
            println!("{}, Twój ruch (wprowadź numer pola od 1 do 9):", player_name);

            let mut user_input = String::new();
            io::stdin().read_line(&mut user_input).expect("Błąd odczytu wprowadzonych danych");

            match user_input.trim().parse::<usize>() {
                Ok(pos) if pos >= 1 && pos <= 9 => {
                    let index = pos - 1;
                    if board.is_field_empty(index) {
                        break index;
                    } else {
                        println!("Pole jest już zajęte! Wybierz inne pole.");
                    }
                }
                _ => println!("Nieprawidłowy numer pola. Wprowadź liczbę od 1 do 9."),
            }
        };

        board.make_move(position, current_player);
        board.display();

        if let Some(winner) = board.check_winner() {
            match winner {
                Field::X => println!("Gracz 1 (X) wygrał!"),
                Field::O => println!("Gracz 2 (O) wygrał!"),
                _ => unreachable!(),
            }
            game_over = true;
        } else if board.is_full() {
            println!("Remis! Plansza jest pełna.");
            game_over = true;
        }

        current_player = match current_player {
            Field::X => Field::O,
            Field::O => Field::X,
            _ => unreachable!(),
        };
    }

    println!("Koniec gry! Dziękujemy za grę.");
}
use std::fmt;
use std::str::FromStr;
use std::result::Result;
use std::error::Error;

#[derive(Debug)]
pub struct CommandParseError;
impl Error for CommandParseError{}
impl fmt::Display for CommandParseError{
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        write!(f, "Fehler bei der Verarbeitung der Usereingabe")
    }
}

#[derive(Eq, PartialEq)]
enum Cell{
    LEER,
    X,
    O
}

impl fmt::Display for Cell{
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::LEER => write!(f, "░"),
            Cell::X => write!(f, "X"),
            Cell::O => write!(f, "O")
        }    
    }
}
#[derive(Debug)]


pub enum Player{
    P1,
    P2,
    TBD
}
impl fmt::Display for Player{
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::P1 => write!(f, "P1"),
            Player::P2 => write!(f, "P2"),
            Player::TBD => write!(f, "TBD")
        }    
    }
}
#[derive(Debug)]
pub struct CellCoord(u8,u8);

pub struct GameState{
    board:[Cell; 9],
    finished:bool,
    winner:Player
}
impl GameState{
    pub fn new()->GameState{
         GameState{
                        board:
                        [Cell::LEER, Cell::LEER, Cell::LEER,
                         Cell::LEER, Cell::LEER, Cell::LEER,
                          Cell::LEER, Cell::LEER, Cell::LEER],
                        finished:false,
                        winner:Player::TBD}
    }
    fn step(& mut self, attempt :CellCoord){
        let (x, y) = (attempt.0, attempt.1);
        let array_coord:usize = (3*(y-1)+(x-1)).try_into().unwrap();
        if self.board[array_coord] == Cell::LEER {
            self.board[array_coord] = Cell::X;
        }
        //Der Gegenspieler macht einen Zug
        self.step_ai();

        //Falls es keine Leeren Felder mehr gibt:
        let mut board_full:bool = true;
        for pos in 1..10 {
            if self.board[pos-1] == Cell::LEER {
                board_full = false;
                break;
            }
        }
        //P2 gewinnt, falls es keine möglichen Züge für P1 (den Spieler) gibt
        if board_full{
            self.fin(Player::P2);
        }
    }
    fn step_ai(& mut self){
        //AI versucht, die Zellen in Reihenfolge auszufüllen 
        for pos in 1..9 {
            if self.board[pos-1]==Cell::LEER {
                self.board[pos-1]=Cell::O;
                break;
            }
        }
    }
    fn fin(& mut self, wins: Player){
        self.finished=true;
        self.winner= wins;
    }

    pub fn process_command(& mut self,c: Command) {
        
        match c {
            Command::CELL(coord) => self.step(coord),
            Command::FINISH => self.fin(Player::P1)
        }

    }
    pub fn is_finished(&mut self)->bool{
        self.finished
    }
}
impl fmt::Display for GameState{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, "{} {} {}\n", &self.board[0], &self.board[1], &self.board[2]);
        write!(f, "{} {} {}\n", &self.board[3], &self.board[4], &self.board[5]);
        write!(f, "{} {} {}\n", &self.board[6], &self.board[7], &self.board[8]);
        if self.finished {

            write!(f,  "GAME OVER! {} hat gewonnen!", &self.winner);
        }
        write!(f,"")
    }
}

pub enum Command{
    CELL(CellCoord),
    FINISH
}
impl FromStr for Command {
    type Err = CommandParseError;
    fn from_str(command_line: &str) -> Result<Self, Self::Err> {
        
        let INVALID_COORD = i8::MIN;

        let res : Result<Self, Self::Err>;

        match command_line.len() {
            2 => {
                let x_int;
                let y_int;
                //Eingaben sind in der Form [X][Y], X ist in Buchstaben [A..C], Y in Zahlen [1..3] gegeben
                let iter:Vec<char> = command_line.chars().collect();
                let x_input = iter[0];
                let y_input = iter[1];
                match x_input {
                    'A' => x_int = 1,
                    'B' => x_int = 2,
                    'C' => x_int = 3,
                    _ => x_int = INVALID_COORD
                }
                match y_input {
                    '1' => y_int = 1,
                    '2' => y_int = 2,
                    '3' => y_int = 3,
                    _ => y_int = INVALID_COORD
                }

                if x_int != INVALID_COORD && y_int != INVALID_COORD {
                    res = Ok(Command::CELL(CellCoord(x_int.try_into().unwrap(), y_int.try_into().unwrap())));
                }else{
                    res = Err(CommandParseError);
                } 
            }
            3 => {
                let string_lowercase = command_line.to_lowercase();
                if string_lowercase == "fin" {
                    res = Ok(Command::FINISH);
                }else {
                    res = Err(CommandParseError);
                }

            },
            _ => res = Err(CommandParseError),
        }

        res
    }
}
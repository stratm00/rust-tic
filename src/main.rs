use std::io::stdin;
use std::result::Result;
use std::str::FromStr;

mod game_core;
use crate::game_core::{Command, GameState};

fn main() {

    println!("***TIC TAC TOE***");
    let mut current : GameState = GameState::new();
    println!("{}", current);
    //R E P L
    loop {

        //Lese Zeile, gebe Output an Fkt. 
        
        let mut buffer = String::new();

        stdin().read_line(& mut buffer);
        //Behandle Mögliche Fehler
        let com = Command::from_str(&buffer.trim());
        match com {
            Result::Ok(command) => current.process_command(command),
            Result::Err(e) => {
                println!("***ERROR:  {}***", e);
            }
        }
        //Stelle Gamestate dar
        println!("{}", current);
        if current.is_finished() { break;}
    }

}

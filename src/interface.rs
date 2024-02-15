use std::io;
use std::fs;
use crate::game_loader;
use crate::graph;
use crate::player_status;
use crate::karmic_catastrophe::KarmicCatastrophe;
use crate::command_line_game_loop;


pub trait Interface {
    fn run(&mut self) -> Result<(), KarmicCatastrophe>;
}

pub struct CommandLineInterface {
    
}

impl CommandLineInterface {
    pub fn new() -> Self {
        CommandLineInterface{}
    }
}


impl Interface for CommandLineInterface {
    fn run(&mut self) -> Result<(), KarmicCatastrophe> {
        let contents= fs::read_to_string("./assets/game.json").expect("Could not read game file.");
        let sakya_pandita = game_loader::build_game_from_json_string(contents.as_str()).expect("Could not parse game JSON string");

        let g = graph::Graph::new(&sakya_pandita);

        loop {
            println!("Welcome to the main menu. Your choices are\nNew,\nLoad,\nQuit.");

            let mut ps = player_status::PlayerStatus::new("Neo", sakya_pandita.starting_square);
            let mut choice = String::new();

            loop {
                choice.clear();
                loop {
                    match io::stdin().read_line(&mut choice) {
                        Ok(_size) => break,
                        Err(_why) => println!("An error impeded the transmission of thought. Please type something, as long as it is different.")
                    }
                }
                
                let true_choice = choice.trim();
                match true_choice {
                    "New" => {break;},
                    "Load" => {ps.data.current_square = 9; break;},
                    "Quit" => {return Ok(());},
                    _ => {continue;}
                }
            }

            println!("Welcome to the game. At any point you may type Quit to exit.");

            let result = command_line_game_loop::game_loop(&mut ps, &sakya_pandita, &g);
            match result {
                Ok(()) => {continue;},
                Err(why) => {return Err(why);}
            }
        }
    }
}

impl CommandLineInterface {

}
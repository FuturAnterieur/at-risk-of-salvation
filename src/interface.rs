use std::io;
use std::fs;
use crate::game_loader;
use crate::graph;
use crate::player_status;
use crate::karmic_catastrophe::KarmicCatastrophe;
use crate::command_line_game_loop;
use serde_json;


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

pub fn menu_for_load_game() -> Option<player_status::PlayerStatusPersistentData> {
    let saved_games_folder = "./assets/saved_games";
    let maybe_saved_games_files = std::fs::read_dir(saved_games_folder);
    match maybe_saved_games_files {
        Ok(saved_games_files) => {
            let i : usize = 1;
            let mut sgf = Vec::<String>::new();
            for file in saved_games_files {
                sgf.push(String::from(file.unwrap().path().to_str().unwrap()));
                println!("{} : {}", i.to_string(), sgf.last().unwrap());
            }

            //let vec : Vec<fs::DirEntry>=  saved_games_files.collect();
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
                if true_choice == "Cancel" {
                    return None;
                }


                let mut chosen_val : usize = 0;
                match true_choice.parse::<usize>() {
                    Ok(val) => {chosen_val = val - 1;},
                    Err(_why) => continue,
                }

                if chosen_val >= sgf.len() {
                    continue;
                }

                let chosen_file = &sgf[chosen_val];

                let maybe_content = fs::read_to_string(format!("{}/{}", saved_games_folder, chosen_file));
                match maybe_content {
                    Err(_why) => continue,
                    Ok(content) => {
                        let maybe_player_data : serde_json::Result<player_status::PlayerStatusPersistentData> = serde_json::from_str(content.as_str());
                        match maybe_player_data {
                            Err(_why) => continue,
                            Ok(player_data) => return Some(player_data)
                        }
                    }
                }
            }
        },
        Err(_why) => None,
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
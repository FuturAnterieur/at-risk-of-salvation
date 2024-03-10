use std::io;
use crate::game_loader;
use crate::graph;
use crate::shortest_path;
use crate::dice_event_parser::int_to_digit_text;
use crate::lalrpop::dice::NumParser;
use crate::karmic_catastrophe::KarmicCatastrophe;
use crate::player_status::{PlayerStatus, RemainingRequirementsForEdge};
use dyn_clone::clone_box;
use serde_json;
use std::fs;

fn show_edges_choices(edges : &Vec::<graph::Edge>) -> String {
    let mut all_distinct_digits : Vec<i16> = edges.iter().map(|edge| edge.requirement.enumerate_roll_values()).flatten().collect();
    all_distinct_digits.sort();
    all_distinct_digits.dedup();
    let all_names : Vec<&str> = all_distinct_digits.iter().map(|value| int_to_digit_text(value.clone()) ).collect();
    all_names.join(", ")
}

fn show_cheat_edge_choices(square : &game_loader::Square, game : &game_loader::Game) -> String {
    let pairs : Vec<(&String, &u32)> = square.paths.iter().collect();
    let mut choices = Vec::<String>::new();
    for pair in pairs {
        let maybe_dest = game.squares.iter().find(|square| &square.number == pair.1);
        match maybe_dest {
            Some(dest) => choices.push(format!("{} : go to {}", pair.0, dest.name)),
            None => choices.push(format!("{} : go to {} (not written yet)", pair.0, pair.1))
        }
    }
    choices.join("\n")
}

fn determine_next_node(ps: &mut PlayerStatus, remaining_reqs_for_each_edge : &mut Vec::<RemainingRequirementsForEdge>, original_edges : &Vec::<graph::Edge>, selected_option : &str) -> Option<u32> {
    let selected_option_value = NumParser::new().parse(selected_option);
    match selected_option_value {
        Err(_why) => None,
        Ok(val) => {
            ps.data.rolls_on_current_square.push(val);
            let maybe_idx = remaining_reqs_for_each_edge.iter_mut().position(|reqs| reqs.remaining.fullfill_with(&val) );
            match maybe_idx {
                Some(idx) => Some(original_edges[idx].destination.clone()),
                None => None,
            }
            
        }
    }
}

pub fn save_player_status_menu(ps : &PlayerStatus) -> Result<(), KarmicCatastrophe>{
    let mut choice = String::new();

    let saved_games_folder = "./assets/saved_games";
    let is_ok = fs::create_dir_all(saved_games_folder);
    match is_ok {
        Err(why) => return Err(KarmicCatastrophe{message: why.to_string()}),
        Ok(()) => ()
    }

    let mut content : String = String::new();
    let maybe_contents = serde_json::to_string_pretty(&ps.data);
    match maybe_contents {
        Err(why) => return Err(KarmicCatastrophe{message: why.to_string()}),
        Ok(contents) => {content = contents;}
    }

    println!("Please type a name for the save game file :");
    
    loop {
        match io::stdin().read_line(&mut choice) {
            Ok(_size) => break,
            Err(_why) => println!("An error impeded the transmission of thought. Please type something, as long as it is different.")
        }
    }

    let full_path = format!("{}/{}.json", saved_games_folder, choice.trim());
    
    match fs::write(full_path, content) {
        Err(why) => Err(KarmicCatastrophe{message : why.to_string()}),
        Ok(()) => Ok(())
    }
    
}


pub fn game_loop(ps : &mut PlayerStatus, sakya_pandita : &game_loader::Game, g : &graph::Graph) -> Result<(), KarmicCatastrophe>{
    
    let mut next_square_num = ps.data.current_square.clone();

    println!("Welcome to the game. At any point you may type Save to save your game, or Quit to exit.");

    loop {
        let current_square_opt = sakya_pandita.squares.iter().find(|square| square.number == next_square_num);
        if current_square_opt.is_none() {
            println!("You landed on square {}, which does not exist, but may be there when I complete the JSON file.", next_square_num);
            println!("Going back to where we were...");
            next_square_num = ps.data.current_square.clone();
            continue;
        }

        ps.data.current_square = next_square_num.clone();
        let current_square = current_square_opt.unwrap();
        println!("-- {} -- {}", &current_square.number, &current_square.name);
        println!("{}", &current_square.description);

        if current_square.number == sakya_pandita.winning_square {
            println!("This is the end of your journey.");
            break;
        }

        let mut predecessors = shortest_path::dijkstra(&g, &current_square.number, &sakya_pandita.winning_square, shortest_path::EdgeDistanceMetric::ExpectedRolls).unwrap().predecessors;
        predecessors.reverse();
        predecessors.push(sakya_pandita.winning_square);
        let chances_of_reaching_end = g.path_probability(&predecessors);
        println!("Your odds of following the shortest path to the end are {:.12}%.", chances_of_reaching_end*100_f64);
        let original_edges = g.edges_for_node(&current_square.number);

        if original_edges.is_none() {
            return Err(KarmicCatastrophe{message:"No edges found for node -- that's a programming error. Aborting.".to_string()});
        }

        let mut remaining_reqs_for_each_edge = Vec::<RemainingRequirementsForEdge>::new();

        for orig_edge in original_edges.unwrap_or(&Vec::<graph::Edge>::new()) {
            remaining_reqs_for_each_edge.push(RemainingRequirementsForEdge{remaining: clone_box(&*orig_edge.requirement)});
        }
        
        for val in &ps.data.rolls_on_current_square {
            remaining_reqs_for_each_edge.iter_mut().position(|reqs| reqs.remaining.fullfill_with(&val) );
        }
        
        let mut choice = String::new();
        loop {
            println!("Your choices are :");
            //println!("{}", show_edges_choices(original_edges.unwrap()));
            println!("{}", show_edges_choices(original_edges.unwrap()));
        
            choice.clear();
            loop {
                match io::stdin().read_line(&mut choice) {
                    Ok(_size) => break,
                    Err(_why) => println!("An error impeded the transmission of thought. Please type something, as long as it is different.")
                }
            }
            
            let true_choice = choice.trim().to_string();
            if true_choice == "Quit" {
                return Ok(());
            }

            if true_choice == "Save" {
                match save_player_status_menu(ps) {
                    Ok(()) => {println!("Game saved successfully"); continue; },
                    Err(_why) => continue,
                }
            }

            let maybe_next_node = determine_next_node(ps, &mut remaining_reqs_for_each_edge, original_edges.unwrap(), &true_choice);
            match maybe_next_node {
                Some(next_node) => {next_square_num = next_node.clone(); ps.data.rolls_on_current_square.clear(); break;},
                None => {choice.clear(); println!("Your voice resounds loudly, but your wishes sound in the void.");}
            }
        }
    }
    Ok(())

}
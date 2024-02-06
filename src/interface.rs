
use std::error::Error;
use std::fs;
use std::io;
use crate::game_loader;
use crate::graph;
use crate::shortest_path;
use crate::dice_event_parser::int_to_digit_text;

pub struct KarmicCatastrophe
{
    message: String
}

impl Error for KarmicCatastrophe {   }

impl std::fmt::Debug for KarmicCatastrophe 
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({})", self.message)
    }
}

impl std::fmt::Display for KarmicCatastrophe
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "({})", self.message)
    }
}

pub trait Interface {
    fn run(&mut self) -> Result<(), KarmicCatastrophe>;
}

pub struct CommandLineInterface {
    
}

impl CommandLineInterface {
    pub fn new() -> Self {
        CommandLineInterface{}
    }

    fn show_edges_choices(&self, edges : &Vec::<graph::Edge>) -> String {
        let mut all_distinct_digits : Vec<i16> = edges.iter().map(|edge| edge.requirement.enumerate_roll_values()).flatten().collect();
        all_distinct_digits.sort();
        all_distinct_digits.dedup();
        let all_names : Vec<&str> = all_distinct_digits.iter().map(|value| int_to_digit_text(value.clone()) ).collect();
        all_names.join(", ")
    }
}


impl Interface for CommandLineInterface {
    fn run(&mut self) -> Result<(), KarmicCatastrophe> {
        let contents= fs::read_to_string("./assets/game.json").expect("Could not read game file.");
        let sakya_pandita = game_loader::build_game_from_json_string(contents.as_str()).expect("Could not parse game JSON string");

        let g = graph::Graph::new(&sakya_pandita);
        
        let mut current_square_num = sakya_pandita.starting_square;
        let mut next_square_num = current_square_num.clone();

        loop {
            let current_square_opt = sakya_pandita.squares.iter().find(|square| square.number == next_square_num);
            if current_square_opt.is_none() {
                println!("You landed on square {}, which does not exist, but may be there when I complete the JSON file.", next_square_num);
                println!("Going back to where we were...");
                next_square_num = current_square_num.clone();
                continue;
            }

            current_square_num = next_square_num.clone();
            let current_square = current_square_opt.unwrap();
            println!("-- {} -- {}", &current_square.number, &current_square.name);
            println!("{}", &current_square.description);

            if current_square.number == sakya_pandita.winning_square {
                println!("This is the end of your journey.");
                break;
            }

            let mut predecessors = shortest_path::dijkstra(&g, &current_square.number, &sakya_pandita.winning_square, shortest_path::EdgeDistanceMetric::ExpectedRolls).unwrap().predecessors;
            predecessors.reverse();
            let chances_of_reaching_end = g.path_probability(&predecessors);
            println!("Your odds of following the shortest path to the end are {:.7}%.", chances_of_reaching_end*100_f64);

            println!("Your choices are :");
            
            let edges = g.edges_for_node(&current_square.number);
            
            println!("{}", self.show_edges_choices(edges.unwrap()));
            let mut choice = String::new();

            match io::stdin()
            .read_line(&mut choice) {
                Ok(_size) => (),
                Err(why) => return Err(KarmicCatastrophe{message : format!("Your answer does not please the emptiness : {}", why)})
            }

            let true_choice = choice.trim().to_string();
            let maybe_next_node = current_square.paths.get(&true_choice);
            match maybe_next_node {
                Some(next_node) => next_square_num = next_node.clone(),
                None => return Err(KarmicCatastrophe{message : "Your choice lies only in nothingness.".to_string()}),
            }
        }
        
        Ok(())
    }
}

impl CommandLineInterface {

}
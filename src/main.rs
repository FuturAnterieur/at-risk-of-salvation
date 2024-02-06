mod game_loader;
mod factorial;
mod dice_event_parser;
mod graph;
mod min_heap;
mod shortest_path;
mod interface;
mod dice_event;
mod tests;

use crate::interface::Interface;

fn main() {
    
    //TODO : Maybe use matches here
    /*let contents= fs::read_to_string("./assets/game.json").expect("Could not read game file.");
    let sakya_pandita = game_loader::build_game_from_json_string(contents.as_str()).expect("Could not parse game JSON string");

    println!("First square data : {}", &sakya_pandita.squares[0].name);
    println!("Second square data : {}", &sakya_pandita.squares[1].number);

    println!("Full second square data : {}", serde_json::to_string(&sakya_pandita.squares[1]).expect("Could not serialize square 1!!!"));*/

    let mut app = interface::CommandLineInterface::new();
    let result = app.run();
    match result {
        Ok(()) => println!("Goodbye!!!"),
        Err(karma) => println!("A catastrophe occurred! \n{}", karma)
    }

}

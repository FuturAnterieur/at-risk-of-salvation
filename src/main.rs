mod game_loader;

use std::fs;

fn main() {
    
    //TODO : Maybe use matches here
    let contents= fs::read_to_string("./assets/game.json").expect("Could not read game file.");
    let sakya_pandita = game_loader::build_game_from_json_string(contents.as_str()).expect("Could not parse game JSON string");

    println!("First square data : {}", &sakya_pandita.squares[0].name);
    println!("Second square data : {}", &sakya_pandita.squares[1].number);

}

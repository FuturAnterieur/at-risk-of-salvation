use crate::game_loader;
use std::collections::LinkedList;
use crate::probability_parser;

pub struct Edge {
    dice_roll_code:String,
    dice_roll_probability:f64,
    expected_rolls_required:f64,
    destination:u32,
}

impl Edge {
    pub fn new(drc : &str, dest : &u32) -> Self {
        let ret = Self{
            dice_roll_code:drc.to_string(), 
            dice_roll_probability: probability_parser::get_proba_from_code(drc),
            expected_rolls_required:probability_parser::get_expected_rolls_from_code(drc),
            destination:dest.clone()};
        ret
    }
}

pub struct Node{
    number : u32, //when the JSON will contain all the squares this will be redundant
    edges : Vec<Edge>,
}

impl Node {
    pub fn new(num : u32, edgies: Vec<Edge>) -> Self {
        Self{number:num, edges:edgies}
    }
}

pub struct Graph {
    nodes : Vec<Node>,
}

impl Graph{
    pub fn new(game_data : &game_loader::Game) -> Self {
        let mut ret = Self{nodes: Vec::<Node>::new()};
        for node_data in &game_data.squares {
            let mut edges = Vec::<Edge>::new();
            for edge_data in &node_data.paths {
                edges.push(Edge::new(&edge_data.0, edge_data.1))
            }

            ret.nodes.push(Node::new(node_data.number, edges));
        }
        ret
    }
}
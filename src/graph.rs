use crate::dice_event::DiceRollRequirement;
use crate::dice_event::SingleValueRequirement;
use crate::game_loader;
use crate::dice_event_parser;
use crate::dice_event;
use std::sync::Arc;
use std::collections::HashMap;

pub struct Edge {
    pub dice_roll_code:String,
    pub requirement : Arc<dyn dice_event::DiceRollRequirement>,
    pub destination:u32,
}

impl Edge {
    pub fn new(drc : &str, dest : &u32, die_face_num : &u8) -> Self {
        
        let ret = Self{
            dice_roll_code:drc.to_string(), 
            requirement: dice_event_parser::parse_event_code(drc, die_face_num),
            destination:dest.clone()};
        ret
    }
}


//also calculate the expected amount of turns passed on each square???? MIGHT BE VERY USEFUL
pub struct Node{
    pub number : u32,
    pub edges : Vec<Edge>,
}

impl Node {
    pub fn new(num : &u32, edgies: Vec<Edge>) -> Self {
        Self{number:num.clone(), edges:edgies}
    }
}

pub struct Graph {
    nodes : HashMap<u32, Node>, //when the JSON will be complete it will be safe to make this a Vec
}

impl Graph{
    pub fn new(game_data : &game_loader::Game) -> Self {
        let mut ret = Self{nodes: HashMap::<u32, Node>::new()};
        for node_data in &game_data.squares {
            let mut edges = Vec::<Edge>::new();
            for edge_data in &node_data.paths {
                edges.push(Edge::new(&edge_data.0, edge_data.1, &game_data.die_faces))
            }

            ret.nodes.insert(node_data.number, Node::new(&node_data.number, edges));
        }
        ret
    }

    pub fn edges_for_node(&self, node_num:&u32) -> Option<&Vec<Edge>> {
        match self.nodes.get(node_num) {
            Some(node) => Some(&node.edges),
            None => None,
        }
    }

    pub fn node_nums(&self) -> Vec<u32> {
        let mut result = Vec::<u32>::new();
        for node in &self.nodes {
            result.push(node.0.clone());
        }

        result
    }

    pub fn path_probability(&self, path : &Vec<u32>) -> f64 {
        //let mut result = 1.0_f64;
        let mut global_event = dice_event::ConsecutiveSequentialRollsRequirement{rolls: Vec::<Arc::<dyn DiceRollRequirement>>::new()};
        
        if path.len() < 2 {
            return 0.0_f64
        }

        for i in 0..path.len() - 1 {
            let node_num = path[i];
            let node = self.nodes.get(&node_num).unwrap();
            let edge = node.edges.iter().find(|desc| desc.destination == path[i+1]);
            if edge.is_none() {
                continue;
            }
            global_event.rolls.push(edge.unwrap().requirement.clone());
            
        }
        global_event.success_probability_for_one_turn()
    }
}
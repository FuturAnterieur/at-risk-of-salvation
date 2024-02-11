use crate::graph;
use crate::min_heap;
use std::collections::HashMap;
use ordered_float::NotNan;

pub enum EdgeDistanceMetric {
    Constant,
    ExpectedRolls,
}


pub struct ShortestPathResultForNode {
    pub predecessors: Vec<u32>,
    pub total_distance : f64,  //probably measure in expected rolls, the distance metric for edges

}


pub fn dijkstra(graph : &graph::Graph, start_node_num :&u32, dest_node_num :&u32, metric : EdgeDistanceMetric) -> Option<ShortestPathResultForNode> {
    
    let node_nums = graph.node_nums();
    
    let mut previous = HashMap::<u32, u32>::new();
    let mut distances = HashMap::<u32, f64>::new();
    let mut heap = min_heap::NodeDistanceMinHeap::new();

    distances.insert(start_node_num.clone(), 0_f64);
    for node_num in node_nums {
        if &node_num != start_node_num { 
            //previous.insert(node_num.clone(), 0); //0 is undefined
            distances.insert(node_num.clone(), f64::INFINITY);
        }
        heap.push(node_num.clone(), distances.get(&node_num).unwrap().clone());
    }

    while !heap.is_empty() {
        let closest_node_pair = heap.pop().unwrap();
        // if only dest is needed : 
        //if &closest_node_pair.node_num == dest_node_num {break;}

        let recorded_distance = distances.get(&closest_node_pair.node_num).unwrap();

        if NotNan::<f64>::new(recorded_distance.clone()).expect("") < closest_node_pair.distance.0 { continue; }

        for edge in graph.edges_for_node(&closest_node_pair.node_num).unwrap() {
            let alternative_distance = distances.get(&closest_node_pair.node_num).unwrap() 
                + match metric {
                    EdgeDistanceMetric::ExpectedRolls => edge.requirement.expected_turns(),
                    EdgeDistanceMetric::Constant => 1.0_f64,
                };

            let dest_distance = distances.get(&edge.destination);

            if dest_distance.is_none() { continue; }

            if &alternative_distance < dest_distance.unwrap() {
                heap.push(edge.destination.clone(), alternative_distance.clone());
                *distances.entry(edge.destination.clone()).or_insert(f64::INFINITY) = alternative_distance;
                //get_mut could also be used
                *previous.entry(edge.destination.clone()).or_insert(0) = closest_node_pair.node_num.clone();
            }  
        }   
    }
    
    
    let mut predecessors = Vec::<u32>::new();
    let mut current_node = dest_node_num;
    while let Some(next_node) = previous.get(current_node) {
        predecessors.push(next_node.clone());
        current_node = next_node;
    }
    

    let result = ShortestPathResultForNode{predecessors:predecessors, total_distance:distances.get(&dest_node_num).unwrap().clone()};

    Some(result)
}


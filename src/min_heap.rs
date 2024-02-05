use std::collections::BinaryHeap;
use std::cmp::{Reverse, Ord, PartialOrd, PartialEq, Ordering};
use ordered_float::NotNan;

pub type MinNotNan = Reverse<NotNan<f64>>;

#[derive(Debug)]
pub struct NodeDistance {
    pub node_num : u32,
    pub distance : MinNotNan,
}

impl Eq for NodeDistance {}

impl PartialEq for NodeDistance {
    fn eq(&self, other : &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl PartialOrd for NodeDistance {
    fn partial_cmp(&self, other : &Self) -> Option<Ordering> {
        self.distance.partial_cmp(&other.distance)
    }
}

impl Ord for NodeDistance {
    fn cmp(&self, other : &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

pub struct NodeDistanceMinHeap(BinaryHeap<NodeDistance>);

impl NodeDistanceMinHeap {

    pub fn new() -> Self {
        NodeDistanceMinHeap{0:BinaryHeap::<NodeDistance>::new()}
    }

    pub fn push(&mut self, num : u32, dist : f64) {
        self.0.push(NodeDistance{node_num:num, distance:Reverse::<NotNan::<f64>>(NotNan::new(dist).expect(""))})
    }

    pub fn pop(&mut self) -> Option<NodeDistance> {
        self.0.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

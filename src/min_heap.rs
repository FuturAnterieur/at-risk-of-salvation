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

pub type NodeDistanceMinHeap = BinaryHeap<NodeDistance>;

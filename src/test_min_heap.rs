

#[cfg(test)]
mod tests {
    use crate::min_heap;
    use std::cmp::Reverse;
    use ordered_float::NotNan;
    #[test]
    fn test_min_heap() {
        let mut my_heap = min_heap::NodeDistanceMinHeap::new();
        my_heap.push(min_heap::NodeDistance{node_num:0, distance:Reverse::<NotNan::<f64>>(NotNan::new(4.0_f64).expect(""))});
        my_heap.push(min_heap::NodeDistance{node_num:1, distance:Reverse::<NotNan::<f64>>(NotNan::new(0.123345_f64).expect(""))});
        my_heap.push(min_heap::NodeDistance{node_num:2, distance:Reverse::<NotNan::<f64>>(NotNan::new(1999293_f64).expect(""))});
        let first_node = my_heap.pop();
        assert_eq!(first_node.is_some(), true);
        assert_eq!(first_node.unwrap().node_num, 1);
        let second_node = my_heap.pop();
        assert_eq!(second_node.unwrap().node_num, 0);
        let third_node = my_heap.pop();
        assert_eq!(third_node.unwrap().node_num, 2);
        assert_eq!(my_heap.pop(), None);
    }
}
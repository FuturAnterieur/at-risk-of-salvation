

#[cfg(test)]
mod tests {
    use crate::min_heap;
    use std::cmp::Reverse;
    use ordered_float::NotNan;
    #[test]
    fn test_min_heap() {
        let mut my_heap = min_heap::NodeDistanceMinHeap::new();
        my_heap.push(0, 4.0_f64);
        my_heap.push(1, 0.12345_f64);
        my_heap.push(2, 1000_f64);
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
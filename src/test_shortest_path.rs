
#[cfg(test)]
mod test_shortest_path {
    use crate::graph;
    use crate::game_loader;
    use crate::shortest_path;
    use assert_float_eq::*;
    use std::fs;
    #[test]
    fn test_dijkstra() {
        let contents= fs::read_to_string("./assets/path_test_game.json").expect("Could not read game file.");
        let sakya_pandita = game_loader::build_game_from_json_string(contents.as_str()).expect("Could not parse game JSON string");

        let g = graph::Graph::new(&sakya_pandita);
        let dijkstra_result = shortest_path::dijkstra(&g, &1, &4, false);

        assert_eq!(dijkstra_result.is_some(), true);
        let res = dijkstra_result.unwrap();
        assert_float_absolute_eq!(res.total_distance, 2.0_f64);
        assert_eq!(res.predecessors.len(), 3);
        assert_eq!(res.predecessors, vec![4,2,1]);

    }
}
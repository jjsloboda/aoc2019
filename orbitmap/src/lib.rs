use std::collections::HashMap;

pub struct Tree {
    edges: HashMap<String, Vec<String>>,
}
impl Tree {
    pub fn new(orbits: Vec<(String, String)>) -> Tree {
        let mut edge_map = HashMap::new();
        for orbit in orbits {
            edge_map.entry(orbit.0).or_insert(Vec::new()).push(orbit.1);
        }
        Tree{ edges: edge_map }
    }
    pub fn dfs_depth_total(&self) -> i32 {
        self.dfs_depth_total_helper(0, &String::from("COM"))
    }
    fn dfs_depth_total_helper(&self, depth: i32, key: &String) -> i32 {
        let next_edges = match self.edges.get(key) {
            Some(x) => x,
            None => return depth,
        };
        let mut total_depth = depth;
        for edge in next_edges {
            total_depth += self.dfs_depth_total_helper(depth + 1, edge);
        }
        total_depth
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

    #[test]
    fn example_depth_totals() {
        let tree = Tree::new(vec![
            (String::from("COM"), String::from("B")),
            (String::from("B"), String::from("C")),
            (String::from("C"), String::from("D")),
            (String::from("D"), String::from("E")),
            (String::from("E"), String::from("F")),
            (String::from("B"), String::from("G")),
            (String::from("G"), String::from("H")),
            (String::from("D"), String::from("I")),
            (String::from("E"), String::from("J")),
            (String::from("J"), String::from("K")),
            (String::from("K"), String::from("L")),
        ]);
        assert_eq!(42, tree.dfs_depth_total());
    }
}

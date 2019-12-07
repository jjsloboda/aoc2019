use std::collections::HashMap;

pub struct Tree {
    edges: HashMap<String, Vec<String>>,
    root_key: String,
}
impl Tree {
    pub fn new(orbits: Vec<(String, String)>) -> Tree {
        let mut edge_map = HashMap::new();
        for orbit in orbits {
            edge_map.entry(orbit.0).or_insert(Vec::new()).push(orbit.1);
        }
        Tree{ edges: edge_map, root_key: String::from("COM") }
    }

    // Part 1
    pub fn dfs_depth_total(&self) -> i32 {
        self.dfs_depth_total_helper(0, &self.root_key)
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

    // Part 2
    pub fn dfs_path_from_root(&self, target: &String) -> Vec<&String> {
        self.dfs_path_from_root_helper(&self.root_key, target, vec![]).expect("key not found")
    }
    fn dfs_path_from_root_helper<'a>(&'a self, key: &'a String, target: &String, path_so_far: Vec<&'a String>) -> Option<Vec<&'a String>> {
        if key == target {
            return Some(path_so_far.clone());
        }
        let next_edges = self.edges.get(key)?;
        for edge in next_edges {
            let mut new_path_so_far = path_so_far.clone();
            new_path_so_far.push(key);
            if let Some(path) = self.dfs_path_from_root_helper(edge, target, new_path_so_far) {
                return Some(path);
            }
        }
        None
    }
    pub fn shared_prefix_len(&self, p1: &Vec<&String>, p2: &Vec<&String>) -> usize {
        let shorter_len = *vec![p1.len(), p2.len()].iter().min().expect("dang");
        for i in 0..shorter_len {
            if p1[i] != p2[i] {
                return i - 1;
            }
        }
        shorter_len - 1
    }
    pub fn min_orbital_transfers(&self, n1: &String, n2: &String) -> usize {
        let p1 = self.dfs_path_from_root(n1);
        let p2 = self.dfs_path_from_root(n2);
        let pfx = self.shared_prefix_len(&p1, &p2);
        (p1.len() - pfx - 1) + (p2.len() - pfx - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::Tree;

    fn make_tree() -> Tree {
        Tree::new(vec![
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
        ])
    }

    #[test]
    fn example_depth_totals() {
        let tree = make_tree();
        assert_eq!(42, tree.dfs_depth_total());
    }

    #[test]
    fn depth_path() {
        let tree = make_tree();
        assert_eq!(1, tree.dfs_path_from_root(&String::from("B")).len());
        assert_eq!(3, tree.dfs_path_from_root(&String::from("H")).len());
        assert_eq!(7, tree.dfs_path_from_root(&String::from("L")).len());
    }

    #[test]
    fn shared_prefix_len() {
        let tree = make_tree();
        let pd = tree.dfs_path_from_root(&String::from("D"));
        let pf = tree.dfs_path_from_root(&String::from("F"));
        let ph = tree.dfs_path_from_root(&String::from("H"));
        let pi = tree.dfs_path_from_root(&String::from("I"));
        let pk = tree.dfs_path_from_root(&String::from("K"));
        assert_eq!(2, tree.shared_prefix_len(&pd, &pd));
        assert_eq!(1, tree.shared_prefix_len(&pf, &ph));
        assert_eq!(3, tree.shared_prefix_len(&pi, &pk));
        assert_eq!(3, tree.shared_prefix_len(&pk, &pi));
    }

    #[test]
    fn min_orbital_transfers() {
        let tree = make_tree();
        assert_eq!(2, tree.min_orbital_transfers(&String::from("K"), &String::from("I")));
        assert_eq!(2, tree.min_orbital_transfers(&String::from("I"), &String::from("K")));
        assert_eq!(6, tree.min_orbital_transfers(&String::from("L"), &String::from("H")));
        assert_eq!(4, tree.min_orbital_transfers(&String::from("F"), &String::from("H")));
    }
}

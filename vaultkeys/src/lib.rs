use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use std::io::BufRead;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
struct Maze {
    grid: Vec<Vec<char>>,
    symbols: HashMap<char, (usize, usize)>,
    distance_travelled: u64,
    keys_collected: HashSet<char>,
    pub keys_trail: Vec<char>,
    distance_lookup: Rc<HashMap<(usize, usize, usize, usize), u64>>,
    prereqs: Rc<HashMap<char, HashSet<char>>>,
}
impl Maze {
    pub fn from_data<T: BufRead>(data: T) -> Self {
        let mut symbols = HashMap::new();
        let mut grid = Vec::new();
        for (y, line) in data.lines().enumerate() {
            let mut row = Vec::new();
            for (x, ch) in line.expect("line read failed").trim().chars().enumerate() {
                row.push(ch);
                if ch != '#' && ch != '.' {
                    symbols.insert(ch, (x, y));
                }
            }
            grid.push(row);
        }
        let lookup = Self::build_distance_lookup(&grid, &symbols);
        let prereqs = Self::build_prereqs(&grid, &symbols);
        let mut maze = Maze{
            grid: grid,
            symbols: symbols,
            distance_travelled: 0,
            keys_collected: HashSet::new(),
            keys_trail: Vec::new(),
            distance_lookup: Rc::new(lookup),
            prereqs: Rc::new(prereqs),
        };
        maze.fill_dead_ends_optimizer();
        println!("maze:\n{}", maze);
        maze
    }
    fn fill_dead_ends_optimizer(&mut self) {
        // TODO super naive brute force
        let mut changed = true;
        while changed {
            changed = false;
            for y in 1..self.grid.len()-1 {
                for x in 1..self.grid[0].len()-1 {
                    if self.grid[y][x] == '.' || self.grid[y][x].is_ascii_uppercase() {
                        let mut blocked_sides = 0;
                        if self.grid[y-1][x] == '#' { blocked_sides += 1; }
                        if self.grid[y+1][x] == '#' { blocked_sides += 1; }
                        if self.grid[y][x-1] == '#' { blocked_sides += 1; }
                        if self.grid[y][x+1] == '#' { blocked_sides += 1; }
                        if blocked_sides >= 3 {
                            self.grid[y][x] = '#';
                            changed = true;
                        }
                    }
                }
            }
        }
    }
    fn distance(grid: &Vec<Vec<char>>, loc1: &(usize, usize), loc2: &(usize, usize)) -> (Option<(u64, HashSet<char>)>) {
        let (x1, y1) = *loc1;
        let (x2, y2) = *loc2;
        let mut grid = grid.clone();
        let mut q = VecDeque::new();
        q.push_back((x1, y1, 0, Rc::new(HashSet::new())));
        while !q.is_empty() {
            let (x, y, d, seen) = q.pop_front().unwrap();
            if x == x2 && y == y2 {
                return Some((d, (*seen).clone()));
            } else if grid[y][x] != '#' {
                let new_seen = if grid[y][x].is_ascii_alphabetic() {
                    let mut ns = (*seen).clone();
                    ns.insert(grid[y][x].to_ascii_lowercase());
                    Rc::new(ns)
                } else {
                    seen
                };
                grid[y][x] = '#';
                q.push_back((x-1, y, d+1, new_seen.clone()));
                q.push_back((x+1, y, d+1, new_seen.clone()));
                q.push_back((x, y-1, d+1, new_seen.clone()));
                q.push_back((x, y+1, d+1, new_seen.clone()));
            }
        }
        None
    }
    fn build_distance_lookup(
        grid: &Vec<Vec<char>>, symbols: &HashMap<char, (usize, usize)>
    ) -> HashMap<(usize, usize, usize, usize), u64> {
        let mut lookup = HashMap::new();
        for &(x1, y1) in symbols.values() {
            for &(x2, y2) in symbols.values() {
                if let Some((d, _)) = Self::distance(grid, &(x1, y1), &(x2, y2)) {
                    lookup.insert((x1, y1, x2, y2), d);
                }
            }
        }
        lookup
    }
    fn build_prereqs(
        grid: &Vec<Vec<char>>, symbols: &HashMap<char, (usize, usize)>
    ) -> HashMap<char, HashSet<char>> {
        let mut prereqs = HashMap::new();
        let (x1, y1) = symbols[&'@'];
        for (&ch, &(x2, y2)) in symbols.iter().filter(|&(&c, _)| c != '@') {
            if let Some((_, mut s)) = Self::distance(grid, &(x1, y1), &(x2, y2)) {
                s.remove(&ch);
                prereqs.insert(ch, s);
            }
        }
        prereqs
    }
    pub fn get_loc_for_symbol(&self, c: char) -> (usize, usize) {
        self.symbols[&c]
    }
    pub fn num_keys(&self) -> u64 {
        self.get_keys().len() as u64
    }
    pub fn distance_travelled(&self) -> u64 {
        self.distance_travelled
    }
    fn distance_to_key(&self, k: char) -> Option<u64> {
        if self.prereqs[&k].is_subset(&self.keys_collected) {
            let (start_x, start_y) = self.symbols[&'@'];
            let (target_x, target_y) = self.symbols[&k];
            match self.distance_lookup.get(&(start_x, start_y, target_x, target_y)) {
                Some(&d) => Some(d),
                None => None,
            }
        } else {
            None
        }
    }
    pub fn move_to_key(&self, k: char) -> Option<Self> {
        if let Some(d) = self.distance_to_key(k) {
            let (key_x, key_y) = self.symbols[&k];
            let (cur_x, cur_y) = self.symbols[&'@'];
            let mut grid = self.grid.clone();
            grid[key_y][key_x] = '@';
            grid[cur_y][cur_x] = '.';
            if let Some((door_x, door_y)) = self.symbols.get(
                &k.to_uppercase().next().unwrap()) {
                grid[*door_y][*door_x] = '.';
            }
            let mut symbols = self.symbols.clone();
            symbols.remove(&k);
            symbols.remove(&k.to_uppercase().next().unwrap());
            symbols.insert('@', (key_x, key_y));
            let mut keys_collected = self.keys_collected.clone();
            keys_collected.insert(k);
            let mut keys_trail = self.keys_trail.clone();
            keys_trail.push(k);
            let maze = Maze{
                grid: grid,
                symbols: symbols,
                distance_travelled: self.distance_travelled + d,
                keys_collected: keys_collected,
                keys_trail: keys_trail,
                distance_lookup: self.distance_lookup.clone(),
                prereqs: self.prereqs.clone(),
            };
            Some(maze)
        } else {
            None
        }
    }
    pub fn get_keys(&self) -> Vec<char> {
        self.symbols.keys().filter(|k| k.is_lowercase()).map(|k| *k).collect()
    }
    pub fn get_cur_loc(&self) -> (usize, usize) {
        self.get_loc_for_symbol('@')
    }
    pub fn make_memo_entry_key(&self) -> (usize, usize, BTreeSet<char>) {
        let (x1, y1) = self.get_cur_loc();
        let keys_remaining = self.get_keys().iter().map(|&k| k)
            .collect::<BTreeSet<char>>();
        (x1, y1, keys_remaining)
    }
}
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.grid.iter() {
            for ch in v.iter() {
                write!(f, "{}", ch)?
            }
            write!(f, "\n")?
        }
        write!(f, "\n")
    }
}

fn fewest_steps_helper(
    m: Maze, dist_memo: &mut HashMap<(usize, usize, BTreeSet<char>), Option<u64>>
) -> Option<u64> {
    let memo_key = m.make_memo_entry_key();
    if m.num_keys() == 0 {
        let dist = m.distance_travelled();
        println!("trail: {:?}, dist: {}", m.keys_trail, dist);
        Some(dist)
    } else if dist_memo.contains_key(&memo_key) {
        match dist_memo[&memo_key] {
            Some(d) => Some(m.distance_travelled() + d),
            None => None,
        }
    } else {
        let mut min_dist = None;
        for &k in m.get_keys().iter() {
            if let Some(new_m) = m.move_to_key(k) {
                if let Some(dist) = fewest_steps_helper(new_m, dist_memo) {
                    min_dist = match min_dist {
                        Some(m) => Some(min(m, dist)),
                        None => Some(dist),
                    };
                }
            }
        }
        match min_dist {
            Some(d) => {
                let min_from_here = d - m.distance_travelled();
                dist_memo.insert(memo_key, Some(min_from_here));
            },
            None => {
                dist_memo.insert(memo_key, None);
            },
        };
        min_dist
    }
}

pub fn fewest_steps_for_all_keys<T: BufRead>(data: T) -> Option<u64> {
    let m = Maze::from_data(data);
    let mut dist_memo = HashMap::new();
    fewest_steps_helper(m, &mut dist_memo)
}

#[cfg(test)]
mod tests {
    const EXAMPLE1: &[u8] =
      b"#########\n\
        #b.A.@.a#\n\
        #########" as &[u8];

    const EXAMPLE2: &[u8] =
      b"########################\n\
        #f.D.E.e.C.b.A.@.a.B.c.#\n\
        ######################.#\n\
        #d.....................#\n\
        ########################" as &[u8];

    const EXAMPLE3: &[u8] =
      b"########################\n\
        #...............b.C.D.f#\n\
        #.######################\n\
        #.....@.a.B.c.d.A.e.F.g#\n\
        ########################" as &[u8];

    const EXAMPLE4: &[u8] =
      b"#################\n\
        #i.G..c...e..H.p#\n\
        ########.########\n\
        #j.A..b...f..D.o#\n\
        ########@########\n\
        #k.E..a...g..B.n#\n\
        ########.########\n\
        #l.F..d...h..C.m#\n\
        #################" as &[u8];

    const EXAMPLE5: &[u8] =
      b"########################\n\
        #@..............ac.GI.b#\n\
        ###d#e#f################\n\
        ###A#B#C################\n\
        ###g#h#i################\n\
        ########################" as &[u8];

    const EXAMPLE6: &[u8] =
      b"#################\n\
        #e.A..b...c..D.h#\n\
        ########@########\n\
        #f.C..a...d..B.g#\n\
        #################" as &[u8];

    #[test]
    fn check_maze_loading() {
        use super::Maze;
        let m = Maze::from_data(EXAMPLE1);
        assert_eq!((1, 1), m.get_loc_for_symbol('b'));
        assert_eq!((3, 1), m.get_loc_for_symbol('A'));
        assert_eq!((5, 1), m.get_loc_for_symbol('@'));
        assert_eq!((7, 1), m.get_loc_for_symbol('a'));

        let keys = m.get_keys();
        assert!(keys.contains(&'a'));
        assert!(keys.contains(&'b'));
    }

    #[test]
    fn check_key_queries() {
        use super::Maze;

        let m1 = Maze::from_data(EXAMPLE1);
        assert_eq!(2, m1.num_keys());
        assert_eq!(Some(2), m1.distance_to_key('a'));
        assert_eq!(None, m1.distance_to_key('b'));

        let m2 = Maze::from_data(EXAMPLE2);
        assert_eq!(6, m2.num_keys());
        assert_eq!(Some(2), m2.distance_to_key('a'));
        assert_eq!(None, m2.distance_to_key('b'));
    }

    #[test]
    fn check_move_to_key() {
        use super::Maze;

        let m1 = Maze::from_data(EXAMPLE1).move_to_key('a').unwrap();
        assert_eq!(1, m1.num_keys());
    }

    #[test]
    fn check_fewest_steps_basic() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(Some(8), fewest_steps_for_all_keys(EXAMPLE1));
        assert_eq!(Some(86), fewest_steps_for_all_keys(EXAMPLE2));
    }

    #[test]
    fn check_fewest_steps_3() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(Some(132), fewest_steps_for_all_keys(EXAMPLE3));
    }

    #[test]
    fn check_fewest_steps_4() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(Some(136), fewest_steps_for_all_keys(EXAMPLE4));
    }

    #[test]
    fn check_fewest_steps_5() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(Some(81), fewest_steps_for_all_keys(EXAMPLE5));
    }

    #[test]
    fn check_fewest_steps_6() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(Some(62), fewest_steps_for_all_keys(EXAMPLE6));
    }
}

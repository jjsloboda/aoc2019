use std::collections::{HashMap, VecDeque};
use std::io::BufRead;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
struct Maze {
    grid: Vec<Vec<char>>,
    symbols: HashMap<char, (usize, usize)>,
    distance_travelled: u64,
    distance_lookup: Rc<HashMap<(usize, usize, usize, usize), u64>>,
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
        let mut maze = Maze{
            grid: grid,
            symbols: symbols,
            distance_travelled: 0,
            distance_lookup: Rc::new(lookup),
        };
        maze.fill_dead_ends_optimizer();
        maze
    }
    fn fill_dead_ends_optimizer(&mut self) {
        // TODO super naive brute force
        let mut changed = true;
        while changed {
            changed = false;
            for y in 1..self.grid.len()-1 {
                for x in 1..self.grid[0].len()-1 {
                    if self.grid[y][x] == '.' {
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
    fn distance(grid: &Vec<Vec<char>>, loc1: &(usize, usize), loc2: &(usize, usize)) -> Option<u64> {
        let (x1, y1) = *loc1;
        let (x2, y2) = *loc2;
        let mut grid = grid.clone();
        let mut q = VecDeque::new();
        q.push_back((x1, y1, 0));
        while !q.is_empty() {
            let (x, y, d) = q.pop_front().unwrap();
            if x == x2 && y == y2 {
                return Some(d);
            } else if grid[y][x] != '#' {
                grid[y][x] = '#';
                q.push_back((x-1, y, d+1));
                q.push_back((x+1, y, d+1));
                q.push_back((x, y-1, d+1));
                q.push_back((x, y+1, d+1));
            }
        }
        None
    }
    fn build_distance_lookup(grid: &Vec<Vec<char>>, symbols: &HashMap<char, (usize, usize)>
    ) -> HashMap<(usize, usize, usize, usize), u64> {
        let mut lookup = HashMap::new();
        for &(x1, y1) in symbols.values() {
            for &(x2, y2) in symbols.values() {
                if let Some(d) = Self::distance(grid, &(x1, y1), &(x2, y2)) {
                    lookup.insert((x1, y1, x2, y2), d);
                }
            }
        }
        lookup
    }
    pub fn get_loc_for_symbol(&self, c: char) -> (usize, usize) {
        self.symbols[&c]
    }
    pub fn num_keys(&self) -> u64 {
        self.get_keys().iter().count() as u64
    }
    pub fn distance_travelled(&self) -> u64 {
        self.distance_travelled
    }
    fn distance_to_key(&self, k: char) -> Option<u64> {
        let (start_x, start_y) = self.symbols[&'@'];
        let (target_x, target_y) = self.symbols[&k];
        match self.distance_lookup.get(&(start_x, start_y, target_x, target_y)) {
            Some(&d) => Some(d),
            None => None,
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
            let mut maze = Maze{
                grid: grid,
                symbols: symbols,
                distance_travelled: self.distance_travelled + d,
                distance_lookup: Rc::new(HashMap::new()),
            };
            maze.fill_dead_ends_optimizer();
            println!("{}", maze);
            Some(maze)
        } else {
            None
        }
    }
    pub fn get_keys(&self) -> Vec<char> {
        self.symbols.keys().filter(|k| k.is_lowercase()).map(|k| *k).collect()
    }
}
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.grid.iter() {
            for ch in v.iter() {
                write!(f, "{}", ch)?
            }
        }
        write!(f, "\n")
    }
}

fn fewest_steps_helper(m: Maze) -> Option<u64> {
    if m.num_keys() == 0 {
        Some(m.distance_travelled())
    } else {
        m.get_keys().iter()
            .map(|&k| {
                if let Some(new_m) = m.move_to_key(k) {
                    fewest_steps_helper(new_m)
                        .unwrap_or(u64::max_value())
                } else {
                    u64::max_value()
                }
            })
            .min()
    }
}

pub fn fewest_steps_for_all_keys<T: BufRead>(data: T) -> Option<u64> {
    let m = Maze::from_data(data);
    fewest_steps_helper(m)
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
        assert_eq!(8, fewest_steps_for_all_keys(EXAMPLE1).unwrap());
        assert_eq!(86, fewest_steps_for_all_keys(EXAMPLE2).unwrap());
    }

    #[test]
    fn check_fewest_steps_3() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(132, fewest_steps_for_all_keys(EXAMPLE3).unwrap());
    }

    #[test]
    fn check_fewest_steps_4() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(136, fewest_steps_for_all_keys(EXAMPLE4).unwrap());
    }

    #[test]
    fn check_fewest_steps_5() {
        use super::fewest_steps_for_all_keys;
        assert_eq!(81, fewest_steps_for_all_keys(EXAMPLE5).unwrap());
    }
}

use std::collections::{HashMap, HashSet, VecDeque};
use std::io::BufRead;

type Pt = (usize, usize);

struct Maze {
    grid: Vec<Vec<char>>,
    portals: HashMap<Pt, Pt>,
    inner_portals: HashSet<Pt>,
    start: Pt,
    end: Pt,
}
impl Maze {
    pub fn from_data<T: BufRead>(data: T) -> Self {
        let grid = Self::load_grid(data);
        let (portals, in_ptls, start, end) =
            Self::build_portals_start_end(&grid);
        let mut maze = Maze{
            grid: grid,
            portals: portals,
            inner_portals: in_ptls,
            start: start,
            end: end,
        };
        maze.fill_dead_ends_optimizer();
        maze.block_terminal_labels();
        print_grid(&maze.grid);
        maze
    }
    fn load_grid<T: BufRead>(data: T) -> Vec<Vec<char>> {
        let mut grid = Vec::new();
        for line in data.lines() {
            let mut row = Vec::new();
            for ch in line.expect("line read failed").chars() {
                row.push(ch);
            }
            grid.push(row);
        }
        grid
    }
    fn build_portals_start_end(
        grid: &Vec<Vec<char>>
    ) -> (HashMap<Pt, Pt>, HashSet<Pt>, Pt, Pt) {
        let mut prtl_map = HashMap::new();
        for y in 0..(grid.len()-1) {
            for x in 0..(grid[0].len()-1) {
                if grid[y][x].is_ascii_uppercase() {
                    if x > 0 && grid[y][x-1].is_ascii_uppercase() ||
                        y > 0 && grid[y-1][x].is_ascii_uppercase() {
                        // Already processed this label
                        continue;
                    }
                    let ch1 = grid[y][x];
                    let (label, point) = if grid[y+1][x].is_ascii_uppercase() {
                        // Vertical
                        let ch2 = grid[y+1][x];
                        if y > 0 && grid[y-1][x] == '.' { // Portal above label
                            (vec![ch1, ch2].iter().collect::<String>(), (x, y-1))
                        } else { // Portal below label
                            (vec![ch1, ch2].iter().collect::<String>(), (x, y+2))
                        }
                    } else {
                        // Horizontal
                        let ch2 = grid[y][x+1];
                        if x > 0 && grid[y][x-1] == '.' { // Portal left of label
                            (vec![ch1, ch2].iter().collect::<String>(), (x-1, y))
                        } else { // Portal right of label
                            (vec![ch1, ch2].iter().collect::<String>(), (x+2, y))
                        }
                    };
                    prtl_map.entry(label).or_insert(Vec::new()).push(point);
                }
            }
        }
        let start = prtl_map.remove("AA").expect("AA label not found")[0];
        let end = prtl_map.remove("ZZ").expect("ZZ label not found")[0];
        let mut portals = HashMap::new();
        let mut inner_portals = HashSet::new();
        for pts in prtl_map.values() {
            if pts.len() != 2 {
                panic!("portal with non-2 count");
            }
            portals.insert(pts[0], pts[1]);
            portals.insert(pts[1], pts[0]);
            if pts[0].0 != 2 && pts[0].1 != 2 &&
                    pts[0].0 != grid[0].len()-3 && pts[0].1 != grid.len()-3 {
                inner_portals.insert(pts[0]);
            }
            if pts[1].0 != 2 && pts[1].1 != 2 &&
                    pts[1].0 != grid[0].len()-3 && pts[1].1 != grid.len()-3 {
                inner_portals.insert(pts[1]);
            }
        }
        (portals, inner_portals, start, end)
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
    fn block_terminal_labels(&mut self) {
        let (ax, ay) = Self::find_adjacent_point(&self.grid, &self.start, 'A')
            .expect("no such maze");
        let (zx, zy) = Self::find_adjacent_point(&self.grid, &self.end, 'Z')
            .expect("no such maze");
        self.grid[ay][ax] = '#';
        self.grid[zy][zx] = '#';
    }
    fn find_adjacent_point(grid: &Vec<Vec<char>>, &(x, y): &Pt, c: char) -> Option<Pt> {
        if x > 0 && grid[y][x-1] == c {
            Some((x-1, y))
        } else if x < grid[0].len()-1 && grid[y][x+1] == c {
            Some((x+1, y))
        } else if y > 0 && grid[y-1][x] == c {
            Some((x, y-1))
        } else if y < grid.len()-1 && grid[y+1][x] == c {
            Some((x, y+1))
        } else {
            None
        }
    }
    pub fn basic_distance(&self, src: Pt, dst: Pt) -> Option<u64> {
        let mut grid = self.grid.clone();
        let mut q = VecDeque::new();
        q.push_back((src, 0));
        while !q.is_empty() {
            let ((x, y), d) = q.pop_front().unwrap();
            if x == dst.0 && y == dst.1 {
                return Some(d)
            } else if grid[y][x].is_ascii_uppercase() {
                let adj_pt = Self::find_adjacent_point(&grid, &(x, y), '#')
                    .expect("no adjacent point");
                grid[adj_pt.1][adj_pt.0] = '#';
                q.push_front((self.portals[&adj_pt], d));
            } else if grid[y][x] == '.' {
                grid[y][x] = '#';
                q.push_back(((x-1, y), d+1));
                q.push_back(((x+1, y), d+1));
                q.push_back(((x, y-1), d+1));
                q.push_back(((x, y+1), d+1));
            }
        }
        None
    }
    pub fn recursive_distance(&self, src: Pt, dst: Pt) -> Option<u64> {
        let mut grids = Vec::new();
        let mut q = VecDeque::new();
        grids.push(self.grid.clone());
        q.push_back((src, 0, 0));
        while !q.is_empty() {
            let ((x, y), d, lvl) = q.pop_front().unwrap();
            if x == dst.0 && y == dst.1 && lvl == 0 {
                return Some(d)
            } else if grids[lvl][y][x].is_ascii_uppercase() {
                let adj_pt = Self::find_adjacent_point(&grids[lvl], &(x, y), '#')
                    .expect("no adjacent point");
                grids[lvl][adj_pt.1][adj_pt.0] = '#';
                if self.inner_portals.contains(&adj_pt) {
                    grids.push(self.grid.clone());
                    q.push_front((self.portals[&adj_pt], d, lvl+1));
                } else if lvl > 0 {
                    q.push_front((self.portals[&adj_pt], d, lvl-1));
                }
            } else if grids[lvl][y][x] == '.' {
                grids[lvl][y][x] = '#';
                q.push_back(((x-1, y), d+1, lvl));
                q.push_back(((x+1, y), d+1, lvl));
                q.push_back(((x, y-1), d+1, lvl));
                q.push_back(((x, y+1), d+1, lvl));
            }
        }
        None
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for v in grid.iter() {
        for ch in v.iter() {
            print!("{}", ch);
        }
        println!("");
    }
    println!("");
}

pub fn find_shortest_path<T: BufRead>(data: T) -> Option<u64> {
    let maze = Maze::from_data(data);
    maze.basic_distance(maze.start, maze.end)
}

pub fn find_shortest_recursive_path<T: BufRead>(data: T) -> Option<u64> {
    let maze = Maze::from_data(data);
    maze.recursive_distance(maze.start, maze.end)
}

#[cfg(test)]
mod tests {
    const EXAMPLE1: &[u8] =
b"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       \n" as &[u8];

    const PART2EXAMPLE: &[u8] =
b"             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     \n" as &[u8];

    #[test]
    fn check_example_1() {
        use super::find_shortest_path;
        assert_eq!(Some(23), find_shortest_path(EXAMPLE1));
    }

    #[test]
    fn check_part_2_example() {
        use super::find_shortest_recursive_path;
        assert_eq!(Some(396), find_shortest_recursive_path(PART2EXAMPLE));
    }
}

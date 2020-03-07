use std::collections::{HashSet, VecDeque};
use std::io::BufRead;

type Grid = [[bool; 5]; 5];

pub fn load_grid<T: BufRead>(data: T) -> Grid {
    let mut arr = [[false; 5]; 5];
    for (i, line) in data.lines().enumerate() {
        for (j, ch) in line.expect("error reading line from file")
                .trim().chars().enumerate() {
            arr[i][j] = ch == '#';
        }
    }
    arr
}

pub fn print_grid(grid: &Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let ch = if grid[i][j] { '#' } else { '.' };
            print!("{}", ch);
        }
        println!("");
    }
}

pub fn print_multigrid(grids: &VecDeque<Grid>) {
    for g in grids.iter() {
        println!("");
        print_grid(g);
        println!("");
    }
}

fn iterate_grid(grid: &Grid) -> Grid {
    let mut new_grid = grid.clone();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let mut adj_bugs = 0;
            if j > 0 && grid[j-1][i] {
                adj_bugs += 1;
            }
            if j < grid[0].len()-1 && grid[j+1][i] {
                adj_bugs += 1;
            }
            if i > 0 && grid[j][i-1] {
                adj_bugs += 1;
            }
            if i < grid.len()-1 && grid[j][i+1] {
                adj_bugs += 1;
            }

            if grid[j][i] && adj_bugs != 1 {
                new_grid[j][i] = false;
            } else if !grid[j][i] && (adj_bugs == 1 || adj_bugs == 2) {
                new_grid[j][i] = true;
            }
        }
    }
    new_grid
}

fn biodiversity(grid: &Grid) -> u64 {
    let mut b = 0;
    for i in (0..grid.len()).rev() {
        for j in (0..grid[0].len()).rev() {
            b <<= 1;
            if grid[i][j] {
                b |= 1;
            }
        }
    }
    b
}

pub fn find_first_repeated(grid: &Grid) -> u64 {
    let mut seen = HashSet::new();
    let mut g = grid.clone();
    loop {
        let b = biodiversity(&g);
        if seen.contains(&b) {
            return b;
        }
        seen.insert(b);
        g = iterate_grid(&g);
    }
}

fn empty(grid: &Grid) -> bool {
    biodiversity(grid) == 0
}

fn count_bugs(grid: &Grid) -> u32 {
    biodiversity(grid).count_ones()
}

fn count_adjacent(grids: &VecDeque<Grid>, i: usize, j: usize, k: usize) -> u64 {
    let mut cnt = 0;

    // Assume a 5x5 grid for now
    let (y, x) = (j as isize, k as isize);
    if x == 2 && y == 2 {
        return 0;
    }
    let squares_to_check = vec![(y-1, x), (y+1, x), (y, x-1), (y, x+1)];
    for &(yy, xx) in squares_to_check.iter() {
        let outer_bound = yy == -1 || yy == 5 || xx == -1 || xx == 5;
        let inner_bound = yy == 2 && xx == 2;
        if outer_bound {
            if yy == -1 { // top outer edge
                if i > 0 && grids[i-1][1][2] {
                    cnt += 1;
                }
            } else if yy == 5 { // bottom outer edge
                if i > 0 && grids[i-1][3][2] {
                    cnt += 1;
                }
            } else if xx == -1 { // left outer edge
                if i > 0 && grids[i-1][2][1] {
                    cnt += 1;
                }
            } else if xx == 5 { // right outer edge
                if i > 0 && grids[i-1][2][3] {
                    cnt += 1;
                }
            }
        } else if inner_bound {
            if x == 1 { // left
                for m in 0..5 {
                    if i < grids.len()-1 && grids[i+1][m][0] {
                        cnt += 1;
                    }
                }
            } else if x == 3 { // right
                for m in 0..5 {
                    if i < grids.len()-1 && grids[i+1][m][4] {
                        cnt += 1;
                    }
                }
            } else if y == 1 { // top
                for m in 0..5 {
                    if i < grids.len()-1 && grids[i+1][0][m] {
                        cnt += 1;
                    }
                }
            } else if y == 3 { // bottom
                for m in 0..5 {
                    if i < grids.len()-1 && grids[i+1][4][m] {
                        cnt += 1;
                    }
                }
            }
        } else if grids[i][yy as usize][xx as usize] {
            cnt += 1;
        }
    }
    cnt
}

fn iterate_multigrid(grids: &VecDeque<Grid>) -> VecDeque<Grid> {
    let mut old_grids = grids.clone();
    if !empty(grids.front().expect("no grids")) {
        old_grids.push_front([[false; 5]; 5]);
    }
    if !empty(grids.back().expect("no grids")) {
        old_grids.push_back([[false; 5]; 5]);
    }
    let mut new_grids = old_grids.clone();
    for i in 0..old_grids.len() {
        for j in 0..old_grids[0].len() {
            for k in 0..old_grids[0][0].len() {
                let adj_bugs = count_adjacent(&old_grids, i, j, k);
                if old_grids[i][j][k] && adj_bugs != 1 {
                    new_grids[i][j][k] = false;
                } else if !old_grids[i][j][k] && (adj_bugs == 1 || adj_bugs == 2) {
                    new_grids[i][j][k] = true;
                }
            }
        }
    }
    new_grids
}

pub fn find_num_bugs(grid: &Grid, n: usize) -> u32 {
    let mut grids = VecDeque::new();
    grids.push_back(grid.clone());
    for _ in 0..n {
        grids = iterate_multigrid(&mut grids);
    }
    grids.iter().map(|g| count_bugs(g)).sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn check_read_from_file() {
        use super::load_grid;
        const BUF: &[u8] =
          b"....#\n\
            #..#.\n\
            #..##\n\
            ..#..\n\
            #...." as &[u8];
        assert_eq!(load_grid(BUF), [
            [false, false, false, false,  true],
            [ true, false, false,  true, false],
            [ true, false, false,  true,  true],
            [false, false,  true, false, false],
            [ true, false, false, false, false],
        ]);
    }

    #[test]
    fn iterate_successfully() {
        use super::iterate_grid;
        let grid = [
            [false, false, false, false,  true],
            [ true, false, false,  true, false],
            [ true, false, false,  true,  true],
            [false, false,  true, false, false],
            [ true, false, false, false, false],
        ];
        let grid1 = iterate_grid(&grid);
        assert_eq!(grid1, [
            [ true, false, false,  true, false],
            [ true,  true,  true,  true, false],
            [ true,  true,  true, false,  true],
            [ true,  true, false,  true,  true],
            [false,  true,  true, false, false],
        ]);
    }

    #[test]
    fn check_biodiversity() {
        use super::biodiversity;
        let grid = [
            [false, false, false, false, false],
            [false, false, false, false, false],
            [false, false, false, false, false],
            [ true, false, false, false, false],
            [false,  true, false, false, false],
        ];
        assert_eq!(2129920, biodiversity(&grid));
    }

    #[test]
    fn check_first_repeated() {
        use super::find_first_repeated;
        let grid = [
            [false, false, false, false,  true],
            [ true, false, false,  true, false],
            [ true, false, false,  true,  true],
            [false, false,  true, false, false],
            [ true, false, false, false, false],
        ];
        assert_eq!(2129920, find_first_repeated(&grid));
    }
}

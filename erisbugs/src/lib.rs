use std::collections::HashSet;
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

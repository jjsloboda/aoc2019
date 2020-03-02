use std::fmt;
use std::io::BufRead;

type Grid = [[bool; 5]; 5];

pub fn load_grid<T: BufRead>(data: T) -> Grid {
    let mut arr = [[false; 5]; 5];
    for (i, line) in data.lines().enumerate() {
        for (j, ch) in line.expect("error reading line from file")
                .trim().chars().enumerate() {
            arr[j][i] = ch == '#';
        }
    }
    arr
}

pub fn print_grid(grid: &Grid) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let ch = if grid[j][i] { '#' } else { '.' };
            print!("{}", ch);
        }
        println!("");
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

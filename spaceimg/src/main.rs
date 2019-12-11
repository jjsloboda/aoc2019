extern crate spaceimg;

use std::fs::read_to_string;

use spaceimg::Image;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let buf: Vec<u32> = input.trim().chars()
        .map(|x| x.to_digit(10).expect("failed to parse input"))
        .collect();
    let img = Image::new(buf, 25, 6);

    let dcpl = img.digit_count_per_layer();
    let min_zeros = dcpl.iter()
        .min_by(|x, y| x[0].cmp(&y[0]))
        .expect("no items");
    println!("one digit count * two digit count: {}", min_zeros[1] * min_zeros[2]);
    println!("rendered image:");
    img.print();
}

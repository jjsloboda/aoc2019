use std::char;

pub struct Image {
    buf: Vec<u32>,
    w: u32,
    h: u32,
    num_pixels: u32,
    num_layers: u32,
}
impl Image {
    pub fn new(buf: Vec<u32>, w: u32, h: u32) -> Image {
        let num_px = w * h;
        let num_layers = buf.len() as u32 / num_px;
        Image{
            buf: buf,
            w: w,
            h: h,
            num_pixels: num_px,
            num_layers: num_layers,
        }
    }
    pub fn digit_count_per_layer(&self) -> Vec<[u32; 10]> {
        let mut output = Vec::new();
        for i in 0..self.num_layers {
            let mut arr: [u32; 10] = [0; 10];
            for j in 0..self.num_pixels {
                arr[self.buf[(i*self.num_pixels+j) as usize] as usize] += 1;
            }
            output.push(arr);
        }
        output
    }
    pub fn render(&self) -> Vec<u32> {
        let mut output = Vec::new();
        for i in 0..self.num_pixels {
            for l in 0..self.num_layers {
                let px = self.buf[(l*self.num_pixels+i) as usize];
                if px != 2 {
                    output.push(px);
                    break;
                }
            }
        }
        output
    }
    pub fn print(&self) {
        let img = self.render();
        for j in 0..self.h {
            for i in 0..self.w {
                let px = img[(j*self.w+i) as usize];
                let c = if px == 0 { ' ' } else { char::from_digit(px, 10).unwrap() };
                print!("{}", c);
            }
            println!("");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Image;

    #[test]
    fn small_example() {
        let img = Image::new(vec![0,0,1,2,1,1,2,0,2,2,1,0], 2, 2);
        let dcpl = img.digit_count_per_layer();
        assert_eq!(3, dcpl.len());
        assert_eq!([2,1,1,0,0,0,0,0,0,0], dcpl[0]);
        assert_eq!([1,2,1,0,0,0,0,0,0,0], dcpl[1]);
        assert_eq!([1,1,2,0,0,0,0,0,0,0], dcpl[2]);
    }

    #[test]
    fn render_example() {
        let img = Image::new(vec![0,2,2,2,1,1,2,2,2,2,1,2,0,0,0,0], 2, 2);
        let rendered_img = img.render();
        assert_eq!(4, rendered_img.len());
        assert_eq!(vec![0,1,1,0], rendered_img);
    }
}

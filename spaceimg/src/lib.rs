

pub struct Image {
    buf: Vec<u32>,
    w: u32,
    h: u32,
}
impl Image {
    pub fn new(buf: Vec<u32>, w: u32, h: u32) -> Image {
        Image{ buf: buf, w: w, h: h }
    }
    pub fn digit_count_per_layer(&self) -> Vec<[u32; 10]> {
        let num_pixels = self.w * self.h;
        let num_layers = self.buf.len() as u32 / num_pixels;
        let mut output = Vec::new();
        for i in 0..num_layers {
            let mut arr: [u32; 10] = [0; 10];
            for j in 0..num_pixels {
                arr[self.buf[(i*num_pixels+j) as usize] as usize] += 1;
            }
            output.push(arr);
        }
        output
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
}

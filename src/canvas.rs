const FACTOR: usize = 50;
const WIDTH: usize = 16 * FACTOR;
const HEIGHT: usize = 9 * FACTOR;

const CANVAS_BYTES_COUNT: usize = WIDTH * HEIGHT * 4;

static mut CANVAS_BYTES: [u8; CANVAS_BYTES_COUNT] = [0; CANVAS_BYTES_COUNT];

pub unsafe fn get_default_canvas() -> Canvas<'static> {
    Canvas {
        pixels: &mut CANVAS_BYTES,
        width: WIDTH,
        height: HEIGHT,
    }
}

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Canvas<'a> {
    pixels: &'a mut [u8],
    width: usize,
    height: usize,
}

impl<'a> Canvas<'a> {
    pub fn new(pixels: &'a mut [u8], width: usize) -> Self {
        let count = pixels.len();
        Self {
            pixels,
            width,
            height: count / 4 / width,
        }
    }

    pub fn get_bytes(&'a self) -> &'a [u8] {
        self.pixels
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn set(&mut self, x: usize, y: usize, color: &Color) {
        let idx = (y * self.width + x) * 4;
        self.pixels[idx + 0] = color.r;
        self.pixels[idx + 1] = color.g;
        self.pixels[idx + 2] = color.b;
        self.pixels[idx + 3] = color.a;
    }

    pub fn at(&self, x: usize, y: usize) -> Color {
        let idx = (y * self.width + x) * 4;
        Color {
            r: self.pixels[idx + 0],
            g: self.pixels[idx + 1],
            b: self.pixels[idx + 2],
            a: self.pixels[idx + 3],
        }
    }

    pub fn fill(&mut self, color: &Color) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.set(x, y, color);
            }
        }
    }

    pub fn rect(&mut self, x0: usize, y0: usize, w: usize, h: usize, color: &Color) {
        for y in y0..(y0 + h) {
            for x in x0..(x0 + w) {
                self.set(x, y, color);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_canvas_with_proper_height() {
        let mut pixels: [u8; 250 * 4] = [0; 250 * 4];
        let c = Canvas::new(&mut pixels, 50);
        assert_eq!(c.get_height(), 5);
    }

    #[test]
    fn fill_sets_uniform_color() {
        let mut pixels: [u8; 250 * 4] = [0; 250 * 4];
        let mut c = Canvas::new(&mut pixels, 50);
        let bg = Color {
            r: 255,
            g: 11,
            b: 22,
            a: 33,
        };
        c.fill(&bg);

        for y in 0..c.get_height() {
            for x in 0..c.get_height() {
                let actual = c.at(x, y);
                assert_eq!(bg, actual);
            }
        }
    }

    #[test]
    fn rect_dimensions() {
        let mut pixels: [u8; 250 * 4] = [0; 250 * 4];
        let mut c = Canvas::new(&mut pixels, 50);
        let fg = Color {
            r: 255,
            g: 11,
            b: 22,
            a: 33,
        };
        c.rect(5, 2, 10, 2, &fg);

        {
            let expected = Color {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            };
            assert_eq!(c.at(0, 0), expected);
            assert_eq!(c.at(5, 1), expected);
            assert_eq!(c.at(5, 4), expected);
            assert_eq!(c.at(15, 2), expected);
        }

        {
            assert_eq!(c.at(5, 2), fg);
            assert_eq!(c.at(5, 3), fg);
        }
    }
}

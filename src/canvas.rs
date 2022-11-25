use crate::color::Color;

pub struct Canvas {
    width: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0.0, 0.0, 0.0); width * height];
        Self { width, pixels }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.pixels.len() / self.width
    }

    pub fn pixels(&self) -> Pixels {
        Pixels {
            pixels: self.pixels.iter(),
        }
    }

    pub fn pixels_mut(&mut self) -> PixelsMut {
        PixelsMut {
            pixels: self.pixels.iter_mut(),
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn write_pixel(mut self, x: usize, y: usize, c: Color) -> Self {
        let i = self.index(x, y);
        self.pixels[i] = c;
        self
    }

    pub fn enumerate_pixels_mut(&mut self) -> EnumeratePixelsMut {
        let width = self.width();
        EnumeratePixelsMut::new(self.pixels.iter_mut(), width)
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[self.index(x, y)]
    }

    pub fn to_ppm(&self) -> Vec<u8> {
        let header = format!("P6\n{} {}\n255\n", self.width(), self.height());
        let mut result = header.into_bytes();
        for pixel in self.pixels() {
            let (r, g, b) = pixel.to_byte_triple();
            result.push(r);
            result.push(g);
            result.push(b);
        }
        result
    }
}

pub struct Pixels<'a> {
    pixels: std::slice::Iter<'a, Color>,
}

impl<'a> Iterator for Pixels<'a> {
    type Item = &'a Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.pixels.next()
    }
}

pub struct PixelsMut<'a> {
    pixels: std::slice::IterMut<'a, Color>,
}

impl<'a> Iterator for PixelsMut<'a> {
    type Item = &'a mut Color;

    fn next(&mut self) -> Option<Self::Item> {
        self.pixels.next()
    }
}

pub struct EnumeratePixelsMut<'a> {
    cells: std::slice::IterMut<'a, Color>,
    width: usize,
    row: usize,
    col: usize,
}

impl<'a> EnumeratePixelsMut<'a> {
    fn new(cells: std::slice::IterMut<'a, Color>, width: usize) -> Self {
        Self {
            cells,
            width,
            row: 0,
            col: 0,
        }
    }
}

impl<'a> Iterator for EnumeratePixelsMut<'a> {
    type Item = (usize, usize, &'a mut Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.col >= self.width {
            self.col = 0;
            self.row += 1;
        }
        let r = self.row;
        let c = self.col;
        self.col += 1;
        self.cells.next().map(|cell| (r, c, cell))
    }
}

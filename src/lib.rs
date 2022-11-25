use canvas::Canvas;
use color::Color;

pub mod canvas;
pub mod color;

type Point = (usize, usize);

#[derive(Debug, Clone, Copy)]
struct Focus {
    point: Point,
    color: Color,
}

impl Focus {
    fn new(row: usize, col: usize, color: Color) -> Self {
        Self {
            point: (row, col),
            color,
        }
    }
}

struct VoronoiImage {
    width: usize,
    height: usize,
    focii: Vec<Focus>,
}

impl VoronoiImage {
    fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            focii: Vec::new(),
        }
    }

    fn with_focii(mut self, focii: &[Focus]) -> Self {
        self.focii.extend_from_slice(focii);
        self
    }

    fn nearest_focus(&self, row: usize, col: usize) -> Option<(f64, &Focus)> {
        let mut output = None;
        for f in self.focii.iter() {
            let curr_distance = distance(&f.point, &(row, col));
            match output {
                Some((d, _)) => {
                    if curr_distance < d {
                        output = Some((curr_distance, f))
                    }
                }
                None => output = Some((curr_distance, f)),
            }
        }
        output
    }

    fn build_canvas(&self) -> Canvas {
        let mut canvas = Canvas::new(self.width, self.height);
        const DOT_SIZE: f64 = 4.0;
        for (r, c, p) in canvas.enumerate_pixels_mut() {
            let color = self
                .nearest_focus(r, c)
                .map(|(d, f)| {
                    if d <= DOT_SIZE {
                        color::consts::BLACK
                    } else {
                        f.color
                    }
                })
                .unwrap_or(*p);

            *p = color;
        }
        canvas
    }
}

fn distance(p1: &Point, p2: &Point) -> f64 {
    let &(r0, c0) = p1;
    let &(r1, c1) = p2;
    let delta_c = c0.abs_diff(c1) as f64;
    let delta_r = r0.abs_diff(r1) as f64;
    f64::sqrt(f64::powi(delta_c, 2) + f64::powi(delta_r, 2))
}

fn random_point(width: usize, height: usize) -> Point {
    let row = rand::random::<usize>() % height;
    let col = rand::random::<usize>() % width;
    (row, col)
}

fn random_color() -> Color {
    let r = rand::random::<f64>();
    let g = rand::random::<f64>();
    let b = rand::random::<f64>();
    Color::new(r, g, b)
}

fn random_focus(width: usize, height: usize) -> Focus {
    let (r, c) = random_point(width, height);
    Focus::new(r, c, random_color())
}

pub fn run() -> Canvas {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    let num_focii = 20;
    let mut focii = Vec::with_capacity(num_focii);
    for _ in 0..num_focii {
        focii.push(random_focus(WIDTH, HEIGHT));
    }

    let image = VoronoiImage::new(WIDTH, HEIGHT).with_focii(&focii);
    image.build_canvas()
}

#[cfg(test)]
mod voronoi_tests {
    mod distance_test {
        use crate::distance;

        #[test]
        fn the_distance_between_a_point_and_itself_is_zero() {
            let p1 = (0, 0);
            let p2 = (0, 0);
            assert_eq!(distance(&p1, &p2), 0.0);
        }

        #[test]
        fn the_distance_between_two_points_in_same_row() {
            let p1 = (0, 5);
            let p2 = (0, 10);
            assert_eq!(distance(&p1, &p2), 5.0);
        }

        #[test]
        fn the_distance_between_two_points_in_same_column() {
            let p1 = (5, 0);
            let p2 = (10, 0);
            assert_eq!(distance(&p1, &p2), 5.0);
        }

        #[test]
        fn the_distance_between_two_arbitrary_points() {
            let p1 = (0, 0);
            let p2 = (1, 1);
            assert_eq!(distance(&p1, &p2), f64::sqrt(2.0));
        }
    }
}

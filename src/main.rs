use std::{fs, io};

use voronoi::run;

fn main() -> io::Result<()> {
    let canvas = run();
    let ppm = canvas.to_ppm();
    let filename = "voronoi.ppm";
    fs::write(filename, ppm)?;
    Ok(())
}

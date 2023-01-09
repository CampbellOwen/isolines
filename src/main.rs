use image::io::Reader as ImageReader;
//use rand::{thread_rng, Rng};

mod marching_squares;
use marching_squares::*;
mod util;
use util::*;

fn main() {
    //let points = vec![
    //    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 4.0, 4.0, 0.0, 0.0, 4.0, 4.0,
    //    4.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    //];

    let img = ImageReader::open("maple_bay_square.tif")
        .expect("Should exist")
        .decode()
        .expect("Should be valid")
        .into_luma16();

    let data = img
        .as_flat_samples()
        .image_slice()
        .expect("Has data")
        .iter()
        .map(|&x| x as f32)
        .collect();

    let field = Field {
        extent: (img.width(), img.height()),
        vals: data,
    };

    println!(
        "<svg width=\"{}\" height=\"{}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">",
        img.width(),
        img.height()
    );

    let highest = field
        .vals
        .iter()
        .reduce(|biggest, f| if f > biggest { f } else { biggest })
        .unwrap()
        .to_owned();

    let num_lines = 15;
    let step = (highest as usize) / num_lines;
    for threshold in (0..highest as usize).step_by(step) {
        let paths = field.layer_paths(threshold as f32);
        for path in paths.iter().filter(|p| p.points.len() > 2) {
            println!(
                "<path stroke=\"black\" stroke-width=\"1\" fill=\"none\" d=\"{}\" />",
                path.to_svg()
            );
        }
    }
    println!("</svg>");
}

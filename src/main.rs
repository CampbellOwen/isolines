use rand::{thread_rng, Rng};

mod marching_squares;
use marching_squares::*;
mod util;
use util::*;

fn main() {
    //let points = vec![
    //    0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 4.0, 4.0, 4.0, 0.0, 0.0, 4.0, 4.0,
    //    4.0, 0.0, 0.0, 0.0, 4.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,
    //];

    let width = 10;
    let height = 10;
    let mut points = Vec::new();
    for _ in 0..(width * height) {
        points.push(thread_rng().gen::<f32>() * 25.0);
    }
    let field = Field {
        extent: (width, height),
        vals: points,
    };
    let threshold = 5.0;

    println!(
        "<svg width=\"{}\" height=\"{}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">",
        width, height
    );

    let paths = field.layer_paths(threshold);
    for path in paths {
        println!(
            "<path stroke=\"black\" stroke-width=\"1\" fill=\"none\" d=\"{}\" />",
            path.to_svg()
        );
    }
    println!("</svg>");
}

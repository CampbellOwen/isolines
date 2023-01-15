use image::io::Reader as ImageReader;
//use rand::{thread_rng, Rng};

mod marching_squares;
use marching_squares::*;
mod util;
use util::*;

fn main() {
    let palette = [
        "#ffd8ba", "#f7a983", "#f28a91", "#db3b5d", "#57253b", "#ac2925", "#ef692f", "#eca549",
        "#3e88b7", "#4b3b9c", "#6a6c56", "#adac8e", "#fff4e0", "#cecfbf", "#939487", "#2b2b26",
    ];

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

    let num_lines = palette.len();
    let step = (highest as usize) / num_lines;
    for (i, threshold) in (0..highest as usize).step_by(step).enumerate() {
        let colour = palette[(palette.len() - 1) - (i % palette.len())];
        println!("<g stroke=\"{colour}\" stroke-width=\"1\" fill=\"none\" >");
        let paths = field.layer_paths(threshold as f32, true);
        for path in paths.iter().filter(|p| p.points.len() > 2) {
            let fill = if path.closed { colour } else { "none" };
            println!("<path fill=\"{fill}\" d=\"{}\" />", path.to_svg(true));
        }
        println!("</g>");
    }

    //let threshold = 47.0;
    //let paths = field.layer_paths(threshold);
    //paths.iter().for_each(|path| println!("{}", path.to_svg()));
    println!("</svg>");
}

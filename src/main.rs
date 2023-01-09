use itertools::join;
use rand::{thread_rng, Rng};

mod marching_squares;
use marching_squares::*;
mod util;
use util::*;

#[derive(Debug, Clone)]
struct Path {
    points: Vec<Point>,
}
impl Path {
    pub fn start(&self) -> Point {
        self.points.first().expect("Shouldn't be empty").to_owned()
    }
    pub fn end(&self) -> Point {
        self.points.last().expect("Shouldn't be empty").to_owned()
    }
    pub fn to_svg(&self) -> String {
        let svg_path = "M ".to_string();

        svg_path
            + &(join(
                self.points
                    .iter()
                    .map(|p| format!("{} {}", p.x + 0.5, p.y + 0.5)),
                " L ",
            ))
            + if self.start() == self.end() { " Z" } else { "" }
    }
}

fn paths_from_lines(lines: &[Line]) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    for line in lines {
        let mut matching_start: Option<u32> = None;
        let mut matching_end: Option<u32> = None;
        for (i, path) in paths.iter().enumerate() {
            if path.end() == line.start {
                if matching_start.is_some() {
                    panic!("Multiple matches???")
                }
                matching_start = Some(i as u32);
            }
            if path.start() == line.end {
                if matching_end.is_some() {
                    panic!("Multiple matches???")
                }
                matching_end = Some(i as u32);
            }
        }
        if matching_start.is_some() && matching_end.is_some() {
            let i_start = matching_start.unwrap();
            let i_end = matching_end.unwrap();

            if i_start == i_end {
                paths[i_start as usize].points.push(line.end);
            } else {
                let mut start_path;
                let mut end_path;
                if i_start < i_end {
                    end_path = paths.remove(i_end as usize);
                    start_path = paths.remove(i_start as usize);
                } else {
                    start_path = paths.remove(i_start as usize);
                    end_path = paths.remove(i_end as usize);
                }

                start_path.points.push(line.end);
                start_path.points.append(&mut end_path.points);
                paths.push(start_path);
            }
        } else if let Some(i_start) = matching_start {
            paths[i_start as usize].points.push(line.end);
        } else if let Some(i_end) = matching_end {
            paths[i_end as usize].points.insert(0, line.start);
        } else {
            paths.push(Path {
                points: vec![line.start, line.end],
            })
        }
    }

    paths
}

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
    let mut lines = Vec::new();
    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            let cell = field.cell_at(threshold, (x, y));
            //println!("Cell: {:?}", cell);
            match cell.segment {
                CellSegment::Zero => (),
                CellSegment::One(line) => {
                    //println!("<line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"1\" />", line.start.0, line.end.0, line.start.1, line.end.1);
                    lines.push(line);
                }
                CellSegment::Two(line1, line2) => {
                    //println!("<line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"1\" />", line1.start.0, line1.end.0, line1.start.1, line1.end.1);
                    //println!("<line x1=\"{}\" x2=\"{}\" y1=\"{}\" y2=\"{}\" stroke=\"black\" stroke-width=\"1\" />", line2.start.0, line2.end.0, line2.start.1, line2.end.1);
                    lines.push(line1);
                    lines.push(line2);
                }
            };
        }
    }

    let paths = paths_from_lines(&lines);
    for path in paths {
        println!(
            "<path stroke=\"black\" stroke-width=\"1\" fill=\"none\" d=\"{}\" />",
            path.to_svg()
        );
    }
    println!("</svg>");
}

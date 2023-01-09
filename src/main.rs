use itertools::join;
use rand::{distributions::Uniform, thread_rng, Rng};

#[derive(Debug, PartialEq)]
struct Line {
    start: (f32, f32),
    end: (f32, f32),
}

#[derive(Debug)]
struct Cell {
    // Upper left corner
    pos: (u32, u32),
    id: u8,
    segment: CellSegment,
}

#[derive(Debug, PartialEq)]
enum CellSegment {
    Zero,
    One(Line),
    Two(Line, Line),
}

#[derive(Debug)]
struct Field {
    extent: (u32, u32),
    vals: Vec<f32>,
}

impl Field {
    pub fn val_at(&self, pos: (u32, u32)) -> f32 {
        let (x, y) = pos;
        if x > self.extent.0 - 1 || y > self.extent.1 - 1 {
            panic!("Invalid coords");
        }
        let i = x + y * self.extent.0;
        self.vals[i as usize]
    }

    pub fn cell_at(&self, threshold: f32, pos: (u32, u32)) -> Cell {
        let (x, y) = pos;
        if x > self.extent.0 - 2 || y > self.extent.1 - 2 {
            panic!("Invalid cell coords");
        }

        let vals = [
            self.val_at((x, y)),
            self.val_at((x + 1, y)),
            self.val_at((x, y + 1)),
            self.val_at((x + 1, y + 1)),
        ];

        let id = id_from_vals(threshold, &vals);

        let segment = cell_segment(threshold, pos, id, &vals);

        Cell { pos, id, segment }
    }
}

fn id_from_vals(threshold: f32, vals: &[f32; 4]) -> u8 {
    vals.iter().fold(0, |id, &val| {
        (id << 1) + (if val > threshold { 1 } else { 0 })
    })
}

fn cell_segment(threshold: f32, pos: (u32, u32), id: u8, vals: &[f32; 4]) -> CellSegment {
    let (x, y) = pos;
    let top_left = vals[0];
    let top_right = vals[1];
    let bottom_left = vals[2];
    let bottom_right = vals[3];
    match id {
        0b0000 | 0b1111 => CellSegment::Zero,
        0b0001 | 0b1110 => {
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let right_t = (threshold - top_right) / (bottom_right - top_right);

            let a = (x as f32 + (bottom_t), y as f32 + 1.0);
            let b = (x as f32 + 1.0, y as f32 + (1.0 * right_t));

            if id == 0b0001 {
                (CellSegment::One(Line { start: a, end: b }))
            } else {
                (CellSegment::One(Line { start: b, end: a }))
            }
        }
        0b0010 | 0b1101 => {
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let a = (x as f32, y as f32 + left_t);
            let b = (x as f32 + bottom_t, y as f32 + 1.0);

            if id == 0b0010 {
                (CellSegment::One((Line { start: a, end: b })))
            } else {
                (CellSegment::One((Line { start: b, end: a })))
            }
        }
        0b0011 | 0b1100 => {
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let right_t = (threshold - top_right) / (bottom_right - top_right);
            let a = (x as f32, y as f32 + left_t);
            let b = (x as f32 + 1.0, y as f32 + right_t);
            if id == 0b0011 {
                (CellSegment::One(Line { start: a, end: b }))
            } else {
                (CellSegment::One(Line { start: b, end: a }))
            }
        }
        0b0100 | 0b1011 => {
            let right_t = (threshold - top_right) / (bottom_right - top_right);
            let top_t = (threshold - top_left) / (top_right - top_left);
            let a = (x as f32 + 1.0, y as f32 + right_t);
            let b = (x as f32 + top_t, y as f32);
            if id == 0b0100 {
                (CellSegment::One(Line { start: a, end: b }))
            } else {
                (CellSegment::One(Line { start: b, end: a }))
            }
        }
        0b0101 | 0b1010 => {
            let top_t = (threshold - top_left) / (top_right - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let a = (x as f32 + bottom_t, y as f32 + 1.0);
            let b = (x as f32 + top_t, y as f32);
            if id == 0b0101 {
                (CellSegment::One(Line { start: a, end: b }))
            } else {
                (CellSegment::One(Line { start: b, end: a }))
            }
        }
        0b0110 => {
            let top_t = (threshold - top_left) / (top_right - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let right_t = (threshold - top_right) / (bottom_right - top_right);

            let center = vals.iter().sum::<f32>() / 4.0;

            if center > threshold {
                let a = (x as f32, y as f32 + left_t);
                let b = (x as f32 + top_t, y as f32);
                let first = Line { start: a, end: b };

                let a = (x as f32 + 1.0, y as f32 + right_t);
                let b = (x as f32 + bottom_t, y as f32 + 1.0);
                let second = Line { start: a, end: b };
                (CellSegment::Two(first, second))
            } else {
                let a = (x as f32 + 1.0, y as f32 + right_t);
                let b = (x as f32 + top_t, y as f32);
                let first = Line { start: a, end: b };

                let a = (x as f32, y as f32 + left_t);
                let b = (x as f32 + bottom_t, y as f32 + 1.0);
                let second = Line { start: a, end: b };
                (CellSegment::Two(first, second))
            }
        }
        0b1001 => {
            let top_t = (threshold - top_left) / (top_right - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let right_t = (threshold - top_right) / (bottom_right - top_right);

            let center = vals.iter().sum::<f32>() / 4.0;

            if center > threshold {
                let a = (x as f32 + bottom_t, y as f32 + 1.0);
                let b = (x as f32, y as f32 + left_t);
                let first = Line { start: a, end: b };

                let a = (x as f32 + top_t, y as f32);
                let b = (x as f32 + 1.0, y as f32 + right_t);
                let second = Line { start: a, end: b };
                (CellSegment::Two(first, second))
            } else {
                let a = (x as f32 + bottom_t, y as f32 + 1.0);
                let b = (x as f32 + 1.0, y as f32 + right_t);
                let first = Line { start: a, end: b };

                let a = (x as f32 + top_t, y as f32);
                let b = (x as f32, y as f32 + left_t);
                let second = Line { start: a, end: b };
                (CellSegment::Two(first, second))
            }
        }
        0b0111 | 0b1000 => {
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let top_t = (threshold - top_left) / (top_right - top_left);

            let a = (x as f32, y as f32 + left_t);
            let b = (x as f32 + top_t, y as f32);

            if id == 0b0111 {
                (CellSegment::One(Line { start: a, end: b }))
            } else {
                (CellSegment::One(Line { start: b, end: a }))
            }
        }
        _ => panic!("Invalid id"),
    }
}

#[derive(Debug, Clone)]
struct Path {
    points: Vec<(f32, f32)>,
}
impl Path {
    pub fn start(&self) -> (f32, f32) {
        self.points.first().expect("Shouldn't be empty").to_owned()
    }
    pub fn end(&self) -> (f32, f32) {
        self.points.last().expect("Shouldn't be empty").to_owned()
    }
    pub fn to_svg(&self) -> String {
        let svg_path = "M ".to_string();

        svg_path
            + &(join(
                self.points
                    .iter()
                    .map(|p| format!("{} {}", p.0 + 0.5, p.1 + 0.5)),
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
    for i in 0..(width * height) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_test() {
        let threshold = 5.0;

        let vals = [1.0, 1.0, 2.0, 3.0];
        assert_eq!(0, id_from_vals(threshold, &vals));

        let vals = [1.0, 6.0, 6.0, 6.0];
        assert_eq!(0b0111, id_from_vals(threshold, &vals));

        let vals = [1.0, 6.0, 2.0, 6.0];
        assert_eq!(0b0101, id_from_vals(threshold, &vals));
    }

    #[test]
    fn segment_test() {
        let threshold = 5.0;

        let vals = [1.0, 3.0, 3.0, 7.0];
        let segment = cell_segment(threshold, (0, 0), 0b0001, &vals);
        assert_eq!(
            CellSegment::One(Line {
                start: (0.5, 1.0),
                end: (1.0, 0.5)
            }),
            segment
        );

        let vals = [9.0, 7.0, 7.0, 3.0];
        let segment = cell_segment(threshold, (0, 0), 0b1110, &vals);
        assert_eq!(
            CellSegment::One(Line {
                start: (1.0, 0.5),
                end: (0.5, 1.0),
            }),
            segment
        );
    }
}

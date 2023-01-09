use itertools::join;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, PartialEq)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}
#[derive(Debug, Clone)]
pub struct Path {
    points: Vec<Point>,
    closed: bool,
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

pub fn paths_from_lines(lines: &[Line]) -> Vec<Path> {
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
                paths[i_start as usize].closed = true;
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
                closed: false,
            })
        }
    }

    paths
}

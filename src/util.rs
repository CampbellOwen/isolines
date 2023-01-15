use std::cell;

use itertools::join;

use crate::marching_squares::CellLine;

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

#[derive(Debug, PartialEq, Clone)]
pub struct RichPoint {
    pub raw_point: Point,
    pub interpolated_point: Point,
}
#[derive(Debug, Clone)]
pub struct Path {
    pub points: Vec<RichPoint>,
    pub closed: bool,
}
impl Path {
    pub fn start(&self) -> RichPoint {
        self.points.first().expect("Shouldn't be empty").to_owned()
    }
    pub fn end(&self) -> RichPoint {
        self.points.last().expect("Shouldn't be empty").to_owned()
    }
    pub fn to_svg(&self, interpolated: bool) -> String {
        let svg_path = "M ".to_string();

        svg_path
            + &(join(
                self.points
                    .iter()
                    .map(|p| {
                        if interpolated {
                            p.interpolated_point
                        } else {
                            p.raw_point
                        }
                    })
                    .map(|p| format!("{} {}", p.x + 0.5, p.y + 0.5)),
                " L ",
            ))
            + if self.closed { " Z" } else { "" }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Edge {
    Left,
    Top,
    Right,
    Bottom,
}

fn on_edge(point: &RichPoint, extents: (u32, u32)) -> Option<Edge> {
    let (width, height) = extents;
    let Point { x, y } = point.raw_point;
    if x <= 0.5 {
        return Some(Edge::Left);
    }
    if y <= 0.5 {
        return Some(Edge::Top);
    }
    if x >= (extents.0 as f32 - 1.0) {
        return Some(Edge::Right);
    }
    if y >= (extents.1 as f32 - 1.0) {
        return Some(Edge::Bottom);
    }
    None
}

pub enum CloseEdges {
    None,
    ForExtent(u32, u32),
}
pub fn paths_from_lines(lines: &[CellLine], close_edges: CloseEdges) -> Vec<Path> {
    let mut paths: Vec<Path> = Vec::new();

    for cell_line in lines {
        let line = &cell_line.interpolated_line;
        let raw_line = &cell_line.raw_line;

        let line_start = RichPoint {
            interpolated_point: line.start,
            raw_point: raw_line.start,
        };

        let line_end = RichPoint {
            interpolated_point: line.end,
            raw_point: raw_line.end,
        };

        let mut matching_start: Option<u32> = None;
        let mut matching_end: Option<u32> = None;
        for (i, path) in paths.iter().enumerate() {
            if path.end().raw_point == line_start.raw_point {
                if matching_start.is_some() {
                    println!("i:  {i}, path: {path:?}, line: {line:?}");
                    let matching_path = &paths[matching_start.unwrap() as usize];
                    println!("Previous match: {matching_start:?} - {matching_path:?}");
                    println!("Previous match: {matching_start:?}");
                    panic!("Multiple matches???")
                }
                matching_start = Some(i as u32);
            }
            if path.start().raw_point == line_end.raw_point {
                if matching_end.is_some() {
                    println!("i:  {i}, path: {path:?}, line: {line:?}");
                    let matching_path = &paths[matching_end.unwrap() as usize];
                    println!("Previous match: {matching_end:?} - {matching_path:?}");
                    panic!("Multiple matches???")
                }
                matching_end = Some(i as u32);
            }
        }
        if matching_start.is_some() && matching_end.is_some() {
            let i_start = matching_start.unwrap();
            let i_end = matching_end.unwrap();

            if i_start == i_end {
                paths[i_start as usize].points.push(line_end);
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

                start_path.points.push(line_end);
                start_path.points.append(&mut end_path.points);
                paths.push(start_path);
            }
        } else if let Some(i_start) = matching_start {
            paths[i_start as usize].points.push(line_end);
        } else if let Some(i_end) = matching_end {
            paths[i_end as usize].points.insert(0, line_start);
        } else {
            paths.push(Path {
                points: vec![line_start, line_end],
                closed: false,
            })
        }
    }

    if let CloseEdges::ForExtent(width, height) = close_edges {
        for open_path in paths.iter_mut().filter(|p| !p.closed) {
            let begin_edge = on_edge(&open_path.start(), (width, height));
            let end_edge = on_edge(&open_path.end(), (width, height));
            if begin_edge.is_none() || end_edge.is_none() {
                continue;
            }
            let begin_edge = begin_edge.unwrap();
            let end_edge = end_edge.unwrap();

            let top_left = RichPoint {
                raw_point: Point { x: 0.0, y: 0.0 },
                interpolated_point: Point { x: 0.0, y: 0.0 },
            };
            let top_right = RichPoint {
                raw_point: Point {
                    x: width as f32 - 0.5,
                    y: 0.0,
                },
                interpolated_point: Point {
                    x: width as f32 - 0.5,
                    y: 0.0,
                },
            };
            let bottom_left = RichPoint {
                raw_point: Point {
                    x: 0.0,
                    y: height as f32 - 0.5,
                },
                interpolated_point: Point {
                    x: 0.0,
                    y: height as f32 - 0.5,
                },
            };
            let bottom_right = RichPoint {
                raw_point: Point {
                    x: width as f32 - 0.5,
                    y: height as f32 - 0.5,
                },
                interpolated_point: Point {
                    x: width as f32 - 0.5,
                    y: height as f32 - 0.5,
                },
            };

            match (begin_edge, end_edge) {
                (Edge::Left, Edge::Left) => open_path.closed = true,
                (Edge::Left, Edge::Top) => {
                    open_path.points.push(top_right.clone());
                    open_path.points.push(bottom_right.clone());
                    open_path.points.push(bottom_left.clone());
                    open_path.closed = true;
                }
                (Edge::Left, Edge::Right) => {
                    open_path.points.push(bottom_right.clone());
                    open_path.points.push(bottom_left.clone());
                    open_path.closed = true;
                }
                (Edge::Left, Edge::Bottom) => {
                    open_path.points.push(bottom_left.clone());
                    open_path.closed = true;
                }
                (Edge::Top, Edge::Left) => {
                    open_path.points.push(top_left.clone());
                    open_path.closed = true;
                }
                (Edge::Top, Edge::Top) => open_path.closed = true,
                (Edge::Top, Edge::Right) => {
                    open_path.points.push(top_right.clone());
                    open_path.closed = true;
                }
                (Edge::Top, Edge::Bottom) => {
                    open_path.points.push(bottom_left.clone());
                    open_path.points.push(top_left.clone());
                    open_path.closed = true;
                }
                (Edge::Right, Edge::Left) => {
                    open_path.points.push(top_left.clone());
                    open_path.points.push(top_right.clone());
                    open_path.closed = true;
                }
                (Edge::Right, Edge::Top) => {
                    open_path.points.push(top_right.clone());
                    open_path.closed = true;
                }
                (Edge::Right, Edge::Right) => open_path.closed = true,
                (Edge::Right, Edge::Bottom) => {
                    open_path.points.push(bottom_left.clone());
                    open_path.points.push(top_left.clone());
                    open_path.points.push(top_right.clone());
                    open_path.closed = true;
                }
                (Edge::Bottom, Edge::Left) => {
                    open_path.points.push(bottom_left.clone());
                    open_path.closed = true;
                }
                (Edge::Bottom, Edge::Top) => {
                    open_path.points.push(top_right.clone());
                    open_path.points.push(bottom_right.clone());
                    open_path.closed = true;
                }
                (Edge::Bottom, Edge::Right) => {
                    open_path.points.push(bottom_right.clone());
                    open_path.closed = true;
                }
                (Edge::Bottom, Edge::Bottom) => open_path.closed = true,
            };
        }
    }

    paths
}

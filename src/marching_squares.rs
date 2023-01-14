use crate::{paths_from_lines, util::*};

#[derive(Debug)]
pub struct Cell {
    // Upper left corner
    pub pos: (u32, u32),
    pub id: u8,
    pub segment: CellSegment,
}

#[derive(Debug, PartialEq)]
pub struct CellLine {
    pub interpolated_line: Line,
    pub raw_line: Line,
}

#[derive(Debug, PartialEq)]
pub enum CellSegment {
    Zero,
    One(CellLine),
    Two(CellLine, CellLine),
}

#[derive(Debug)]
pub struct Field {
    pub extent: (u32, u32),
    pub vals: Vec<f32>,
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

    pub fn raw_lines(&self, threshold: f32) -> Vec<CellLine> {
        let mut lines = Vec::new();
        for y in 0..(self.extent.1 - 1) {
            for x in 0..(self.extent.0 - 1) {
                let cell = self.cell_at(threshold, (x, y));
                match cell.segment {
                    CellSegment::Zero => (),
                    CellSegment::One(line) => {
                        lines.push(line);
                    }
                    CellSegment::Two(line1, line2) => {
                        lines.push(line1);
                        lines.push(line2);
                    }
                };
            }
        }

        lines
    }

    pub fn layer_paths(&self, threshold: f32) -> Vec<Path> {
        let lines = self.raw_lines(threshold);
        paths_from_lines(&lines)
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
            let a = Point {
                x: x as f32 + (bottom_t),
                y: y as f32 + 1.0,
            };
            let b = Point {
                x: x as f32 + 1.0,
                y: y as f32 + (1.0 * right_t),
            };

            let interpolated_line = if id == 0b0001 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            let bottom_t = 0.5;
            let right_t = 0.5;

            let a = Point {
                x: x as f32 + (bottom_t),
                y: y as f32 + 1.0,
            };
            let b = Point {
                x: x as f32 + 1.0,
                y: y as f32 + (1.0 * right_t),
            };

            let raw_line = if id == 0b0001 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            CellSegment::One(CellLine {
                interpolated_line,
                raw_line,
            })
        }
        0b0010 | 0b1101 => {
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let a = Point {
                x: x as f32,
                y: y as f32 + left_t,
            };
            let b = Point {
                x: x as f32 + bottom_t,
                y: y as f32 + 1.0,
            };

            let interpolated_line = if id == 0b0010 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            let bottom_t = 0.5;
            let left_t = 0.5;
            let a = Point {
                x: x as f32,
                y: y as f32 + left_t,
            };
            let b = Point {
                x: x as f32 + bottom_t,
                y: y as f32 + 1.0,
            };

            let raw_line = if id == 0b0010 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };
            CellSegment::One(CellLine {
                interpolated_line,
                raw_line,
            })
        }
        0b0011 | 0b1100 => {
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let right_t = (threshold - top_right) / (bottom_right - top_right);
            let a = Point {
                x: x as f32,
                y: y as f32 + left_t,
            };
            let b = Point {
                x: x as f32 + 1.0,
                y: y as f32 + right_t,
            };
            let interpolated_line = if id == 0b0011 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            let right_t = 0.5;
            let left_t = 0.5;
            let a = Point {
                x: x as f32,
                y: y as f32 + left_t,
            };
            let b = Point {
                x: x as f32 + 1.0,
                y: y as f32 + right_t,
            };
            let raw_line = if id == 0b0011 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            CellSegment::One(CellLine {
                interpolated_line,
                raw_line,
            })
        }
        0b0100 | 0b1011 => {
            let right_t = (threshold - top_right) / (bottom_right - top_right);
            let top_t = (threshold - top_left) / (top_right - top_left);
            let a = Point {
                x: x as f32 + 1.0,
                y: y as f32 + right_t,
            };
            let b = Point {
                x: x as f32 + top_t,
                y: y as f32,
            };
            let interpolated_line = if id == 0b0100 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            let right_t = 0.5;
            let top_t = 0.5;

            let a = Point {
                x: x as f32 + 1.0,
                y: y as f32 + right_t,
            };
            let b = Point {
                x: x as f32 + top_t,
                y: y as f32,
            };
            let raw_line = if id == 0b0100 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            CellSegment::One(CellLine {
                interpolated_line,
                raw_line,
            })
        }
        0b0101 | 0b1010 => {
            let top_t = (threshold - top_left) / (top_right - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let a = Point {
                x: x as f32 + bottom_t,
                y: y as f32 + 1.0,
            };
            let b = Point {
                x: x as f32 + top_t,
                y: y as f32,
            };
            let interpolated_line = if id == 0b0101 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            let bottom_t = 0.5;
            let top_t = 0.5;

            let a = Point {
                x: x as f32 + bottom_t,
                y: y as f32 + 1.0,
            };
            let b = Point {
                x: x as f32 + top_t,
                y: y as f32,
            };
            let raw_line = if id == 0b0101 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };
            CellSegment::One(CellLine {
                interpolated_line,
                raw_line,
            })
        }
        0b0110 => {
            let top_t = (threshold - top_left) / (top_right - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let right_t = (threshold - top_right) / (bottom_right - top_right);

            let center = vals.iter().sum::<f32>() / 4.0;

            if center > threshold {
                let a = Point {
                    x: x as f32,
                    y: y as f32 + left_t,
                };
                let b = Point {
                    x: x as f32 + top_t,
                    y: y as f32,
                };
                let first = Line { start: a, end: b };

                let a = Point {
                    x: x as f32 + 1.0,
                    y: y as f32 + right_t,
                };
                let b = Point {
                    x: x as f32 + bottom_t,
                    y: y as f32 + 1.0,
                };
                let second = Line { start: a, end: b };
                CellSegment::Two(
                    CellLine {
                        interpolated_line: first,
                        raw_line: Line {
                            start: Point {
                                x: x as f32,
                                y: y as f32 + 0.5,
                            },
                            end: Point {
                                x: x as f32 + 0.5,
                                y: y as f32,
                            },
                        },
                    },
                    CellLine {
                        interpolated_line: second,
                        raw_line: Line {
                            start: Point {
                                x: x as f32 + 1.0,
                                y: y as f32 + 0.5,
                            },
                            end: Point {
                                x: x as f32 + 0.5,
                                y: y as f32 + 1.0,
                            },
                        },
                    },
                )
            } else {
                let a = Point {
                    x: x as f32 + 1.0,
                    y: y as f32 + right_t,
                };
                let b = Point {
                    x: x as f32 + top_t,
                    y: y as f32,
                };
                let first = Line { start: a, end: b };

                let a = Point {
                    x: x as f32,
                    y: y as f32 + left_t,
                };
                let b = Point {
                    x: x as f32 + bottom_t,
                    y: y as f32 + 1.0,
                };
                let second = Line { start: a, end: b };
                CellSegment::Two(
                    CellLine {
                        interpolated_line: first,
                        raw_line: Line {
                            start: Point {
                                x: x as f32 + 1.0,
                                y: y as f32 + 0.5,
                            },
                            end: Point {
                                x: x as f32 + 0.5,
                                y: y as f32,
                            },
                        },
                    },
                    CellLine {
                        interpolated_line: second,
                        raw_line: Line {
                            start: Point {
                                x: x as f32,
                                y: y as f32 + 0.5,
                            },
                            end: Point {
                                x: x as f32 + 0.5,
                                y: y as f32 + 1.0,
                            },
                        },
                    },
                )
            }
        }
        0b1001 => {
            let top_t = (threshold - top_left) / (top_right - top_left);
            let bottom_t = (threshold - bottom_left) / (bottom_right - bottom_left);
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let right_t = (threshold - top_right) / (bottom_right - top_right);

            let center = vals.iter().sum::<f32>() / 4.0;

            if center > threshold {
                let a = Point {
                    x: x as f32 + bottom_t,
                    y: y as f32 + 1.0,
                };
                let b = Point {
                    x: x as f32,
                    y: y as f32 + left_t,
                };
                let first = Line { start: a, end: b };

                let a = Point {
                    x: x as f32 + top_t,
                    y: y as f32,
                };
                let b = Point {
                    x: x as f32 + 1.0,
                    y: y as f32 + right_t,
                };
                let second = Line { start: a, end: b };
                CellSegment::Two(
                    CellLine {
                        interpolated_line: first,
                        raw_line: Line {
                            start: Point {
                                x: x as f32 + 0.5,
                                y: y as f32 + 1.0,
                            },
                            end: Point {
                                x: x as f32,
                                y: y as f32 + 0.5,
                            },
                        },
                    },
                    CellLine {
                        interpolated_line: second,
                        raw_line: Line {
                            start: Point {
                                x: x as f32 + 0.5,
                                y: y as f32,
                            },
                            end: Point {
                                x: x as f32 + 1.0,
                                y: y as f32 + 0.5,
                            },
                        },
                    },
                )
            } else {
                let a = Point {
                    x: x as f32 + bottom_t,
                    y: y as f32 + 1.0,
                };
                let b = Point {
                    x: x as f32 + 1.0,
                    y: y as f32 + right_t,
                };
                let first = Line { start: a, end: b };

                let a = Point {
                    x: x as f32 + top_t,
                    y: y as f32,
                };
                let b = Point {
                    x: x as f32,
                    y: y as f32 + left_t,
                };
                let second = Line { start: a, end: b };
                CellSegment::Two(
                    CellLine {
                        interpolated_line: first,
                        raw_line: Line {
                            start: Point {
                                x: x as f32 + 0.5,
                                y: y as f32 + 1.0,
                            },
                            end: Point {
                                x: x as f32 + 1.0,
                                y: y as f32 + 0.5,
                            },
                        },
                    },
                    CellLine {
                        interpolated_line: second,
                        raw_line: Line {
                            start: Point {
                                x: x as f32 + 0.5,
                                y: y as f32,
                            },
                            end: Point {
                                x: x as f32,
                                y: y as f32 + 0.5,
                            },
                        },
                    },
                )
            }
        }
        0b0111 | 0b1000 => {
            let left_t = (threshold - top_left) / (bottom_left - top_left);
            let top_t = (threshold - top_left) / (top_right - top_left);
            let a = Point {
                x: x as f32,
                y: y as f32 + left_t,
            };
            let b = Point {
                x: x as f32 + top_t,
                y: y as f32,
            };

            let interpolated_line = if id == 0b0111 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };

            let top_t = 0.5;
            let left_t = 0.5;
            let a = Point {
                x: x as f32,
                y: y as f32 + left_t,
            };
            let b = Point {
                x: x as f32 + top_t,
                y: y as f32,
            };

            let raw_line = if id == 0b0111 {
                Line { start: a, end: b }
            } else {
                Line { start: b, end: a }
            };
            CellSegment::One(CellLine {
                interpolated_line,
                raw_line,
            })
        }
        _ => panic!("Invalid id"),
    }
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
                start: Point { x: 0.5, y: 1.0 },
                end: Point { x: 1.0, y: 0.5 }
            }),
            segment
        );

        let vals = [9.0, 7.0, 7.0, 3.0];
        let segment = cell_segment(threshold, (0, 0), 0b1110, &vals);
        assert_eq!(
            CellSegment::One(Line {
                start: Point { x: 1.0, y: 0.5 },
                end: Point { x: 0.5, y: 1.0 },
            }),
            segment
        );
    }
}

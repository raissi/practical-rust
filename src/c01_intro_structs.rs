
/// Usually points in plane defined by first and second coordinates
/// So we can skip naming them (x, and y)
pub struct Point(i32, i32);

pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn is_horizontal(&self) -> bool {
        self.a.1 == self.b.1
    }

    pub fn new(a: Point, b: Point) -> Self {
        Line{
            a,
            b,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

   

    #[test]
    fn detect_horizontal() {
        let horizontal_line = Line::new(Point(1, 7), Point(9, 7));
        assert_eq!(horizontal_line.is_horizontal(), true);

        let vertical_line = Line::new(Point(1, 0), Point(1, 7));
        assert_eq!(vertical_line.is_horizontal(), false);

        let line = Line::new(Point(1, 0), Point(5, 7));
        assert_eq!(line.is_horizontal(), false);

    }

}
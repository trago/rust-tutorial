mod ml_data;

fn main() {}

fn consume_s(s: String) -> usize {
    s.len()
}

enum State<T, Q = i32> {
    ON(Q),
    OFF(T),
}

mod topology {
    pub struct Point {
        x: f64,
        y: f64,
    }

    pub struct Square {
        p_tl: Point,
        p_br: Point,
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn x(&self) -> f64 {
            self.x
        }
        pub fn y(&self) -> f64 {
            self.y
        }
    }

    impl Square {
        pub fn new(p1: Point, p2: Point) -> Self {
            let min_x = p1.x.min(p2.x);
            let max_x = p1.x.max(p2.x);
            let min_y = p1.y.min(p2.y);
            let max_y = p1.y.max(p2.y);
            Self {
                p_tl: Point::new(min_x, min_y),
                p_br: Point::new(max_x, max_y),
            }
        }

        pub fn lower(&self) -> &Point {
            &self.p_tl
        }
        pub fn upper(&self) -> &Point {
            &self.p_br
        }

        pub fn height(&self) -> f64 {
            f64::abs(self.p_br.y - self.p_tl.y)
        }
        pub fn width(&self) -> f64 {
            f64::abs(self.p_br.x - self.p_tl.x)
        }

        pub fn area(&self) -> f64 {
            self.width() * self.height()
        }

        pub fn erosion(&mut self, d: f64) {
            self.p_tl.x = self.p_tl.x + d;
            self.p_tl.y = self.p_tl.y + d;
            self.p_br.x = self.p_br.x - d;
            self.p_br.y = self.p_br.y - d;
        }

        pub fn dilate(&mut self, d: f64) -> () {
            self.p_tl.x = self.p_tl.x - d;
            self.p_tl.y = self.p_tl.y - d;
            self.p_br.x = self.p_br.x + d;
            self.p_br.y = self.p_br.y + d;
        }
        pub fn intersection(&self, other: &Self) -> Self {
            let x1 = self.p_tl.x.max(other.p_tl.x);
            let y1 = self.p_tl.y.max(other.p_tl.y);
            let x2 = self.p_br.x.min(other.p_br.x);
            let y2 = self.p_br.y.min(other.p_br.y);

            if x1 > x2 || y1 > y2 {
                Square::new(Point::new(0.0, 0.0), Point::new(0.0, 0.0))
            } else {
                Square::new(Point::new(x1, y1), Point::new(x2, y2))
            }
        }

        pub fn union(&self, other: &Self) -> Self {
            let x1 = self.p_tl.x.min(other.p_tl.x);
            let y1 = self.p_tl.y.min(other.p_tl.y);
            let x2 = self.p_br.x.max(other.p_br.x);
            let y2 = self.p_br.y.max(other.p_br.y);
            Square::new(Point::new(x1, y1), Point::new(x2, y2))
        }

        pub fn dilate_x(&mut self, d: f64) -> () {
            let wth = self.width() * 0.5 * d;
            let mid_x = (self.p_br.x - self.p_tl.x) * 0.5;
            self.p_tl.x = mid_x - wth;
            self.p_br.x = mid_x + wth;
        }

        pub fn dilate_y(&mut self, d: f64) -> () {
            let wth = self.height() * 0.5 * d;
            let mid_y = (self.p_tl.y - self.p_tl.y) * 0.5;
            self.p_tl.y = mid_y + wth;
            self.p_br.y = mid_y - wth;
        }

        pub fn erosion_x(&mut self, d: f64) -> () {
            self.dilate_x(1.0 / d);
        }

        pub fn erosion_y(&mut self, d: f64) -> () {
            self.dilate_y(1.0 / d);
        }

        pub fn has_point(&self, p1: &Point) -> bool {
            todo!()
        }

        pub fn has_square(&self, sq: &Square) -> bool {
            if ((self.p_tl.x <= sq.p_br.x && sq.p_br.x <= self.p_br.x)
                || (self.p_tl.x <= sq.p_tl.x && sq.p_tl.x <= self.p_br.x))
                && ((self.p_tl.y <= sq.p_br.y && sq.p_br.y <= self.p_br.y)
                    || (self.p_tl.y <= sq.p_tl.y && sq.p_tl.y <= self.p_br.y))
                || ((sq.p_tl.x <= self.p_br.x && self.p_br.x <= sq.p_br.x)
                    || (sq.p_tl.x <= self.p_tl.x && self.p_tl.x <= sq.p_br.x))
                    && ((sq.p_tl.y <= self.p_br.y && self.p_br.y <= sq.p_br.y)
                        || (sq.p_tl.y <= self.p_tl.y && self.p_tl.y <= sq.p_br.y))
            {
                true
            } else {
                false
            }
        }

        pub fn manhattan_distance(&self, sq: &Square) -> f64 {
            todo!()
        }
    }
}

// filters the nodes if its geometry intersects with that of `base_node`
pub fn find_intersections(
    base_node: topology::Square,
    nodes: &Vec<ml_data::Node>,
) -> Vec<ml_data::Node> {
    let mut intersections = Vec::new();

    for node in nodes.iter() {
        let tmp_node_x = node.a["TP"].parse::<f64>().unwrap();
        let tmp_node_h = node.a["HT"].parse::<f64>().unwrap();
        let tmp_node_w = node.a["WH"].parse::<f64>().unwrap();
        let tmp_sqr = topology::Square::new(
            topology::Point::new(tmp_node_x, tmp_node_x),
            topology::Point::new(tmp_node_x + tmp_node_w, tmp_node_x + tmp_node_h),
        );
        if base_node.has_square(&tmp_sqr) {
            intersections.push(node.clone());
        }
    }
    intersections
}

#[cfg(test)]
mod test {
    use crate::find_intersections;
    use crate::ml_data::{read_ml_json, search_xx};
    use crate::topology::{Point, Square};
    use std::path::Path;

    #[test]
    fn point_test() {
        let p = Point::new(10.0, 10.0);
        assert_eq!(p.y(), 10.0);
        assert_eq!(p.x(), 10.0);
    }

    #[test]
    fn sq_test() {
        let p1: Point = Point::new(0.0, 0.0);
        let p2: Point = Point::new(1.0, 2.0);
        let sq: Square = Square::new(p1, p2);
        //assert_eq!(sq.area(),2.0);
        assert!(sq.lower().x() < sq.upper().x());
        assert!(sq.lower().y() < sq.upper().y());
    }

    #[test]
    fn dilate_test() {
        let p1: Point = Point::new(0.0, 2.0);
        let p2: Point = Point::new(1.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);

        sq.dilate(2.0);

        assert_eq!(sq.area(), 30.0);
    }

    #[test]
    fn intersection_test_corner() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, 1.0), Point::new(4.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 1.0);
        assert_eq!(s3.lower().y(), 1.0);
        assert_eq!(s3.upper().x(), 3.0);
        assert_eq!(s3.upper().y(), 3.0);
    }

    #[test]
    fn intersection_test_cross() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, -1.0), Point::new(2.0, 4.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 1.0);
        assert_eq!(s3.lower().y(), 0.0);
        assert_eq!(s3.upper().x(), 2.0);
        assert_eq!(s3.upper().y(), 3.0);
    }

    #[test]
    fn intersection_test_out() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(5.0, 5.0), Point::new(10.0, 10.0));
        let s3 = s1.intersection(&s2);
        assert_eq!(s3.lower().x(), 0.0);
        assert_eq!(s3.lower().y(), 0.0);
        assert_eq!(s3.upper().x(), 0.0);
        assert_eq!(s3.upper().y(), 0.0);
    }

    #[test]
    fn union_test_cross() {
        let s1: Square = Square::new(Point::new(0.0, 0.0), Point::new(3.0, 3.0));
        let s2: Square = Square::new(Point::new(1.0, -1.0), Point::new(2.0, 4.0));
        let s3 = s1.union(&s2);
        assert_eq!(s3.lower().x(), 0.0);
        assert_eq!(s3.lower().y(), -1.0);
        assert_eq!(s3.upper().x(), 3.0);
        assert_eq!(s3.upper().y(), 4.0);
    }

    #[test]
    fn erosion_test() {
        let p1: Point = Point::new(0.0, 4.0);
        let p2: Point = Point::new(4.0, 0.0);
        let mut sq: Square = Square::new(p1, p2);

        sq.erosion(0.5);

        assert_eq!(sq.area(), 9.0);
    }

    #[test]
    fn has_square_test() {
        let p1 = Point::new(0.0, 0.0);
        let p2 = Point::new(2.0, 2.0);
        let sq1 = Square::new(p1, p2);

        let p3 = Point::new(-1.0, -1.0);
        let p4 = Point::new(3.0, 3.0);
        let sq2 = Square::new(p3, p4);

        let intersection = sq1.has_square(&sq2);
        assert_eq!(intersection, true);
    }

    #[test]
    fn find_intersections_test() {
        let path = Path::new("resources/1645511997141_M8INRNFV6O_curr.json");
        let data = read_ml_json(&path);
        let node = search_xx(&data.element_statistics.nodes);
        let tmp_node_x = node.a["TP"].parse::<f64>().unwrap();
        let tmp_node_h = node.a["HT"].parse::<f64>().unwrap();
        let tmp_node_w = node.a["WH"].parse::<f64>().unwrap();
        let base_sqr = Square::new(
            Point::new(tmp_node_x, tmp_node_x),
            Point::new(tmp_node_x + tmp_node_w, tmp_node_x + tmp_node_h),
        );
        let intersections = find_intersections(base_sqr, &data.element_statistics.nodes);
        println!("Total intersections {}", intersections.len());
    }
}

pub fn display() {
    let p1 = Point::new(-3.0, -3.0);
    let p2 = Point::new(2.0, 0.0);

    let dist = p1.distance(&p2);

    println!("Distance between p1 and p2 is {}", dist);
    println!("Slope between p1 and p2 is {}", p1.slope_with(&p2));
    println!("Midpoint between p1 and p2 is {:?}", p1.mid_point(&p2));
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64
}

impl Point {
    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }

    fn distance(&self, second: &Self) -> f64 {
        let x_diff = self.x - second.x;
        let y_diff = self.y - second.y;

        (x_diff.powi(2) + y_diff.powi(2)).sqrt()
    }

    fn slope_with(&self, second: &Self) -> f64 {
        let rise = self.y - second.y;
        let run = self.x - second.x;

        rise / run
    }

    fn mid_point(&self, other: &Self) -> Self {
        let mid_x = (self.x + other.x) / 2.0;
        let mid_y = (self.y + other.y) / 2.0;

        Point::new(mid_x, mid_y)
    }
}




#[derive(Debug)]

pub struct Point{
    pub x: f64,
    pub y: f64,
}
impl Point{
    pub fn new(x_n: f64, y_n: f64) -> Self{
        Self{
            x: x_n,
            y: y_n,
        }
    }
    pub fn distance(&self, p2: &Point) -> f64{
        ((self.x - p2.x).powf(2.0) + (self.y - p2.y).powf(2.0)).sqrt()
    }
}

#[derive(Debug)]
pub struct Circle{
    pub center: Point,
    pub radius: f64,
}
impl Circle{
    pub fn new(x: f64, y: f64, rad: f64) -> Self{
        Self{
            center: Point::new(x, y),
            radius: rad,
        }
    }
    pub fn diameter(&self) -> f64{
        self.radius * 2.0
    }
    pub fn area(&self) -> f64{
        let pi = std::f64::consts::PI;
        pi * self.radius.powf(2.0)
    }
    pub fn intersect(&self, circle2: &Circle) -> bool{
        self.center.distance(&circle2.center) < self.radius + circle2.radius
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        
    }
}

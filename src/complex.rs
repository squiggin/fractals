use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Complex {
    x: f32,
    y: f32
}

impl ops::Add<Complex> for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Complex {
    pub fn new(x: f32, y: f32) -> Complex {
        Complex {
            x: x,
            y: y
        }
    }

    pub fn square(&self) -> Complex {
        Complex::new(self.x.powi(2) - self.y.powi(2), 2.0 * self.x * self.y)
    }

    pub fn abs(&self) -> f32 {
        (self.x.powi(2) + self.x.powi(2)).sqrt()
    }

    pub fn squared_abs(&self) -> f32 {
        self.x.powi(2) + self.x.powi(2)
    }
}
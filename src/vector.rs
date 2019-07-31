use std::ops;

#[derive(Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn zeros() -> Vector {
        Vector { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn ones() -> Vector {
        Vector { x: 1.0, y: 1.0, z: 1.0 }
    }

    pub fn unit(self) -> Vector {
        self / self.length()
    }

    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(self, other: Self) -> Vector {
        Vector {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn squared_length(self) -> f64 {
        self.dot(self)
    }

    pub fn length(self) -> f64 {
        self.dot(self).sqrt()
    }
}

impl ops::Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign for Vector {
    fn add_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vector {
    fn sub_assign(&mut self, other: Vector) {
        *self = Vector { 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl ops::Mul for Vector {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::MulAssign for Vector {
    fn mul_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Div for Vector {
    type Output = Vector;

    fn div(self, other: Vector) -> Vector {
        Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::DivAssign for Vector {
    fn div_assign(&mut self, other: Vector) {
        *self = Vector {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl ops::Neg for Vector {
    type Output = Vector;
    
    fn neg(self) -> Vector {
        Vector { x: -self.x, y: -self.y, z: -self.z }
    }
}

impl ops::Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
        }
    }
}

impl ops::MulAssign<f64> for Vector {
    fn mul_assign(&mut self, other: f64) {
        *self = Vector {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl ops::DivAssign<f64> for Vector {
    fn div_assign(&mut self, other: f64) {
        *self = Vector {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}
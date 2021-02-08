use std::ops;

// 3D vector
#[derive(Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

// vec1 + vec2
impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self{x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

// vec1 - vec2
impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self{x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

// vec1 * vec2
impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self{x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}


// vec1 * c
impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, c: f64) -> Self {
        Self{x: self.x * c, y: self.y * c, z: self.z * c}
    }
}

// vec1 / c
impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, c: f64) -> Self {
        if c == 0.0 {
          panic!("Cannot divide by 0!");
        }

        Self{x: self.x / c, y: self.y / c, z: self.z / c}
    }

}

trait Math3D {
    fn neg(&self) -> Self;
    fn length_squared(&self) -> f64;
    fn length(&self) -> f64;
    fn dot(Self, Self) ->f64;
    fn cross(Self, Self) -> Self;
    fn unit(Self) -> Self;
}

impl Math3D for Vec3 {
    fn neg(&self) -> Vec3{
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }

    fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    fn length(&self) -> f64 {
        self.length_squared().sqrt()    
    }

    fn dot(v1: Vec3, v2: Vec3) -> f64 {
       v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
    }

    fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3{
            x: v1.y * v2.z - v1.z * v2.y,
            y: v1.z * v2.x - v1.x * v2.z, 
            z: v1.x * v2.y - v1.y * v2.x }
    }

    fn unit(v: Vec3) -> Vec3 {
        v / v.length()
    }
}

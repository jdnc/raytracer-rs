use vec3::Vec3;


#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

trait Move {
    fn at(&self, t: f64) -> Vec3;
}

impl Move for Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.origin + self.dir * t
    }
}

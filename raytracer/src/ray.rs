use crate::Vec3;

#[derive(PartialEq, Debug)]
pub struct Ray {
    pub o: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            o: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.o
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }
    
    pub fn at(&self, t: f32) -> Vec3 {
        self.o + self.dir * t
    }
}


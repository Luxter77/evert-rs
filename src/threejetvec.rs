use crate::threejet::ThreeJet;
use crate::twojetvec::TwoJetVec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThreeJetVec {
  x: ThreeJet,
  y: ThreeJet,
  z: ThreeJet,
}

impl From<TwoJetVec> for ThreeJetVec {
    fn from(src: TwoJetVec) -> ThreeJetVec {
        ThreeJetVec::new(src.x().into(), src.y().into(), src.z().into()) 
    }
}

impl std::ops::Add<ThreeJetVec> for ThreeJetVec {
    type Output = Self;
    fn add(self, rhs: ThreeJetVec) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::AddAssign<ThreeJetVec> for ThreeJetVec {
    fn add_assign(&mut self, rhs: ThreeJetVec) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Mul<ThreeJet> for ThreeJetVec {
    type Output = Self;
    fn mul(self, rhs: ThreeJet) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::MulAssign<ThreeJet> for ThreeJetVec {
    fn mul_assign(&mut self, rhs: ThreeJet) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Add<f64> for ThreeJetVec {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x + rhs,
            y: self.y + rhs,
            z: self.z + rhs,
        }
    }
}
impl std::ops::AddAssign<f64> for ThreeJetVec {
    fn add_assign(&mut self, rhs: f64) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
    }
}


impl std::ops::Mul<f64> for ThreeJetVec {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl std::ops::MulAssign<f64> for ThreeJetVec {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ThreeJetVec {
    pub fn new(x: ThreeJet, y: ThreeJet, z: ThreeJet) -> Self {
        Self { x, y, z }
    }
    #[allow(unused)]
    pub fn zero() -> Self {
        Self {
            x: ThreeJet::zero(),
            y: ThreeJet::zero(),
            z: ThreeJet::zero(),
        }
    }
    pub fn x(&self) -> ThreeJet { self.x }
    pub fn y(&self) -> ThreeJet { self.y }
    pub fn z(&self) -> ThreeJet { self.z }
    #[allow(unused)]
    pub fn anihilate(&mut self, index: i32) {
        self.x.annihilate(index);
        self.y.annihilate(index);
        self.z.annihilate(index);
    }
    pub fn anihilated(&self, index: i32) -> Self {
        Self {
            x: self.x.annihilated(index),
            y: self.y.annihilated(index),
            z: self.z.annihilated(index),
        }
    }
    pub fn d(&self, index: i32) -> TwoJetVec {
        TwoJetVec::new(self.x.d(index), self.y.d(index), self.z.d(index))
    }
    #[allow(unused)]
    pub fn cross(&mut self, rhs: Self) {
        self.x = self.y * rhs.z + self.z * rhs.y * -1.0;
        self.y = self.z * rhs.x + self.x * rhs.z * -1.0;
        self.z = self.x * rhs.y + self.y * rhs.x * -1.0;
    }
    #[allow(unused)]
    pub fn crossed(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z + self.z * rhs.y * -1.0,
            y: self.z * rhs.x + self.x * rhs.z * -1.0,
            z: self.x * rhs.y + self.y * rhs.x * -1.0,
        }
    }
    #[allow(unused)]
    pub fn dot(&self, rhs: Self) -> ThreeJet {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    #[allow(unused)]
    pub fn normalize(&mut self) {
        let mut a: ThreeJet = self.dot(*self);
        if a > 0.0 {
            a ^= -0.5;
        } else {
            a = ThreeJet::zero();
        };
        *self *= a;
    }
    #[allow(unused)]
    pub fn normalized(&self) -> Self {
        let mut a: ThreeJet = self.dot(*self);
        if a > 0.0 {
            a ^= -0.5;
        } else {
            a = ThreeJet::zero();
        };
        return *self * a;
    }
    #[allow(unused)]
    pub fn rotate_z(&mut self, angle: ThreeJet) {
        let s: ThreeJet = angle.sin();
        let c: ThreeJet = angle.cos();
        self.x = self.x * c + self.y * s;
        self.y = self.x * s * -1.0 + self.y * c;
    }
    #[allow(unused)]
    pub fn rotated_z(&self, angle: ThreeJet) -> Self {
        let s: ThreeJet = angle.sin();
        let c: ThreeJet = angle.cos();
        Self {
            x: self.x * c + self.y * s,
            y: self.x * s * -1.0 + self.y * c,
            z: self.z,
        }
    }
    #[allow(unused)]
    pub fn rotate_y(&mut self, angle: ThreeJet) {
        let s: ThreeJet = angle.sin();
        let c: ThreeJet = angle.cos();
        self.x = self.x * c + self.z * s * -1.0;
        self.z = self.x * s + self.z * c;
    }
    #[allow(unused)]
    pub fn rotated_y(&self, angle: ThreeJet) -> Self {
        let s: ThreeJet = angle.sin();
        let c: ThreeJet = angle.cos();
        Self {
            x: self.x * c + self.z * s * -1.0,
            y: self.y,
            z: self.x * s + self.z * c,
        }
    }
    #[allow(unused)]
    pub fn rotate_x(&mut self, angle: ThreeJet) {
        let s: ThreeJet = angle.sin();
        let c: ThreeJet = angle.cos();
        self.y = self.y * c + self.z * s;
        self.z = self.y * s * -1.0 + self.z * c;
    }
    #[allow(unused)]
    pub fn rotated_x(&self, angle: ThreeJet) -> Self {
        let s: ThreeJet = angle.sin();
        let c: ThreeJet = angle.cos();
        Self {
            x: self.x,
            y: self.y * c + self.z * s,
            z: self.y * s * -1.0 + self.z * c,
        }
    }
    #[allow(unused)]
    pub fn interpolate(&mut self, rhs: Self, weight: ThreeJet) {
        *self *= (weight * -1.0) + 1.0;
        *self += rhs * weight;
    }
    pub fn interpolated(&self, rhs: Self, weight: ThreeJet) -> ThreeJetVec {
        ((*self) * ((weight * -1.0) + 1.0)) + (rhs * weight)
    }
    #[allow(unused)]
    pub fn lenght(&self) -> ThreeJet {
        ((self.x ^ 2.0) + (self.y ^ 2.0)) ^ (0.5)
    }
}

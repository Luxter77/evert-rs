use crate::{twojet::TwoJet, points::{SplinePoint, BrezierPoint}, threejetvec::ThreeJetVec, spline::PrintableSpline};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoJetVec {
    x: TwoJet,
    y: TwoJet,
    z: TwoJet,
}

impl From<ThreeJetVec> for TwoJetVec {
    fn from(src: ThreeJetVec) -> Self {
        Self {
            x: src.x().into(),
            y: src.y().into(),
            z: src.z().into(),
        }
    }
}

impl std::ops::Add<TwoJetVec> for TwoJetVec {
    type Output = TwoJetVec;
    fn add(self, rhs: TwoJetVec) -> TwoJetVec {
        TwoJetVec {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign<TwoJetVec> for TwoJetVec {
    fn add_assign(&mut self, rhs: TwoJetVec) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
    }
}

impl std::ops::Mul<TwoJetVec> for TwoJetVec {
    type Output = TwoJetVec;
    fn mul(self, rhs: TwoJetVec) -> TwoJetVec {
        TwoJetVec {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::MulAssign<TwoJetVec> for TwoJetVec {
    fn mul_assign(&mut self, rhs: TwoJetVec) {
        self.x = self.x * rhs.x;
        self.y = self.y * rhs.y;
        self.z = self.z * rhs.z;
    }
}

impl std::ops::Mul<TwoJet> for TwoJetVec {
    type Output = TwoJetVec;
    fn mul(self, rhs: TwoJet) -> TwoJetVec {
        TwoJetVec {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<TwoJet> for TwoJetVec {
    fn mul_assign(&mut self, rhs: TwoJet) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl std::ops::Mul<f64> for TwoJetVec {
    type Output = TwoJetVec;
    fn mul(self, rhs: f64) -> TwoJetVec {
        TwoJetVec {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl std::ops::MulAssign<f64> for TwoJetVec {
    fn mul_assign(&mut self, rhs: f64) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
        self.z = self.z * rhs;
    }
}

impl TwoJetVec {
    pub fn new(x: TwoJet, y: TwoJet, z: TwoJet) -> Self {
        Self { x, y, z }
    }
    pub fn zero() -> Self {
        Self {
            x: TwoJet::zero(),
            y: TwoJet::zero(),
            z: TwoJet::zero(),
        }
    }
    pub fn dot(self, rhs: Self) -> TwoJet {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    #[allow(dead_code)]
    pub fn annihilate(&mut self, index: i32) {
        self.x.annihilate(index);
        self.y.annihilate(index);
        self.z.annihilate(index);
    }
    pub fn annihilated(&self, index: i32) -> Self {
        Self {
            x: self.x.annihilated(index), 
            y: self.y.annihilated(index), 
            z: self.z.annihilated(index), 
        }
    }
    #[allow(dead_code)]
    pub fn cross(&mut self, rhs: Self) {
        self.x = self.y * rhs.z + self.z * rhs.y * -1.0;
        self.y = self.z * rhs.x + self.x * rhs.z * -1.0;
        self.z = self.x * rhs.y + self.y * rhs.x * -1.0;
    }
    pub fn crossed(&self, rhs: Self) -> Self {
        Self {
            x: self.y * rhs.z + self.z * rhs.y * -1.0,
            y: self.z * rhs.x + self.x * rhs.z * -1.0,
            z: self.x * rhs.y + self.y * rhs.x * -1.0,
        }
    }
    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        let mut a: TwoJet = self.dot(*self);
        if a > 0.0 {
            a ^= -0.5;
        } else {
            a = TwoJet::zero();
        };
        *self *= a;
    }
    pub fn normalized(&self) -> Self {
        let mut a: TwoJet = self.dot(*self);
        if a > 0.0 {
            a ^= -0.5;
        } else {
            a = TwoJet::zero();
        };
        return *self * a;
    }
    #[allow(dead_code)]
    pub fn rotate_z(&mut self, angle: TwoJet) {
        let s: TwoJet = angle.sin();
        let c: TwoJet = angle.cos();
        self.x = self.x * c + self.y * s;
        self.y = self.x * s * -1.0 + self.y * c;
    }
    pub fn rotated_z(&self, angle: TwoJet) -> Self {
        let s: TwoJet = angle.sin();
        let c: TwoJet = angle.cos();
        Self {
            x: self.x * c + self.y * s,
            y: self.x * s * -1.0 + self.y * c,
            z: self.z,
        }
    }
    #[allow(dead_code)]
    pub fn rotate_y(&mut self, angle: TwoJet) {
        let s: TwoJet = angle.sin();
        let c: TwoJet = angle.cos();
        self.x = self.x * c + self.z * s * -1.0;
        self.z = self.x * s + self.z * c;
    }
    #[allow(dead_code)]
    pub fn rotated_y(&self, angle: TwoJet) -> Self {
        let s: TwoJet = angle.sin();
        let c: TwoJet = angle.cos();
        Self {
            x: self.x * c + self.z * s * -1.0,
            y: self.y,
            z: self.x * s + self.z * c,
        }
    }
    #[allow(dead_code)]
    pub fn rotate_x(&mut self, angle: TwoJet) {
        let s: TwoJet = angle.sin();
        let c: TwoJet = angle.cos();
        self.y = self.y * c + self.z * s;
        self.z = self.y * s * -1.0 + self.z * c;
    }
    #[allow(dead_code)]
    pub fn rotated_x(&self, angle: TwoJet) -> Self {
        let s: TwoJet = angle.sin();
        let c: TwoJet = angle.cos();
        Self {
            x: self.x,
            y: self.y * c + self.z * s,
            z: self.y * s * -1.0 + self.z * c,
        }
    }
    #[allow(dead_code)]
    pub fn interpolate(&mut self, rhs: Self, weight: TwoJet) {
        *self *= (weight * -1.0) + 1.0;
        *self += rhs * weight;
    }
    #[allow(dead_code)]
    pub fn interpolated(&self, rhs: Self, weight: TwoJet) -> TwoJetVec {
        ((*self) * ((weight * -1.0) + 1.0)) + (rhs * weight)
    }
    #[allow(dead_code)]
    pub fn lenght(&self) -> TwoJet {
        ((self.x ^ 2.0) + (self.y ^ 2.0)) ^ (0.5)
    }
    pub fn brezier_point(&self, ps: f64, pus: f64, pvs: f64, puvs: f64) -> BrezierPoint {
        BrezierPoint::new(
			self.x.brezier_dim(ps, pus, pvs, puvs),
			self.y.brezier_dim(ps, pus, pvs, puvs),
			self.z.brezier_dim(ps, pus, pvs, puvs),
        )
    }
    pub fn point(&self, ps: Option<f64>) -> SplinePoint {
        let x:	f64 = Into::<f64>::into(self.x()) * ps.unwrap_or(1.0);
 		let y:	f64 = Into::<f64>::into(self.y()) * ps.unwrap_or(1.0);
 		let z:	f64 = Into::<f64>::into(self.z()) * ps.unwrap_or(1.0);
 		let nx:	f64 = self.y.fu() * self.z.fv() - self.z.fu() * self.y.fv();
 		let ny:	f64 = self.z.fu() * self.x.fv() - self.x.fu() * self.z.fv();
 		let nz:	f64 = self.x.fu() * self.y.fv() - self.y.fu() * self.x.fv();
 		let mut s:	f64 = nx * nx + ny * ny + nz * nz;
 		if s > 0.0 { s = (1.0 / s).sqrt(); };
        return SplinePoint::new(x, y, z, nx, ny, nz, s);
    }
    pub fn x(&self) -> TwoJet { self.x }
    pub fn y(&self) -> TwoJet { self.y }
    pub fn z(&self) -> TwoJet { self.z }
    #[inline]
    pub fn calc_speed_v(&self) -> f64 {  self.x.fv().powi(2) + self.y.fv().powi(2) + self.z.fv().powi(2) }
    #[inline]
    pub fn calc_speed_u(&self) -> f64 { (self.x.fu().powi(2) + self.y.fu().powi(2) + self.z.fu().powi(2)).sqrt() }
}

impl PrintableSpline for TwoJetVec {
    fn print_spline(&self, v01: TwoJetVec,  v10: TwoJetVec, v11: TwoJetVec, us: f64, vs: f64, s0: f64, s1: f64, t0: f64, t1: f64) {
        let v00: &Self = self;
        if crate::nstrip::EasyAtomic::get(&crate::nstrip::BINARY) {
            let magic: [BrezierPoint; 16] = [
                v00.brezier_point(1.0, 0.0, 0.0, 0.0),
                v00.brezier_point(1.0,  us, 0.0, 0.0),
                v10.brezier_point(1.0, -us, 0.0, 0.0),
                v10.brezier_point(1.0, 0.0, 0.0, 0.0),
                v00.brezier_point(1.0, 0.0,  vs, 0.0),
                v00.brezier_point(1.0,  us, 	vs,  us * vs),
                v10.brezier_point(1.0, -us,  vs, -us * vs),
                v10.brezier_point(1.0, 0.0,  vs, 0.0),
                v01.brezier_point(1.0, 0.0, -vs, 0.0),
                v01.brezier_point(1.0,  us, -vs, -us * vs),
                v11.brezier_point(1.0, -us, -vs,  us * vs),
                v11.brezier_point(1.0, 0.0, -vs, 0.0),
                v01.brezier_point(1.0, 0.0, 0.0, 0.0),
                v01.brezier_point(1.0,  us, 0.0, 0.0),
                v11.brezier_point(1.0, -us, 0.0, 0.0),
                v11.brezier_point(1.0, 0.0, 0.0, 0.0),
            ];
            if crate::nstrip::EasyAtomic::get(&crate::nstrip::BINARY) {
                for bp in magic { std::io::Write::write(&mut std::io::stdout(), &bp.as_bytes()).unwrap(); };
                println!()
            } else {
                for bp in magic { println!("{}", bp.g()) };
                println!("{s0} {t0}  {s1} {t0}  {s0} {t1}  {s1} {t1}\n", s0=s0, t0=t0, s1=s1, t1=t1);
            }
        } else {
            let magic: [SplinePoint; 4] = [ v00.point(None), v10.point(None), v11.point(None), v01.point(None) ];
            for sp in magic { println!("{}", sp) };
            if !crate::nstrip::EasyAtomic::get(&crate::nstrip::BINARY) { println!() };
        }
    }
}
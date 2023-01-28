#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TwoJet {
    pub f:   f64,
    pub fu:  f64,
    pub fv:  f64,
    pub fuv: f64,
}

impl Into<f64> for TwoJet { fn into(self) -> f64 { self.f } }

impl std::ops::Add<TwoJet> for TwoJet {
    type Output = Self;
    fn add(self, _rhs: Self) -> Self::Output {
        return Self {
            f:   self.f   + _rhs.f,
            fu:  self.fu  + _rhs.fu,
            fv:  self.fv  + _rhs.fv,
            fuv: self.fuv + _rhs.fuv,
        };
    }

}

impl std::ops::AddAssign<TwoJet> for TwoJet {
    fn add_assign(&mut self, rhs: TwoJet) {
        self.f   = self.f   + rhs.f;
        self.fu  = self.fu  + rhs.fu;
        self.fv  = self.fv  + rhs.fv;
        self.fuv = self.fuv + rhs.fuv;
    }
}

impl std::ops::Mul<TwoJet> for TwoJet {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        return Self {
            f:   self.f * rhs.f,
            fu:  self.f * rhs.fu  + self.fu * rhs.f,
            fv:  self.f * rhs.fv  + self.fv * rhs.f,
            fuv: self.f * rhs.fuv + self.fu * rhs.fv + self.fv * rhs.fu + self.fuv * rhs.f
        }
    }
}

impl std::ops::MulAssign<TwoJet> for TwoJet {
    fn mul_assign(&mut self, rhs: Self) {
        self.f    = self.f * rhs.f;
        self.fv   = self.f * rhs.fv  + self.fv * rhs.f;
        self.fu   = self.f * rhs.fu  + self.fu * rhs.f;
        self.fuv  = self.f * rhs.fuv + self.fu * rhs.fv + self.fv * rhs.fu + self.fuv * rhs.f;
    }
}

impl std::ops::Add<f64> for TwoJet {
    type Output = Self;
    fn add(self, _rhs: f64) -> Self::Output {
        return Self {
            f:   self.f   + _rhs,
            fu:  self.fu  + _rhs,
            fv:  self.fv  + _rhs,
            fuv: self.fuv + _rhs,
        };
    }
}

impl std::ops::AddAssign<f64> for TwoJet {
    fn add_assign(&mut self, rhs: f64) {
        self.f = self.f + rhs;
    }
}

impl std::ops::Mul<f64> for TwoJet {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        return TwoJet {
            f:   self.f   * rhs,
            fu:  self.fu  * rhs,
            fv:  self.fv  * rhs,
            fuv: self.fuv * rhs,
        }
    }
}

impl std::ops::MulAssign<f64> for TwoJet {
    fn mul_assign(&mut self, rhs: f64) {
        self.f   = self.f   * rhs;
        self.fu  = self.fu  * rhs;
        self.fv  = self.fv  * rhs;
        self.fuv = self.fuv * rhs;
    }
}

impl std::ops::BitXor<f64> for TwoJet {
    type Output = Self;
    fn bitxor(self, rhs: f64) -> Self::Output {
        let (x0, x1, x2): (f64, f64, f64);
        
        x0 = self.f.powf(rhs);
        
        if self.f == 0.0 {
            (x1, x2) = (0.0, 0.0);
        } else {
            x1 =  rhs        * x0 / self.f;
            x2 = (rhs - 1.0) * x1 / self.f;
        }

        return Self {
            f:   x0,
            fu:  x1 * self.fu,
            fv:  x1 * self.fv,
            fuv: x1 * self.fuv + x2 * self.fu * self.fv,
        }
    }
}

impl std::ops::BitXorAssign<f64> for TwoJet {
    fn bitxor_assign(&mut self, rhs: f64) {
        let (x0, x1, x2): (f64, f64, f64);
        
        x0 = self.f.powf(rhs);
        
        if self.f == 0.0 {
            (x1, x2) = (0.0, 0.0);
        } else {
            x1 =  rhs        * x0 / self.f;
            x2 = (rhs - 1.0) * x1 / self.f;
        };

        self.f   = x0;
        self.fu  = x1 * self.fu;
        self.fv  = x1 * self.fv;
        self.fuv = x1 * self.fuv + x2 * self.fu * self.fv;
    }
}

impl std::ops::Rem<f64> for TwoJet {
    type Output = Self;
    fn rem(self, rhs: f64) -> Self {
        let mut o: Self = self.clone();
        o.f %= rhs;
        if o.f < 0.0 {
            o.f = o.f + rhs;
        };
        return o;
    }
}

impl std::ops::RemAssign<f64> for TwoJet {
    fn rem_assign(&mut self, rhs: f64) {
        self.f = self.f % rhs;
        if self.f < 0.0 {
            self.f = self.f + rhs;
        };
    }
}

impl std::cmp::PartialEq<f64> for TwoJet {
    fn eq(&self, other: &f64) -> bool { self.f == *other }
    fn ne(&self, other: &f64) -> bool { self.f != *other }
}

impl std::cmp::PartialOrd<f64> for TwoJet {
    fn ge(&self, other: &f64) -> bool { self.f >= *other }
    fn gt(&self, other: &f64) -> bool { self.f >  *other }
    fn le(&self, other: &f64) -> bool { self.f <= *other }
    fn lt(&self, other: &f64) -> bool { self.f <  *other }
    fn partial_cmp(&self, other: &f64) -> Option<std::cmp::Ordering> {
        return self.f.partial_cmp(&other);
    }
}

impl TwoJet {
    pub fn new(d: f64, du: f64, dv: f64, duv: Option<f64>) -> Self {
        return Self {
            f: d, fu: du, fv: dv,
            fuv: match duv {
                Some(duv) => { duv },
                None => { 0.0 },
            },
        }
    }
    pub fn zero() -> Self {
        Self { f: 0.0, fu: 0.0, fv: 0.0, fuv: 0.0 }
    }
    #[inline]
    pub fn f(&self) -> f64 {
        self.f.clone()
    }
    #[inline]
    pub fn fu(&self) -> f64 {
        self.fu.clone()
    }
    #[inline]
    pub fn fv(&self) -> f64 {
        self.fv.clone()
    }
    #[inline]
    pub fn fuv(&self) -> f64 {
        self.fuv.clone()
    }
    pub fn annihilated(&self, index: i32) -> Self {
        let mut o: Self = self.clone();
        match index {
          0 => { o.fu = 0.0 },
          1 => { o.fv = 0.0 },
          _ => { /* ???? */ },
        };       o.fuv = 0.0;
        return o;
    }
    pub fn annihilate(&mut self, index: i32) {
        match index {
          0 => { self.fu = 0.0 },
          1 => { self.fv = 0.0 },
          _ => { /* ??????? */ },
        };
        self.fuv = 0.0;
    }
    pub fn interpolate(&mut self, rhs: Self, weight: Self) {
        *self *= (weight * -1.0) + 1.0;
        *self += rhs * weight;
    }
    pub fn interpolated(&self, rhs: Self, weight: Self) -> Self {
        ((*self) * ((weight * -1.0) + 1.0)) + (rhs * weight)
    }
    pub fn take_sin(&mut self) {
        *self *= 2.0 * std::f64::consts::PI;
        
        let s: f64 =  self.f.sin();
        let c: f64 =  self.f.cos();
        
        self.f   = s;
        self.fu  = self.fu  * c;
        self.fv  = self.fv  * c;
        self.fuv = self.fuv * c - self.fu * self.fv * s;
    }
    pub fn take_cos(&mut self) {
        *self *= 2.0 * std::f64::consts::PI;
        
        let s: f64 =  self.f.cos();
        let c: f64 = -self.f.sin();

        self.f   = s;
        self.fu  = self.fu  * c;
        self.fv  = self.fv  * c;
        self.fuv = self.fuv * c - self.fu * self.fv * s;
    }
    pub fn sin(&self) -> Self {
        let t: Self = (*self) * 2.0 * std::f64::consts::PI;
        let (s, c): (f64, f64) = (t.f.sin(), t.f.cos());
        return Self::new(s, c * t.fu, c * t.fv, Option::Some(c * t.fuv - s * t.fu * t.fv));
    }
    pub fn cos(&self) -> Self {
        let f: f64 = self.f * 2.0 * std::f64::consts::PI;
        
        let s: f64 =  f.cos();
        let c: f64 = -f.sin();

        return Self {
            f:   s,
            fu:  self.fu  * c,
            fv:  self.fv  * c,
            fuv: self.fuv * c - self.fu * self.fv * s,
        }
    }
    pub fn print(&self) {   
        println!("{f} ({fu} {fv})", f=self.f, fu=self.fu, fv=self.fv);
    }
    pub fn brezier_dim(&self, ps: f64, pus: f64, pvs: f64, puvs: f64) -> f64 {
        Into::<f64>::into(*self) * ps + self.fu() * pus / 3.0 + self.fv() * pvs / 3.0 + self.fuv() * puvs / 9.0
    }
}

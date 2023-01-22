#[derive(Debug, Clone, Copy)]
pub struct TwoJet {
    pub f:   f64,
    pub fu:  f64,
    pub fv:  f64,
    pub fuv: f64,
}

impl From<TwoJet> for f64 { fn from(s: TwoJet) -> f64 { return s.f; } }

impl std::ops::Add<TwoJet> for TwoJet {
    fn add(&self, _rhs: TwoJet) -> TwoJet {
        return TwoJet {
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
    fn mul(&self, rhs: TwoJet) -> TwoJet {
        return TwoJet {
            f:   self.f * rhs.f,
            fu:  self.f * rhs.fu  + self.fu * rhs.f,
            fv:  self.f * rhs.fv  + self.fv * rhs.f,
            fuv: self.f * rhs.fuv + self.fu * rhs.fv + self.fv * rhs.fu + self.fuv * rhs.f
        }
    }
}

impl std::ops::MulAssign<TwoJet> for TwoJet {
    fn mul_assign(&mut self, rhs: TwoJet) {
        self.f    = self.f * rhs.f;
        self.fv   = self.f * rhs.fv  + self.fv * rhs.f;
        self.fu   = self.f * rhs.fu  + self.fu * rhs.f;
        self.fuv  = self.f * rhs.fuv + self.fu * rhs.fv + self.fv * rhs.fu + self.fuv * rhs.f;
    }
}

impl std::ops::Add<f64> for TwoJet {
    fn add(&self, _rhs: f64) -> TwoJet {
        return TwoJet {
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
    fn mul(&self, rhs: TwoJet) -> TwoJet {
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
        self.dv  = self.dv  * rhs;
        self.fuv = self.fuv * rhs;
    }
}

impl std::ops::BitXor<f64> for TwoJet {
    fn bitxor(&self, rhs: f64) -> TwoJet {
        let (x0, x1, x2): (f64, f64, f64);
        let x0: f64 = self.f.powf(rhs);
        
        if self.f == 0.0 {
            (x1, x2) = (0.0, 0.0);
        } else {
            x1 =  rhs      * x0 / self.f;
            x2 = (rhs - 1) * x1 / self.f;
        }

        return TwoJet {
            f:   x0,
            fu:  x1 * self.fu,
            fv:  x1 * self.fv,
            fuv: x1 * self.fuv + x2 * self.fu * self.fv,
        }
    }
}

impl std::cmp::PartialOrd<f64> for TwoJet {
    fn ge(&self, other: &f64) -> bool { self.f >= other }
    fn gt(&self, other: &f64) -> bool { self.f >  other }
    fn le(&self, other: &f64) -> bool { self.f <= other }
    fn lt(&self, other: &f64) -> bool { self.f <  other }
}

impl std::ops::RemAssign<f64> for TwoJet {
    fn rem(&self, rhs: f64) {
        self.f = f % rhs;
        if (self.f < 0) { self.f = self.f + rhs; };
    }   
}

impl std::ops::BitXorAssign<f64> for TwoJet {
    fn bitxor(&self, rhs: f64) {
        let (x0, x1, x2): (f64, f64, f64);
        let x0: f64 = self.f.powf(rhs);
        
        if self.f == 0.0 {
            (x1, x2) = (0.0, 0.0);
        } else {
            x1 =  rhs      * x0 / self.f;
            x2 = (rhs - 1) * x1 / self.f;
        };

        self.f   = x0;
        self.fu  = x1 * self.fu;
        self.fv  = x1 * self.fv;
        self.fuv = x1 * self.fuv + x2 * self.fu * self.fv;
    }
}

impl TwoJet {
    fn new(d: f64, du: f64, dv: f64, duv: Option<f64>) -> TwoJet {
        return TwoJet {
            f:   d,
            fu:  du,
            fv:  dv,
            fuv: match duv {
                Some(duv) => { duv },
                None => { 0.0 },
            },
        }
    }
    fn Annihilate(&mut self, index: i32) {
        match index {
          0 => { self.fu = 0 },
          1 => { self.fv = 0 },
          _ => { /* ????? */ },
        };
        self.fuv = 0;
    }
    fn TakeSin(&mut self) {
        self.f *= 2.0 * std::f64::consts::PI;
        
        let s: f64 =  self.f.sin();
        let c: f64 =  self.f.cos();
        
        self.f   = s;
        self.fu  = self.fu  * c;
        self.fv  = self.fv  * c;
        self.fuv = self.fuv * c - self.fu * self.fv * s;
    }
    fn TakeCos(&mut self) {
        self.f *= 2 * std::f64::consts::PI;
        
        let s: f64 =  self.f.cos();
        let c: f64 = -self.f.sin();

        self.f   = s;
        self.fu  = self.fu  * c;
        self.fv  = self.fv  * c;
        self.fuv = self.fuv * c - self.fu * self.fv * s;
    }
    fn sin(&self) -> TwoJet {
        let t: TwoJet = self * 2.0 * std::f64::consts::PI;
        let (s, c): (f64, f64) = (t.f.sin(), t.f.cos());
        return Self::new(s, c * t.fu, c * t.fv, Option::Some(c * t.fuv - s * t.fu * t.fv));
    }
    fn df_du(&self)    -> f64 { return self.fu;  }
    fn df_dv(&self)    -> f64 { return self.fv;  }
    fn d2f_dudv(&self) -> f64 { return self.fuv; }
}

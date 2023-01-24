use crate::twojet::TwoJet;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ThreeJet {
    f:    f64,
    fu:   f64,
    fv:   f64,
    fuu:  f64,
    fuv:  f64,
    fvv:  f64,
    fuuv: f64,
    fuvv: f64,
}

impl std::ops::Add<ThreeJet> for ThreeJet {
    type Output = Self;
    fn add(self, rhs: ThreeJet) -> Self::Output {
        Self {
            f:    self.f    + rhs.f,
            fu:   self.fu   + rhs.fu,
            fv:   self.fv   + rhs.fv,
            fuu:  self.fuu  + rhs.fuu,
            fuv:  self.fuv  + rhs.fuv,
            fvv:  self.fvv  + rhs.fvv,
            fuuv: self.fuuv + rhs.fuuv,
            fuvv: self.fuvv + rhs.fuvv,
        }
    }
}

impl std::ops::AddAssign<ThreeJet> for ThreeJet {
    fn add_assign(&mut self, rhs: ThreeJet) {
        self.f    = self.f    + rhs.f;
        self.fu   = self.fu   + rhs.fu;
        self.fv   = self.fv   + rhs.fv;
        self.fuu  = self.fuu  + rhs.fuu;
        self.fuv  = self.fuv  + rhs.fuv;
        self.fvv  = self.fvv  + rhs.fvv;
        self.fuuv = self.fuuv + rhs.fuuv;
        self.fuvv = self.fuvv + rhs.fuvv;
    }
}

impl std::ops::Mul<ThreeJet> for ThreeJet {
    type Output = Self;
    fn mul(self, rhs: ThreeJet) -> Self::Output {
        Self {
            f:    self.f * rhs.f,
            fu:   self.f * rhs.fu   +       self.fu * rhs.f,
            fv:   self.f * rhs.fv   +       self.fv * rhs.f,
            fuu:  self.f * rhs.fuu  + 2.0 * self.fu * rhs.fu  + self.fuu * rhs.f,
            fuv:  self.f * rhs.fuv  +       self.fu * rhs.fv  + self.fv  * rhs.fu  +       self.fuv * rhs.f,
            fvv:  self.f * rhs.fvv  + 2.0 * self.fv * rhs.fv  + self.fvv * rhs.f,
            fuuv: self.f * rhs.fuuv + 2.0 * self.fu * rhs.fuv + self.fv  * rhs.fuu + 2.0 * self.fuv * rhs.fu + self.fuu * rhs.fv + self.fuuv * rhs.f,
            fuvv: self.f * rhs.fuvv + 2.0 * self.fv * rhs.fuv + self.fu  * rhs.fvv + 2.0 * self.fuv * rhs.fv + self.fvv * rhs.fu + self.fuvv * rhs.f,
        }
    }
}

impl std::ops::MulAssign<ThreeJet> for ThreeJet {
    fn mul_assign(&mut self, rhs: ThreeJet) {
        self.f    = self.f * rhs.f;
        self.fu   = self.f * rhs.fu   +       self.fu * rhs.f;
        self.fv   = self.f * rhs.fv   +       self.fv * rhs.f;
        self.fuu  = self.f * rhs.fuu  + 2.0 * self.fu * rhs.fu  + self.fuu * rhs.f;
        self.fuv  = self.f * rhs.fuv  +       self.fu * rhs.fv  + self.fv  * rhs.fu  +       self.fuv * rhs.f;
        self.fvv  = self.f * rhs.fvv  + 2.0 * self.fv * rhs.fv  + self.fvv * rhs.f;
        self.fuuv = self.f * rhs.fuuv + 2.0 * self.fu * rhs.fuv + self.fv  * rhs.fuu + 2.0 * self.fuv * rhs.fu + self.fuu * rhs.fv + self.fuuv * rhs.f;
        self.fuvv = self.f * rhs.fuvv + 2.0 * self.fv * rhs.fuv + self.fu  * rhs.fvv + 2.0 * self.fuv * rhs.fv + self.fvv * rhs.fu + self.fuvv * rhs.f;
    }
}

impl std::ops::Add<f64> for ThreeJet {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let mut o: Self = self.clone();
        o.f += rhs;
        return o;
    }
}

impl std::ops::AddAssign<f64> for ThreeJet {
    fn add_assign(&mut self, rhs: f64) {
        self.f += rhs;
    }
}

impl std::ops::Mul<f64> for ThreeJet {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            f:    self.f    * rhs,
            fu:   self.fu   * rhs,
            fv:   self.fv   * rhs,
            fuu:  self.fuu  * rhs,
            fuv:  self.fuv  * rhs,
            fvv:  self.fvv  * rhs,
            fuuv: self.fuuv * rhs,
            fuvv: self.fuvv * rhs,
        }
    }
}

impl std::ops::MulAssign<f64> for ThreeJet {
    fn mul_assign(&mut self, rhs: f64) {
        self.f    *= rhs;
        self.fu   *= rhs;
        self.fv   *= rhs;
        self.fuu  *= rhs;
        self.fuv  *= rhs;
        self.fvv  *= rhs;
        self.fuuv *= rhs;
        self.fuvv *= rhs;
    }
}

impl std::ops::Rem<f64> for ThreeJet {
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

impl std::ops::RemAssign<f64> for ThreeJet {
    fn rem_assign(&mut self, rhs: f64) {
        self.f = self.f % rhs;
        if self.f < 0.0 {
            self.f = self.f + rhs;
        };
    }
}

impl std::ops::BitXor<f64> for ThreeJet {
    type Output = Self;
    fn bitxor(self, rhs: f64) -> Self::Output {
        let x0: f64 = self.f.powf(rhs);

        // they took away my matching capabilities over floats :'|
        // (i'm sure there is a good reaso for that)
        let x1: f64 = if self.f == 0.0 { 0.0 } else { x0 * rhs / self.f };
        let x2: f64 = if self.f == 0.0 { 0.0 } else { x1 * (rhs - 1.0) / self.f };
        let x3: f64 = if self.f == 0.0 { 0.0 } else { x2 * (rhs - 2.0) / self.f };

        Self {
            f:    x0,
            fu:   x1 * self.fu,
            fv:   x1 * self.fv,
            fuu:  x1 * self.fuu  + x2 *        self.fu * self.fu,
            fuv:  x1 * self.fuv  + x2 *        self.fu * self.fv,
            fvv:  x1 * self.fvv  + x2 *        self.fv * self.fv,
            fuuv: x1 * self.fuuv + x2 * (2.0 * self.fu * self.fuv + self.fv * self.fuu) + x3 * self.fu * self.fu * self.fv,
            fuvv: x1 * self.fuvv + x2 * (2.0 * self.fv * self.fuv + self.fu * self.fvv) + x3 * self.fu * self.fv * self.fv,
        }
    }
}

impl std::ops::BitXorAssign<f64> for ThreeJet {
    fn bitxor_assign(&mut self, rhs: f64) {
        let x0: f64 = self.f.powf(rhs);
        
        // they took away my matching capabilities over floats :'|
        // (i'm sure there is a good reaso for that)
        let x1: f64 = if self.f == 0.0 { 0.0 } else { x0 * rhs / self.f };
        let x2: f64 = if self.f == 0.0 { 0.0 } else { x1 * (rhs - 1.0) / self.f };
        let x3: f64 = if self.f == 0.0 { 0.0 } else { x2 * (rhs - 2.0) / self.f };

        self.f    = x0;
        self.fu   = x1 * self.fu;
        self.fv   = x1 * self.fv;
        self.fuu  = x1 * self.fuu  + x2 *        self.fu * self.fu;
        self.fuv  = x1 * self.fuv  + x2 *        self.fu * self.fv;
        self.fvv  = x1 * self.fvv  + x2 *        self.fv * self.fv;
        self.fuuv = x1 * self.fuuv + x2 * (2.0 * self.fu * self.fuv + self.fv * self.fuu) + x3 * self.fu * self.fu * self.fv;
        self.fuvv = x1 * self.fuvv + x2 * (2.0 * self.fv * self.fuv + self.fu * self.fvv) + x3 * self.fu * self.fv * self.fv;
    }
}

impl ThreeJet {
    pub fn new(f: f64, fu: f64, fv: f64, fuu: Option<f64>, fuv: Option<f64>, fvv: Option<f64>, fuuv: Option<f64>, fuvv: Option<f64>) -> Self {
        Self {
            f:      f,
            fu:     fu,
            fv:     fv,
            fuu:    fuu.unwrap_or(0.0),
            fuv:    fuv.unwrap_or(0.0),
            fvv:    fvv.unwrap_or(0.0),
            fuuv:   fuuv.unwrap_or(0.0),
            fuvv:   fuvv.unwrap_or(0.0),
            
        }
    }
    pub fn zero() -> Self {
        Self { f: 0.0, fu: 0.0, fv: 0.0, fuu: 0.0, fuv: 0.0, fvv: 0.0, fuuv: 0.0, fuvv: 0.0 }
    }
    pub fn sin(&self) -> Self {
        let t: Self = (*self) * 2.0 * std::f64::consts::PI;
        let (s, c): (f64, f64) = (t.f.sin(), t.f.cos());
        
        Self {
            f:    s,
            fu:   c * t.fu,
            fv:   c * t.fv,
            fuu:  c * t.fuu  - s * t.fu * t.fu,
            fuv:  c * t.fuv  - s * t.fu * t.fv,
            fvv:  c * t.fvv  - s * t.fv * t.fv,
            fuuv: c * t.fuuv - s * (2.0 * t.fu * t.fuv + t.fv * t.fuu) - c * t.fu * t.fu * t.fv,
            fuvv: c * t.fuvv - s * (2.0 * t.fv * t.fuv + t.fu * t.fvv) - c * t.fu * t.fv * t.fv,
        }
    }
    pub fn take_sin(&mut self) {
        let t: Self = (*self) * 2.0 * std::f64::consts::PI;
        let (s, c): (f64, f64) = (t.f.sin(), t.f.cos());

        self.f    = s;
        self.fu   = c * t.fu;
        self.fv   = c * t.fv;
        self.fuu  = c * t.fuu  - s * t.fu * t.fu;
        self.fuv  = c * t.fuv  - s * t.fu * t.fv;
        self.fvv  = c * t.fvv  - s * t.fv * t.fv;
        self.fuuv = c * t.fuuv - s * (2.0 * t.fu * t.fuv + t.fv * t.fuu) - c * t.fu * t.fu * t.fv;
        self.fuvv = c * t.fuvv - s * (2.0 * t.fv * t.fuv + t.fu * t.fvv) - c * t.fu * t.fv * t.fv;
    }
    pub fn cos(&self) -> Self {
        let t: Self = (*self) * 2.0 * std::f64::consts::PI;
        let (s, c): (f64, f64) = (t.f.cos(), -t.f.sin());

        Self {
            f:    s,
            fu:   c * t.fu,
            fv:   c * t.fv,
            fuu:  c * t.fuu  - s * t.fu * t.fu,
            fuv:  c * t.fuv  - s * t.fu * t.fv,
            fvv:  c * t.fvv  - s * t.fv * t.fv,
            fuuv: c * t.fuuv - s * (2.0 * t.fu * t.fuv + t.fv * t.fuu) - c * t.fu * t.fu * t.fv,
            fuvv: c * t.fuvv - s * (2.0 * t.fv * t.fuv + t.fu * t.fvv) - c * t.fu * t.fv * t.fv,
        }
    }
    pub fn take_cos(&mut self) {
        let t: Self = (*self) * 2.0 * std::f64::consts::PI;
        let (s, c): (f64, f64) = (t.f.cos(), -t.f.sin());

        self.f    = s;
        self.fu   = c * t.fu;
        self.fv   = c * t.fv;
        self.fuu  = c * t.fuu  - s * t.fu * t.fu;
        self.fuv  = c * t.fuv  - s * t.fu * t.fv;
        self.fvv  = c * t.fvv  - s * t.fv * t.fv;
        self.fuuv = c * t.fuuv - s * (2.0 * t.fu * t.fuv + t.fv * t.fuu) - c * t.fu * t.fu * t.fv;
        self.fuvv = c * t.fuvv - s * (2.0 * t.fv * t.fuv + t.fu * t.fvv) - c * t.fu * t.fv * t.fv;
    }
    pub fn d(&self, index: i32) -> TwoJet {
        match index {
            0 => { TwoJet::new(self.fu, self.fuu, self.fuv, Some(self.fuuv)) },
            1 => { TwoJet::new(self.fv, self.fuv, self.fvv, Some(self.fuvv)) },
            _ => { TwoJet::zero() },
        }
    }
    pub fn annihilated(&self, index: i32) -> Self {
        let mut o: Self = Self::zero();
        o.f = self.f;
        match index {
            0 => {
                o.fv  = self.fv;
                o.fvv = self.fvv;
            },
            1 => {
                o.fu =  self.fu;
                o.fuu = self.fuu;
            },
            _ => { /* ????? */ }
        }
        return o;
    }
    pub fn annihilate(&mut self, index: i32) {
        self.fuuv = 0.0;
        self.fuvv = 0.0;

        match index {
            0 => {
                self.fu   = 0.0;
                self.fuu  = 0.0;
                self.fuv  = 0.0;
            },
            1 => {
                self.fv   = 0.0;
                self.fuv  = 0.0;
                self.fvv  = 0.0;
            },
            _ => {
                self.fv   = 0.0;
                self.fu   = 0.0;
                self.fuu  = 0.0;
                self.fuv  = 0.0;
                self.fvv  = 0.0;
            },
        };
    }
    pub fn interpolated(&self, rhs: Self, weight: Self) -> Self {
        return (*self) * ((weight) * (-1.0) + 1.0) + rhs * weight;
    }
    pub fn interpolate(&mut self, rhs: Self, weight: Self) {
        *self *= ((weight) * (-1.0) + 1.0) + rhs * weight;
    }
    pub fn print(&self) {
        println!("{f} ({fu} {fv})", f=self.f, fu=self.fu, fv=self.fv);
    }
}
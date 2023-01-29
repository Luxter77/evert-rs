use crate::c_gformat::CGFloat;

pub enum FormatType { CG, CF }
pub struct BrezierPoint { x: f64, y: f64, z: f64, ftm: FormatType }

impl std::fmt::Display for BrezierPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ftm {
            FormatType::CF => write!(f, "{} {} {}", CGFloat::from(self.x), CGFloat::from(self.y), CGFloat::from(self.z)),
            FormatType::CG => write!(f, "{} {} {}", self.x, self.y, self.z),
       }
    }
}

impl BrezierPoint {
    pub fn new(x: f64, y: f64, z: f64 ) -> Self { Self { x, y, z, ftm: FormatType::CF } }
    #[allow(unused)]
    pub fn zero() -> Self { Self { x: 0.0, y: 0.0, z: 0.0, ftm: FormatType::CF } }
    pub fn g(mut self) -> Self { self.ftm = FormatType::CG; return self; }
    #[allow(unused)]    
    pub fn f(mut self) -> Self { self.ftm = FormatType::CF; return self; }
    pub fn as_bytes(&self) -> [u8; 24] { [self.x.to_be_bytes(), self.y.to_be_bytes(), self.z.to_be_bytes()].concat().try_into().unwrap() }
}

pub struct SplinePoint { x: f64, y: f64, z: f64, nx: f64, ny: f64, nz: f64, s: f64/*, g: FormatType */ }

impl std::fmt::Display for SplinePoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{x} {y} {z}    {nx} {ny} {nz}", x=self.x, y=self.y, z=self.z, nx=self.nx*self.s, ny=self.ny*self.s, nz=self.nz*self.s)
    }
}

impl SplinePoint {
    pub fn new(x: f64, y: f64, z: f64, nx: f64, ny: f64, nz: f64, s: f64) -> Self { Self { x, y, z, nx, ny, nz, s } }
    #[allow(unused)]
    pub fn zero() -> Self { Self { x:  0.0, y:  0.0, z:  0.0, nx: 0.0, ny: 0.0, nz: 0.0, s: 0.0 } }
    // pub fn g(self) -> Self { Self { x: self.x, y: self.y, z: self.z, nx: self.nx, ny: self.ny, nz: self.nz, s: self.s, g: true } }
    // pub fn f(self) -> Self { Self { x: self.x, y: self.y, z: self.z, nx: self.nx, ny: self.ny, nz: self.nz, s: self.s, g: true } }
    // pub fn b(self) -> Self { Self { x: self.x, y: self.y, z: self.z, nx: self.nx, ny: self.ny, nz: self.nz, s: self.s, g: true } }
}
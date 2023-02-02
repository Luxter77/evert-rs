use crate::c_gformat::{CGFloat, signof};

pub enum FormatType { CG, CF }
pub struct BrezierPoint { x: f64, y: f64, z: f64, ftm: FormatType }

impl std::fmt::Display for BrezierPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.ftm {
            FormatType::CF => write!(f, "{:.6} {:.6} {:.6}", CGFloat::from(self.x), CGFloat::from(self.y), CGFloat::from(self.z)),
            FormatType::CG => write!(f, "{:.6} {:.6} {:.6}", self.x, self.y, self.z),
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
        let nx: f64 = self.nx * self.s;
        let ny: f64 = self.ny * self.s;
        let nz: f64 = self.nz * self.s;
        write!(f,
            "{xs}{x:.6} {ys}{y:.6} {zs}{z:.6}    {nxs}{nx:.6} {nys}{ny:.6} {nzs}{nz:.6}",
             xs=signof(self.x),  x=self.x.abs(),
             ys=signof(self.y),  y=self.y.abs(),
             zs=signof(self.z),  z=self.z.abs(),
            nxs=signof(nx),     nx=nx.abs(),
            nys=signof(ny),     ny=ny.abs(),
            nzs=signof(nz),     nz=nz.abs()
        )
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

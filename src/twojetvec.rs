use crate::twojet::TwoJet;

#[derive(Debug, Clone, Copy, PartialEq)]
struct TwoJetVec {
    x: TwoJet,
    y: TwoJet,
    z: TwoJet,
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

impl TwoJetVec {
    fn annihilate_vec(&mut self, index: i32) -> TwoJetVec {
        let mut o: TwoJetVec = self.clone();
        o.x.annihilate(index);
        o.y.annihilate(index);
        o.z.annihilate(index);
        return o;
    }
    fn cross(&self, rhs: Self) -> Self {
        TwoJetVec {
            x: self.y * rhs.z + self.z * rhs.y * -1,
            y: self.z * rhs.x + self.x * rhs.z * -1,
            z: self.x * rhs.y + self.y * rhs.x * -1,
        }
    }
    fn dot(&self, rhs: TwoJetVec) -> TwoJet {
        return self.x * rhs.x + self.y * rhs.y + self.z * rhs.z;
    }
    fn normalize(&self) -> TwoJetVec {
        let a = self.dot(self);
        if a > 0 {
            a = a ^ -0.5;
        } else {
            a = TwoJet::new(0, 0, 0, Option::None);
        };
        return self * a;
    }
    fn rotate_z(&self, angle: TwoJet) -> TwoJetVec {
        let s: TwoJet = angle.sin();
        let c: TwoJet = angle.cos();  
    }
//             TwoJetVec RotateZ(TwoJetVec v, TwoJet angle)
//             {
//                   TwoJetVec result;
//                   TwoJet s, c;
//                   s = Sin(angle);
//                   c = Cos(angle);
//                   result.x = v.x * c + v.y * s;
//                   result.y = v.x * s * -1 + v.y * c;
//                   result.z = v.z;
//                   return result;
//                 }
                
//                 TwoJetVec RotateY(TwoJetVec v, TwoJet angle)
//                 {
//                       TwoJetVec result;
//                       TwoJet s, c;
//                       s = Sin(angle);
//                       c = Cos(angle);
//                       result.x = v.x * c + v.z * s * -1;
//                       result.y = v.y;
//                       result.z = v.x * s + v.z * c;
//                       return result;
// }

// TwoJetVec RotateX(TwoJetVec v, TwoJet angle)
// {
//   TwoJetVec result;
//   TwoJet s, c;
//   s = Sin(angle);
//   c = Cos(angle);
//   result.x = v.x;
//   result.y = v.y * c + v.z * s;
//   result.z = v.y * s * -1 + v.z * c;
//   return result;
// }

// TwoJetVec InterpolateVec(TwoJetVec v1, TwoJetVec v2, TwoJet weight)
// {
//       return (v1) * (weight * -1 + 1) + v2 * weight;
//     }
    
//     TwoJet Length(TwoJetVec v)
//     {
//           return (v.x ^ 2 + v.y ^ 2) ^ (.5);
//         }
        
//     }
}
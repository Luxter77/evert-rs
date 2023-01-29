use crate::{threejet::ThreeJet, twojetvec::TwoJetVec, threejetvec::ThreeJetVec, figureeight::add_figure_eight};

/// Surface Time Operations
pub enum STO {
    Corrugate,
    PushThrough,
    Twist,
    UnPush,
    UnCorrugate,
    BendIn,
}

/// Magic number
static FF_POW: f64 = 3.0; 
/// Magic number
static FS_POW: f64 = 3.0;

trait Interpolable {
    fn t_interp(t: f64) -> Self;
    fn u_interp(&self) -> Self;
    fn ff_interp(&self) -> Self;
    fn fs_interp(&self) -> Self;
}

trait Bendable {
    fn arc(&self, rhs: Self, xsize: f64, ysize: f64, zsize: f64) -> ThreeJetVec;
    fn straight(&self, rhs: Self, xsize: f64, ysize: f64, zsize: f64) -> ThreeJetVec;
}

trait Parametric {
    fn param_1(&self) -> Self;
    fn param_2(&self) -> Self;
}

trait Staged {
    fn stage_0(&self, rhs: Self) -> ThreeJetVec;
    fn stage_1(&self, rhs: Self) -> ThreeJetVec;
    fn stage_2(&self, rhs: Self) -> ThreeJetVec;
    fn stage_3(&self, rhs: Self) -> ThreeJetVec;
    fn stage_4(&self, rhs: Self) -> ThreeJetVec;
    fn scene_01(&self, rhs: Self, t: f64) -> ThreeJetVec;
    fn scene_12(&self, rhs: Self, t: f64) -> ThreeJetVec;
    fn scene_23(&self, rhs: Self, t: f64) -> ThreeJetVec;
    fn scene_34(&self, rhs: Self, t: f64) -> ThreeJetVec;
}

pub trait Eversible {
    fn corrugate(&self, v: Self, t: f64) -> TwoJetVec;
    fn push_through(&self, v: Self, t: f64) -> TwoJetVec;
    fn twist(&self, v: Self, t: f64) -> TwoJetVec;
    fn unpush(&self, v: Self, t: f64) -> TwoJetVec;
    fn uncorrugate(&self, v: Self, t: f64) -> TwoJetVec;
    fn bend_in(&self, v: Self, t: f64) -> TwoJetVec;
}

impl Bendable for ThreeJet {
    fn arc(&self, rhs: Self, xsize: f64, ysize: f64, zsize: f64) -> ThreeJetVec {
        let u: Self = *self * 0.25;
        return ThreeJetVec::new(
            u.sin() * rhs.sin() * xsize,
            u.sin() * rhs.cos() * ysize,
            u.cos() * zsize,
        )
    }
    fn straight(&self, rhs: Self, xsize: f64, ysize: f64, zsize: f64) -> ThreeJetVec {
        let u: Self = *self * 0.25;
        // u = (u) * (-0.15915494) + 1; /* 1/2pi */
        return ThreeJetVec::new(
           rhs.sin() * xsize,
           rhs.cos() * ysize,
             u.cos() * zsize,
        )
    }
}

impl Interpolable for ThreeJet {
    #[inline]
    fn t_interp(t: f64) -> Self {
        Self::new_simple(t, 0.0, 0.0)
    }
    fn u_interp(&self) -> Self {
        let mut x: Self = *self % 2.0;
        if x > 1.0 { x = x * (-1.0) + 2.0 };
        return (x ^ 2.0) * 3.0 + (x ^ 3.0) * (-2.0);
    }
    fn ff_interp(&self) -> Self {
        let mut x: Self = *self % 2.0;
        if x > 1.0 { x = x * -1.0 + 2.0 };
        x = x * 1.06 + -0.05;
        if x < 0.0 {      return Self::zero(); }
        else if x > 1.0 { return Self::zero() + 1.0; }
        else {            return (x ^ (FF_POW - 1.0)) * (FF_POW) + (x ^ FF_POW) * (-FF_POW + 1.0); };
    }
    fn fs_interp(&self) -> Self {
        let mut x: Self = *self % 2.0;
        if x > 1.0 { x = x * (-1.0) + 2.0 }
        return ((x ^ (FS_POW - 1.0)) * (FS_POW) + (x ^ FS_POW) * (-FS_POW + 1.0)) * (-0.2);
    }
}

impl Parametric for ThreeJet {
    fn param_1(&self) -> Self {
        let mut x: Self = *self % 4.0;
        let mut offset: f64 = 0.0;
        if x > 2.0 {
            x += -2.0;
            offset = 2.0;
        };
        if x <= 1.0 {
            return x * 2.0 + (x ^ 2.0) * -1.0 + offset;
        } else {
            return (x ^ 2.0) + x * -2.0 + (2.0 + offset);
        };
    }
    fn param_2(&self) -> Self {
        let mut offset: f64 = 0.0;
        let mut x: Self = *self % 4.0;
        if x > 2.0 {
            x += -2.0;
            offset = 2.0;
        };
        if x <= 1.0 {
            return (x ^ 2.0) + offset;
        } else {
            return (x ^ 2.0) * -1.0 + x * 4.0 + (-2.0 + offset);
        };
    }
}

impl Staged for ThreeJet {
    fn stage_0(&self, rhs: Self) -> ThreeJetVec {
        self.straight(rhs, 1.0, 1.0, 1.0)
    }
    fn stage_1(&self, rhs: Self) -> ThreeJetVec {
        self.arc(rhs, 1.0, 1.0, 1.0)
    }
    fn stage_2(&self, rhs: Self) -> ThreeJetVec {
        self.param_1().arc(rhs, 0.9, 0.9, -1.0).interpolated(self.param_2().arc(rhs, 1.0, 1.0, 0.5), self.u_interp())
    }
    fn stage_3(&self, rhs: Self) -> ThreeJetVec {
        self.param_1().arc(rhs, -0.9, -0.9, -1.0).interpolated(self.param_2().arc( rhs, -1.0, 1.0, -0.5), self.u_interp())
    }
    fn stage_4(&self, rhs: Self) -> ThreeJetVec {
        self.arc(rhs, -1.0, -1.0, -1.0)
    }
    fn scene_01(&self, rhs: Self, t: f64) -> ThreeJetVec {
        self.stage_0(rhs).interpolated(self.stage_1(rhs), Self::t_interp(t))
    }
    fn scene_12(&self, rhs: Self, t: f64) -> ThreeJetVec {
        self.stage_1(rhs).interpolated(self.stage_2(rhs), Self::t_interp(t))
    }
    fn scene_23(&self, rhs: Self, t: f64) -> ThreeJetVec {
        let t: Self = Self::t_interp(t) * 0.5;
        
        let mut tt: f64;
        
        if *self <= 1.0 {
            tt = t.into();
        } else {
            tt  = t.into();
            tt *= -1.0;
        }


        let p1:     Self = self.param_1();
        let interp: Self = self.u_interp();

        let tj:     Self = Self::new_simple(tt, 0.0, 0.0);
        
        let p1a1:   ThreeJetVec = p1.arc(rhs, 0.9, 0.9, -1.0);
        let p1a2:   ThreeJetVec = p1.arc(rhs, 1.0, 1.0, 0.5);

        return p1a1.rotated_z(tj).interpolated(p1a2, interp);
    }
    fn scene_34(&self, rhs: Self, t: f64) -> ThreeJetVec {
        self.stage_3(rhs).interpolated(self.stage_4(rhs), Self::t_interp(t))
    }
}

impl Eversible for ThreeJet {
    fn bend_in(&self, rhs: Self, t: f64) -> TwoJetVec {
        let ti:   Self = Self::t_interp(t);
        let fv1:  Self = Self::new_simple(0.0, 0.0, 1.0);
        let zero: Self = Self::zero();
        return add_figure_eight(self.scene_01(fv1, ti.into()), *self, rhs.into(), zero, self.fs_interp());
    }
    fn corrugate(&self, rhs: Self, t: f64) -> TwoJetVec {
        let t: ThreeJet = Self::t_interp(t);
        return add_figure_eight(
            self.stage_1(Self::new_simple(0.0, 0.0, 1.0)),
            (*self).into(), rhs.into(), self.ff_interp() * Self::from(t), self.fs_interp()
        );
    }
    fn push_through(&self, rhs: Self, t: f64) -> TwoJetVec {
        return add_figure_eight(
            self.scene_12(Self::new_simple(0.0, 0.0, 1.0), t),
            (*self).into(), rhs.into(), self.ff_interp(), self.fs_interp()
        );
    }
    fn twist(&self, rhs: Self, t: f64) -> TwoJetVec {
        return add_figure_eight(
            self.scene_23(Self::new_simple(0.0, 0.0, 1.0), t),
            (*self).into(), rhs.into(), self.ff_interp(), self.fs_interp()
        );
    }
    fn unpush(&self, rhs: Self, t: f64) -> TwoJetVec {
        return add_figure_eight(
            self.scene_34(Self::new_simple(0.0, 0.0, 1.0), t),
            (*self).into(), rhs.into(), self.ff_interp(), self.fs_interp()
        );
    }
    fn uncorrugate(&self, rhs: Self, t: f64) -> TwoJetVec {
        let t: ThreeJet = Self::t_interp(t * -1.0 + 1.0);
        return add_figure_eight(
            self.stage_4(Self::new_simple(0.0, 0.0, 1.0)),
            (*self).into(), rhs.into(), self.ff_interp() * t, self.fs_interp()
        );
    }
}
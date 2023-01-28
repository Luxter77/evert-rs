use crate::nstrip::{N_STRIPS, EasyAtomic};

use crate::threejet::ThreeJet;
use crate::threejetvec::ThreeJetVec;

use crate::twojet::TwoJet;
use crate::twojetvec::TwoJetVec;

fn figure_eight(w: TwoJetVec, h: TwoJetVec, bend: TwoJetVec, form: TwoJet, v: TwoJet) -> TwoJetVec {
    let v: TwoJet = v % 1.0;
    let mut height: TwoJet = ((v * 2.0).cos() + 1.0) * -1.0;
    if v > 0.25 && v < 0.75 {
        height = (height * -1.0) + 4.0;
    }
    height *= 0.6;
    let h: TwoJetVec = h + bend * (height * height * (1.0 / 64.0));
    return w * (v * 2.0) + h * ((v.cos() + -1.0) * -2.0).interpolated(height, form);
}

pub fn add_figure_eight(p: ThreeJetVec, u: ThreeJet, v: TwoJet, form: ThreeJet, scale: ThreeJet) -> TwoJetVec {
    let size: ThreeJet    = form * scale;
    let form: ThreeJet    = form * 2.0 + form * form * -1.0;
    let dv:     TwoJetVec = p.d(1).annihilated(1);
    let p:    ThreeJetVec = p.anihilated(1);
    let du:     TwoJetVec = p.d(0).normalized();
    let h:      TwoJetVec = du.crossed(dv).normalized() * Into::<TwoJet>::into(size);
    let w:      TwoJetVec = h.crossed(du).normalized() * (Into::<TwoJet>::into(size) * 1.1);
    let fig:    TwoJetVec = figure_eight(w, h, du * size.d(0) * (u.d(0) ^ -1.0), form.into(), v);
    let o:      TwoJetVec = (TwoJetVec::from(p) + fig).rotated_z(v * (1.0 / N_STRIPS.get() as f64));
    return o.rotated_z(v * (1.0 / N_STRIPS.get() as f64));
}
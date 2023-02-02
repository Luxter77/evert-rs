#![allow(clippy::needless_return)] // NO

use clap::Parser;
use sphere::Sto;

mod twojet;
mod twojetvec;

mod threejet;
mod threejetvec;

mod nstrip;
mod figureeight;

mod c_gformat;
mod points;

mod sphere;
mod spline;

use crate::nstrip::{
    ALLPARTS,
	N_STRIPS,
    BINARY,
	EasyAtomic
};

static LONG_ABOUT: &str = "Generate an everting sphere in Geomview/OOGL MESH or Bezier at:

    The surface of the sphere at T = 0 is parametrized by:

        u: u = 0 at +Z pole,
           u = 1 at equator,
           u = 2 at -Z pole
        
        v: v = 0 at +Y, increasing toward +X,
           v = 1 at longitude 2pi/(number of strips)

Emits slice ranging in: 
    u from Umin..Umax,
    v from Vmin..Vmax,

With each OOGL MESH or Bezier element spanning range Ustep x Vstep.

Produces radius-S sphere.";

/// Generate an everting sphere in Geomview/OOGL MESH or Bezier
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = LONG_ABOUT)]
#[clap(color = clap::ColorChoice::Auto)]
struct Args {
    /// Timestep [0 <= T <= 1]
    #[arg(long, default_value_t = 0.00)]                 time:       f64,
    #[arg(long, default_value_t = 8)]                    nstrips:    i32,
    #[arg(long, default_value_t = std::f64::consts::PI)] scale:      f64,
    #[arg(long, default_value_t = 0.00)]                 umin:       f64,
    #[arg(long, default_value_t = 1.00)]                 umax:       f64,
    /// Parameter for the surface of the sphere at T = 0
    #[arg(long, default_value_t = 0.08333)]              du:         f64,
    #[arg(long, default_value_t = 0.00)]                 vmin:       f64,
    #[arg(long, default_value_t = 1.00)]                 vmax:       f64,
    /// Parameter for the surface of the sphere at T = 0
    #[arg(long, default_value_t = 0.08333)]              dv:         f64,
    /// Timestamp at which eversion begins to corrugate
    #[arg(long, default_value_t = 0.00)]                 corr:       f64,
    /// Timestamp at which eversion begins to push
    #[arg(long, default_value_t = 0.10)]                 push:       f64,
    /// Timestamp at which eversion begins to twist
    #[arg(long, default_value_t = 0.23)]                 twist:      f64,
    /// Timestamp at which eversion begins to unpush
    #[arg(long, default_value_t = 0.60)]                 unpush:     f64,
    // /// Timestamp at which eversion begins to uncorrugate
    #[arg(long, default_value_t = 0.93)]                 uncorr:     f64,
    /// Generate the sphere in Bezier form.
    #[arg(long, required=false, default_value_t=false)]  bezier:     bool,
    /// Include transformations to replicate [0..1,0..1] to whole sphere
    #[arg(long, required=false, default_value_t=false)]  whole:      bool,
    /// Undocumented switch, sets scene to true
    #[arg(long, required=false, default_value_t=false)]  scene:      bool,
    /// Undocumented swtich, sets scene to false
    #[arg(long, required=false, default_value_t=false)]  bscene:     bool,
    /// Undocumented swtich, triggers binary output instead of human readable.
    #[arg(long, required=false, default_value_t=false)]  binary:     bool,
    /// Replicate selected portions or all if '*'; e.g. +0-0+2+4+6 for one pole-to-pole strip, plus every other strip in +Z hemisphere; numbers range [0..(nstrips-1)].
    #[arg(long, default_value_t = String::from("+0"))]   parts:      String,
}

fn main() {
    let args: Args = Args::parse();
   
    if ALLPARTS && !args.parts.is_empty() { eprintln!("Evert was built with the AllParts feature; parts will be ignored!") };

    N_STRIPS.set(args.nstrips);
    BINARY.set(args.binary);

    let parts: Vec<char> = args.parts.as_bytes().iter().map(|x: &u8 | { *x as char }).collect();

    let (umin, vmin, umax, vmax, adu, adv, _scale): (f64, f64, f64, f64, f64, f64, f64) = (args.umin, args.vmin, args.umax, args.vmax, args.du, args.dv, args.scale);

    let time: f64 = args.time;

    // -1 means don't do bendtime at all
    // it seems the original code had te ability to bend.
    // current implementation has some snippets here and there about it, and the math.
    // but the logic seems to have been removed at some point.
    // and since I'm doing a braindead one2one implementation of the logic on the original program 
    // I can't derive the original implementation
    // so here it lies as an eternal TODO to be hilighted by visual studio code...
    // TODO: reimplement bendtime
    let bendtime:    f64 =  -1.00;

    let corrstart:   f64 =  args.corr;
    let pushstart:   f64 =  args.push;
    let twiststart:  f64 =  args.twist;
    let unpushstart: f64 =  args.unpush;
    let uncorrstart: f64 =  args.uncorr;

    if bendtime >= 0.0 {
        spline::print_scene(Sto::BendIn, umin, umax, adu, vmin, vmax, adv, bendtime, parts);
    } else {
        /* time = (time - howfar) / chunk */
        if !args.bscene || args.scene {
            if time >= uncorrstart && uncorrstart >= 0.0 {
                let t: f64 = (time - uncorrstart) / (1.0 - uncorrstart);
                spline::print_scene(Sto::UnCorrugate, umin, umax, adu, vmin, vmax, adv, t, parts);
            };
        } else if time >= unpushstart && unpushstart >= 0.0 {
            let t: f64 = (time - unpushstart) / (if uncorrstart < 0.0 { 1.0 } else { uncorrstart } - unpushstart); 
            spline::print_scene(Sto::UnPush, umin, umax, adu, vmin, vmax, adv, t, parts);
        } else if time >= twiststart && twiststart >= 0.0 {
            let t: f64 = (time - twiststart) / (if unpushstart < 0.0 { 1.0 } else { unpushstart } - twiststart);
            spline::print_scene(Sto::Twist, umin, umax, adu, vmin, vmax, adv, t, parts);
        } else if time >= pushstart && pushstart >= 0.0 {
            let t: f64 = (time - pushstart) / (if twiststart < 0.0 { 1.0 } else { twiststart } - pushstart);
            spline::print_scene(Sto::PushThrough, umin, umax, adu, vmin, vmax, adv, t, parts);
        } else if time >= corrstart && corrstart >= 0.0 {
            let t: f64 = (time - corrstart) / (if pushstart < 0.0 { 1.0 } else { pushstart } - corrstart);
            spline::print_scene(Sto::Corrugate, umin, umax, adu, vmin, vmax, adv, t, parts);
        };
    }
}

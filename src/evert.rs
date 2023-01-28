use clap::Parser;

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
    #[arg(long, default_value_t = 0.00, required=true)]  time:       f64,
    #[arg(long, default_value_t = 8)]                    nstrips:    i32,
    #[arg(long, default_value_t = 1.00)]                 scale:      f64,
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
    /// Replicate selected portions, e.g. +0-0+2+4+6 for one pole-to-pole strip, plus every other strip in +Z hemisphere; numbers range [0..(nstrips-1)].
    #[arg(long, default_value_t = String::from("+0"))]   parts:      String,
}

fn main() {
    let args: Args = Args::parse();
   
    N_STRIPS.set(args.nstrips);
    BINARY.set(args.binary);

    let mut parts: String = args.parts;

    let (umin, vmin, umax, vmax, du, dv, scale): (f64, f64, f64, f64, f64, f64, f64) = (args.umin, args.vmin, args.umax, args.vmax, args.du, args.dv, args.scale);

    let (corrstart, pushstart, twiststart, unpushstart, uncorrstart): (f64, f64, f64, f64, f64);

    let time: f64 = 0.0;

    let havetime: f64 = args.time;

    let bendtime:    f64 = -1.00; // -1 means don't do bendtime at all
    let corrstart:   f64 =  0.00;
    let pushstart:   f64 =  0.10;
    let twiststart:  f64 =  0.23;
    let unpushstart: f64 =  0.60;
    let uncorrstart: f64 =  0.93;

    if bendtime >= 0.0 {
        // spline::print_scene(BendIn, umin, umax, du, vmin, vmax, dv, bendtime);
    } else {
        /* time = (time - howfar) / chunk */
        if !args.bscene || args.scene {
            if time >= uncorrstart && uncorrstart >= 0.0 {
                // spline::printScene(UnCorrugate, umin, umax, du, vmin, vmax, dv, (time - uncorrstart) / (1.0 - uncorrstart), &mut parts);
            }
        } else if time >= unpushstart && unpushstart >= 0.0 {
            // spline::printScene(UnPush, umin, umax, du, vmin, vmax, dv, (time - unpushstart) / (((uncorrstart < 0.0) ? 1.0 : uncorrstart) - unpushstart), &mut parts);
        } else if time >= twiststart && twiststart >= 0.0 {
            // spline::printScene(Twist, umin, umax, du, vmin, vmax, dv, (time - twiststart) / (((unpushstart < 0.0) ? 1.0 : unpushstart) - twiststart), &mut parts);
        } else if time >= pushstart && pushstart >= 0.0 {
            // spline::printScene(PushThrough, umin, umax, du, vmin, vmax, dv, (time - pushstart) / (((twiststart < 0.0) ? 1.0 : twiststart) - pushstart), &mut parts);
        } else if time >= corrstart && corrstart >= 0.0 {
            // spline::printScene(Corrugate, umin, umax, du, vmin, vmax, dv, (time - corrstart) / (((pushstart < 0.0) ? 1.0 : pushstart) - corrstart), &mut parts);
        };
    }
}

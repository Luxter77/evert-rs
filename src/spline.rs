use crate::{
	nstrip::{ N_STRIPS, BREZIER, BINARY, EasyAtomic },
	twojetvec::TwoJetVec,
	threejet::ThreeJet,
	sphere::{Eversible, Sto},
};

static PART_POS: i32 = 0x1;
static PART_NEG: i32 = 0x2;

pub trait PrintableSpline {
	#[allow(clippy::too_many_arguments)] // aguantese como hombre
	fn print_spline(&self, v01: TwoJetVec,  v10: TwoJetVec, v11: TwoJetVec, us: f64, vs: f64, s0: f64, s1: f64, t0: f64, t1: f64);
}

type TwoJetVVV = Vec<Vec<TwoJetVec>>;
type SpeedVec = Vec<f64>;
type AccelVec = Vec<SpeedVec>;

fn calc_speed_v(oper: Sto, u: f64, t: f64) -> f64 {
	let nu: ThreeJet = ThreeJet::new_simple(u, 1.0, 0.0);
	let vu: ThreeJet = ThreeJet::new_simple(0.0, 0.0, 0.1);

	let o: f64 = match oper {
    	Sto::Corrugate => { nu.corrugate(vu, t) },
    	Sto::PushThrough => { nu.push_through(vu, t) },
    	Sto::Twist => { nu.twist(vu, t) },
    	Sto::UnPush => { nu.unpush(vu, t) },
    	Sto::UnCorrugate => { nu.uncorrugate(vu, t) },
    	Sto::BendIn => { nu.bend_in(vu, t) },
	}.calc_speed_v();

	if o != 0.0 {
		return o;
	} else {
		return calc_speed_v(oper, u + if u < 1.0 { 1e-9 } else { -1e-9 }, t);
	};
}

fn calc_speed_u(oper: Sto, u: f64, v: f64, t: f64) -> TwoJetVec {
	let nu: ThreeJet = ThreeJet::new_simple(u, 1.0, 0.0);
	let vu: ThreeJet = ThreeJet::new_simple(v, 0.0, 1.0);

	return match oper {
    	Sto::Corrugate => { nu.corrugate(vu, t) },
    	Sto::PushThrough => { nu.push_through(vu, t) },
    	Sto::Twist => { nu.twist(vu, t) },
    	Sto::UnPush => { nu.unpush(vu, t) },
    	Sto::UnCorrugate => { nu.uncorrugate(vu, t) },
    	Sto::BendIn => { nu.bend_in(vu, t) },
	};
}

#[allow(clippy::too_many_arguments)]
pub fn print_scene(oper: Sto, umin: f64, umax: f64, adu: f64, vmin: f64, vmax: f64, adv: f64, t: f64, parts: Vec<char>) { 
	let (mut u, mut v): (f64, f64);
	let (mut ju, mut ku): (usize, usize);
	
	let jmax: i32 = (((umax - umin).abs() / adu + 0.5) as i32).max(1);
	let kmax: i32 = (((vmax - vmin).abs() / adv + 0.5) as i32).max(1);

	let du: f64 = (umax - umin) / jmax as f64;
	let dv: f64 = (vmax - vmin) / kmax as f64;

	let mut values: TwoJetVVV = Vec::with_capacity((jmax + 1) as usize);
	let mut speedu: AccelVec  = Vec::with_capacity((jmax + 1) as usize);
	let mut speedv: SpeedVec  = Vec::with_capacity((jmax + 1) as usize);

	for j in 0..jmax {
		ju = j as usize;
		u = umin + du * (j as f64);

		values.push(vec![TwoJetVec::zero(); (kmax + 1) as usize]);
		speedu.push(vec![0.0; (kmax + 1) as usize]);
		speedv.push(calc_speed_v(oper, u, t));

		for k in 0..kmax {
			v = vmin + dv * k as f64;
			ku = k as usize;
			values[ju][ku] = calc_speed_u(oper, u, v, t);
			speedu[ju][ju] = values[ju][ku].calc_speed_u();
		};
	};

	// println!("Declare \"speeds\" \"varying float\"");
	// println!("Declare \"speedt\" \"varying float\"");
	if !parts.len().eq(&0) {
		/* Construct matrices to replicate standard unit (u=0..1, v=0..1) into
		 * complete sphere. */

		let partlist: Vec<char> = parse_parts(parts);
		assert!(!partlist.is_empty());

		println!("{{ INST transforms {{ TLIST");
		
		let mut j = -1;
		while j <= 1 { j += 2; // ???
			for (k, part) in partlist.iter().enumerate().take(N_STRIPS.get() as usize) {
				let sign: i32 = if j < 0 { PART_NEG } else { PART_POS };
				if (*part as i32 & sign) > 0 {
					let jk: f64 = if j < 0 { N_STRIPS.get() as f64 - 1.0 - k as f64 } else { k as f64 };
					let t: f64 = 2.0 * std::f64::consts::PI * jk / N_STRIPS.get() as f64;
					let s: f64 = t.sin();
					let c: f64 = t.cos();

					let sign = if j < 0 { '-' } else { '+' };
					// fprintf(fp, "# %c%d of %d\n", j < 0 ? '-' : '+', k, N_STRIPS.get());
					println!("# {sign}{k} of {ns}", sign=sign, k=k, ns=N_STRIPS.get());
					// fprintf(fp, "\t%10f %10f %10f %10f\n", j * c, -s, 0., 0.);
					println!("\t{jc:10} {s:10} {z:10} {z:10}", jc=(j as f64 * c), s=-s, z=0.0);
					// fprintf(fp, "\t%10f %10f %10f %10f\n", j * s, c, 0., 0.);
					println!("\t{js:10} {c:10} {z:10} {z:10}", js=(j as f64 * s), c=c, z=0.0);
					// fprintf(fp, "\t%10f %10f %10f %10f\n", 0., 0., (double)j, 0.);
					println!("\t{z:10} {z:10} {jf:10} {z:10}", z=0.0, jf=(j as f64));
					// fprintf(fp, "\t%10f %10f %10f %10f\n", 0., 0., 0., 1.);
					println!("\t{z:10} {z:10} {z:10} {u:10}", u=1.0, z=0.0);
				};
			};
		};
		print!("}}\ngeom ");
	}
	if BREZIER.get() {
		println!("{{ STBBP{}", if BINARY.get() { " BINARY" } else { "" });
		for j in 0..jmax as usize {
			// this was ther, doing nothing...
			// u = umin + j as f64 * du;
			for k in 0..kmax as usize {
				// this was ther, doing nothing...
				// v = vmin + k as f64 * dv;
				values[j][k].print_spline(
					values[j][k + 1], values[j + 1][k], values[j + 1][k + 1],
					du, dv,
					umin + j as f64 * du, umin + (j + 1) as f64 * du,
					vmin + k as f64 * dv, vmin + (k + 1) as f64 * dv
				);
			};
		};
	} else {
		let nu: i32 = kmax + 1;
		let nv: i32 = jmax + 1;
		println!("{{ NMESH{}", if BINARY.get() { " BINARY" } else { "" });

		if BINARY.get() {
			std::io::Write::write(&mut std::io::stdout(), &nu.to_be_bytes()).unwrap();
			std::io::Write::write(&mut std::io::stdout(), &nv.to_be_bytes()).unwrap();
		} else {
			println!("%{} %{}\n", nu, nv);
		}
		for valuej in values.iter().take(jmax as usize) {
			for valuejk in valuej.iter().take(kmax as usize) {
				print!("{}",valuejk.point(None)); }
			if !BINARY.get() { println!() };
		}
	}
}

fn parse_parts(parts: Vec<char>) -> Vec<char> {
	/* Construct matrices to replicate standard unit (u=0..1, v=0..1) into
	 * complete sphere.  */
	let mut partlist: Vec<char> = Vec::new();
	let mut sign: char = '!';
	let mut bits: i32;

	let lim: usize = parts.len();
	let mut idx: usize = 0;

	while idx < lim {
		if parts[idx] == ' ' || parts[idx] == ',' {
			idx += 1;
			sign = dbg!(parts[idx]);
			continue;
		};

		if sign == '+' {		bits = PART_POS;
		} else if sign == '-' { bits = PART_NEG;
		} else {				bits = PART_POS | PART_NEG;
			idx = idx.overflowing_sub(1).0.clamp(0, lim-1);
		};

		if parts[idx] == '*' {
			for j in 0..N_STRIPS.get() as usize {
				partlist[j] = (*partlist.get(j).unwrap_or(&char::from(0)) as u8 | bits as u8) as char;
			};
			idx += 1;
		} else {
			let mut ncp: usize = 0;
 			let ji: i64 = crate::c_gformat::str_to_i64(&parts, &mut ncp, 10).expect("Invalid number passed to -parts argument");
 			if idx == ncp { panic!("evert -parts: expected string with alternating signs and strip numbers"); };
 			if ji < 0 || ji >= N_STRIPS.get().into() { panic!("evert -parts: bad strip number {ji}; must be in range 0..{n}\n", ji=ji, n=N_STRIPS.get() - 1); };
 			let element: char = (*partlist.get(ji as usize).unwrap_or(&char::from(0)) as u8 | bits as u8) as char;
			partlist.insert(ji as usize, element);
 			idx = ncp;
 		};
	};
	return partlist;
}
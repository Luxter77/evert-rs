use std::char;


fn printScene(func: &SurfaceTimeFunction, umin: f64, umax: f64, adu: f64, vmin: f64, vmax: f64, adv: f64, t: f64, binary: i32) {

}

// #ifndef __SPLINE
// #define __SPLINE
// #include "threejetvec.h"
// #include "twojetvec.h"

// typedef TwoJetVec SurfaceTimeFunction(ThreeJet u, ThreeJet v, double t);

// void print_point (FILE *fp, TwoJetVec p, double ps, double pus, double pvs, double puvs);
// void printSpline(FILE *fp, TwoJetVec v00, TwoJetVec v01, TwoJetVec v10, TwoJetVec v11, double us, double vs, double s0,double s1,double t0,double t1);
// void printScene(
//   SurfaceTimeFunction *func,
//   double umin, double umax, double du,
//   double vmin, double vmax, double dv,
//   double t, int binary
// );        
// void impossible(char *msg);
// #endif

// #include "evert.h"

fn print_point(fp: &FILE, p: TwoJetVec, ps: f64, pus: f64, pvs: f64, puvs: f64, binary: f64) {
  if(bezier) {
    let xyz: [f64; 3];
    
    xyz[0] = double(p.x)*ps + p.x.df_du()*pus/3.0 + p.x.df_dv()*pvs/3.0 + p.x.d2f_dudv()*puvs/9.;
    xyz[1] = double(p.y)*ps + p.y.df_du()*pus/3.0 + p.y.df_dv()*pvs/3.0 + p.y.d2f_dudv()*puvs/9.;
    xyz[2] = double(p.z)*ps + p.z.df_du()*pus/3.0 + p.z.df_dv()*pvs/3.0 + p.z.d2f_dudv()*puvs/9.;
    
    if (binary) {
      fwrite(&xyz, sizeof(float), 3, fp);
    } else {
      fprintf(fp, "%g %g %g\n", xyz[0], xyz[1], xyz[2]);
    }
  } else {
    let x:  f64 = double(p.x) * ps;
    let y:  f64 = double(p.y) * ps;
    let z:  f64 = double(p.z) * ps;
    let nx: f64 = p.y.df_du() * p.z.df_dv() - p.z.df_du() * p.y.df_dv();
    let ny: f64 = p.z.df_du() * p.x.df_dv() - p.x.df_du() * p.z.df_dv();
    let nz: f64 = p.x.df_du() * p.y.df_dv() - p.y.df_du() * p.x.df_dv();
    let s:  f64 = nx.powi(2) + ny.powi(2) + nz.powi(2);
    
    if(s > 0) {
        s = sqrt(1/s)
    };
    
    fprintf(fp, "%f %f %f    %f %f %f\n", x, y, z, nx*s, ny*s, nz*s);
  }
}

fn printMesh(fp: &FILE, p: TwoJetVec, binary: i32) {
    let x: f64  = double(p.x);
    let y: f64  = double(p.y);
    let z: f64  = double(p.z);
    let nx: f64 = p.y.df_du()*p.z.df_dv() - p.z.df_du()*p.y.df_dv();
    let ny: f64 = p.z.df_du()*p.x.df_dv() - p.x.df_du()*p.z.df_dv();
    let nz: f64 = p.x.df_du()*p.y.df_dv() - p.y.df_du()*p.x.df_dv();
    let s: f64  = nx.powi(2) + ny.powi(2) + nz.powi(2);
    
    if(s > 0) {
        s = sqrt(1/s)
    };
    
    fprintf(fp, "%f %f %f    %f %f %f\n", x, y, z, nx*s, ny*s, nz*s);
}

fn printSpline(fp: &FILE, v00: TwoJetVec, v01: TwoJetVec, v10: TwoJetVec, v11: TwoJetVec, us: f64, vs: f64, s0: f64, s1: f64, t0: f64, t1: f64, binary: bool) {
    if (bezier) {
        print_point(fp, v00, 1, 0, 0, 0, binary);
        print_point(fp, v00, 1, us, 0, 0, binary);
        print_point(fp, v10, 1,-us, 0, 0, binary);
        print_point(fp, v10, 1, 0, 0, 0, binary);
        
        print_point(fp, v00, 1, 0, vs, 0, binary);
        print_point(fp, v00, 1, us, vs, us*vs, binary);
        print_point(fp, v10, 1,-us, vs,-us*vs, binary);
        print_point(fp, v10, 1, 0, vs, 0, binary);
        
        print_point(fp, v01, 1, 0,-vs, 0, binary);
        print_point(fp, v01, 1, us,-vs,-us*vs, binary);
        print_point(fp, v11, 1,-us,-vs, us*vs, binary);
        print_point(fp, v11, 1, 0,-vs, 0, binary);
        
        print_point(fp, v01, 1, 0, 0, 0, binary);
        print_point(fp, v01, 1, us, 0, 0, binary);
        print_point(fp, v11, 1,-us, 0, 0, binary);
        print_point(fp, v11, 1, 0, 0, 0, binary);

        if (binary) {
            let sts: [f64; 8] = [s0, t0, s1, t0, s0, t1, s1, t1];
            fwrite(&sts, sizeof(float), 8, fp);
        } else {
          fprintf(fp, "%g %g  %g %g  %g %g  %g %g\n\n", s0, t0, s1, t0, s0, t1, s1, t1);
        }
    } else {
        print_point(fp, v00, 1, us, vs, us*vs, binary);
        print_point(fp, v10, 1, us, vs, us*vs, binary);
        print_point(fp, v11, 1, us, vs, us*vs, binary);
        print_point(fp, v01, 1, us, vs, us*vs, binary);
    
        if(!binary) {
            fputc('\n', fp);
        }
  }
}

#[inline(always)]
fn calcSpeedV(v: TwoJetVec) -> f64 {
  return v.x.df_dv().powi(2) + v.y.df_dv().powi(2) + v.z.df_dv().powi(2);
}

#[inline(always)]
fn calcSpeedU(v: TwoJetVec) -> f64 {
  return (v.x.df_du().powi(2) + v.y.df_du().powi(2) + v.z.df_du().powi(2)).sqrt();
}

// #define	PART_POS 0x1
// #define	PART_NEG 0x2

fn parse_parts(parts: String) {
	/* Construct matrices to replicate standard unit (u=0..1, v=0..1) into
	 * complete sphere.
	 */
	char *partlist = (char *)calloc(n_strips, sizeof(char));
	char *cp, *ncp, sign;
	int bits, j;

	for (cp = parts; *cp;)
	{
		while ((sign = *cp++) == ' ' || sign == ',')
			;
		if (sign == '+')
			bits = PART_POS;
		else if (sign == '-')
			bits = PART_NEG;
		else
		{
			bits = PART_POS | PART_NEG;
			cp--;
		}
		if (*cp == '*')
		{
			for (j = 0; j < n_strips; j++)
				partlist[j] |= bits;
			cp++;
		}
		else
		{
			j = strtol(cp, &ncp, 0);
			if (cp == ncp)
			{
				fprintf(stderr,
						"evert -parts: expected string with alternating signs and strip numbers\n");
				return NULL;
			}
			if (j < 0 || j >= n_strips)
			{
				fprintf(stderr,
						"evert -parts: bad strip number %d; must be in range 0..%d\n", j, n_strips - 1);
				return NULL;
			}
			partlist[j] |= bits;
			cp = ncp;
		}
	}
	return partlist;
}

fn printScene(
		SurfaceTimeFunction *func,
		double umin, double umax, double adu,
		double vmin, double vmax, double adv,
		double t, int binary
		) {
  static TwoJetVec **values;
  int j, k;
  int jmax = (int) (fabs(umax-umin)/adu+.5);
  int kmax = (int) (fabs(vmax-vmin)/adv+.5);
  double u, v, du, dv;
  FILE *fp = stdout;

  if(jmax == 0) jmax = 1;
  du = (umax-umin) / jmax;
  if(kmax == 0) kmax = 1;
  dv = (vmax-vmin) / kmax;
  values = (TwoJetVec **) calloc(jmax+1, sizeof(TwoJetVec *));
  double *speedv = (double *) calloc(jmax+1, sizeof(double));
  double **speedu = (double **) calloc(jmax+1, sizeof(double *));
  for (j = 0; j <= jmax; j++) {
    u = umin + j*du;
    values[j] = (TwoJetVec *) calloc(kmax+1, sizeof(TwoJetVec));
    speedu[j] = (double *) calloc(kmax+1, sizeof(double));
    speedv[j] = calcSpeedV((*func)(ThreeJet(u, 1, 0), ThreeJet(0, 0, 1), t));
    if(speedv[j] == 0) {
      /* Perturb a bit, hoping to avoid degeneracy */
      u += (u<1) ? 1e-9 : -1e-9;
      speedv[j] = calcSpeedV((*func)(ThreeJet(u, 1, 0), ThreeJet(0, 0, 1), t));
    }
    for (k = 0; k <= kmax; k++) {
      v = vmin + k*dv;
      values[j][k] = (*func)(
       ThreeJet(u, 1, 0),
       ThreeJet(v, 0, 1),
       t
      );
      speedu[j][k] = calcSpeedU(values[j][k]);
    }
  }
/*
  fprintf(fp, "Declare \"speeds\" \"varying float\"\n");
  fprintf(fp, "Declare \"speedt\" \"varying float\"\n");
*/
  if(parts != NULL) {
    /* Construct matrices to replicate standard unit (u=0..1, v=0..1) into
     * complete sphere.
     */
    char *partlist = parse_parts(parts);

    if(partlist == NULL)
	return;

    fprintf(fp, "{ INST transforms { TLIST\n");
    for(j = -1; j <= 1; j += 2) {
	for(k = 0; k < n_strips; k++) {
	  if(partlist[k] & (j<0 ? PART_NEG : PART_POS)) {
	    double t = 2*M_PI * (j < 0 ? n_strips-1-k : k) / n_strips;
	    double s = sin(t), c = cos(t);

	    fprintf(fp, "# %c%d of %d\n", j<0 ? '-' : '+', k, n_strips);
	    fprintf(fp, "\t%10f %10f %10f %10f\n", j*c, -s,	     0., 0.);
	    fprintf(fp, "\t%10f %10f %10f %10f\n", j*s,  c,	     0., 0.);
	    fprintf(fp, "\t%10f %10f %10f %10f\n", 0.,   0., (double)j, 0.);
	    fprintf(fp, "\t%10f %10f %10f %10f\n", 0.,   0.,	     0., 1.);
	  }
	}
    }
    fprintf(fp, "}\ngeom ");
  }

  if(bezier) {
    fprintf(fp, "{ STBBP%s\n", binary ? " BINARY" : "");
    for (j = 0; j < jmax; j++) {
      u = umin + j*du;
      for (k = 0; k < kmax; k++) {
	v = vmin + k*dv;
	printSpline(fp, values[j][k], values[j][k+1],
		  values[j+1][k], values[j+1][k+1],
		  du, dv,
		  umin+j*du, umin+(j+1)*du,  vmin+k*dv, vmin+(k+1)*dv, binary);
      }
    }
  } 
  else {
    int nu = kmax+1, nv = jmax+1;
    fprintf(fp, "{ NMESH%s\n", binary ? " BINARY" : "");

    if(binary) {
	fwrite(&nu, sizeof(int), 1, stdout);
	fwrite(&nv, sizeof(int), 1, stdout);
    } else {
	fprintf(fp, "%d %d\n", nu, nv);
    }
    for(j = 0; j <= jmax; j++) {
	for(k = 0; k <= kmax; k++)
	    printMesh(fp, values[j][k], binary);
	if(!binary)
	    fputc('\n', fp);
    }
  }
  if(parts)
    fprintf(fp, " }\n");
  fprintf(fp, "}\n");
}

void impossible(char *msg) {
  fprintf(stderr, "%s\n", msg);
  exit(1);
}

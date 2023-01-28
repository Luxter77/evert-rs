/// Got this form somewhere on stack overflow ... :P

#[derive(Debug, Clone, Copy)]
pub struct CGFloat(f64);

const MAX_PRECISION: usize = 15;

impl CGFloat {
    fn format(&self, f: &mut std::fmt::Formatter<'_>, force_exponent: bool) -> std::fmt::Result {
        let sigdigs: u8 = f.precision().unwrap_or(8).max(1).min(MAX_PRECISION).try_into().unwrap();
        let num = round(self.0, sigdigs.into());
        if force_exponent || magnitude(num) > sigdigs.into() || (num != 0.0 && num.abs() < 0.0001) {
            write!(f, "{:e}", num)
        } else {
            write!(f, "{}", num)
        }
    }
}

impl From<f64> for CGFloat {
    fn from(src: f64) -> Self { Self(src) }
}

/// The number of digits before the decimal dot.
fn magnitude(num: f64) -> i32 {
    if num == 0.0 || !num.is_normal() { 0 }
    else { num.abs().log10().floor() as i32 + 1 }
}

/// Round a number to a certain number of significant digits.
///
/// This function unfortunately allocates. It's possible to rewrite it to use
/// a [u8] array on the stack, but that requires unsafe and saves negligible
/// runtime according to cargo bench.
fn round(num: f64, sigdigs: usize) -> f64 {
    if num == 0.0 {
        // The method below drops the sign of -0.0
        return num;
    }
    let sigdigs = sigdigs.min(MAX_PRECISION).saturating_sub(1);
    format!("{0:.1$e}", num, sigdigs)
        .parse()
        .expect("Invalid builtin formatting output")
}

impl std::fmt::Display for CGFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.format(f, false)
    }
}

impl std::fmt::LowerExp for CGFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.format(f, true)
    }
}

mod tests {
    #[test]
    fn main() {
        assert_eq!("42", format!("{}", crate::c_gformat::CGFloat(42.)));
        assert_eq!("1.23", format!("{:.3}", crate::c_gformat::CGFloat(1.2345)));
        assert_eq!("-1.23", format!("{:.3}", crate::c_gformat::CGFloat(-1.2345)));
        assert_eq!("1.2345679e8", format!("{}", crate::c_gformat::CGFloat(123456789.0)));
        assert_eq!("-1.2345679e8", format!("{}", crate::c_gformat::CGFloat(-123456789.0)));
        assert_eq!("1.23e2", format!("{:+e}", crate::c_gformat::CGFloat(123.0)));
        assert_eq!("-1.23e2", format!("{:+e}", crate::c_gformat::CGFloat(-123.0)));
        assert_eq!("-0", format!("{}", crate::c_gformat::CGFloat(-0.0)));
        assert_eq!("0", format!("{}", crate::c_gformat::CGFloat(0.0)));
    }
}
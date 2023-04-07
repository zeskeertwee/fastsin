pub struct FastSin {
    precision_mul: f64,
    sin: Vec<f64>,
    cos: Vec<f64>
}

impl FastSin {
    /// The step size will be 10^-precision, and the lut size 360.0/step_size
    pub fn build(precision: u8) -> Self {
        let precision_digits = precision;
        let precision = 10.0_f64.powf(-(precision_digits as f64));
        let lut_size = (360.0 / precision) as usize + 1;

        Self {
            precision_mul: 10.0_f64.powf(precision_digits as f64),
            sin: build_lut(f64::sin, precision, lut_size),
            cos: build_lut(f64::cos, precision, lut_size)
        }
    }

    #[inline(always)]
    pub fn wsin(&self, angle_deg: f64) -> &f64 {
        self.sin(angle_deg % 360.0)
    }

    #[inline(always)]
    pub fn sin(&self, angle_deg: f64) -> &f64 {
        unsafe { self.sin.get_unchecked(self.get_idx(angle_deg)) }
    }

    #[inline(always)]
    pub fn wcos(&self, angle_deg: f64) -> &f64 {
        self.cos(angle_deg % 360.0)
    }

    #[inline(always)]
    pub fn cos(&self, angle_deg: f64) -> &f64 {
        unsafe { self.cos.get_unchecked(self.get_idx(angle_deg)) }
    }

    #[inline(always)]
    fn get_idx(&self, angle_deg: f64) -> usize {
        fast_round(angle_deg * self.precision_mul) as _
    }
}

fn build_lut<F>(f: F, step_size: f64, count: usize) -> Vec<f64>
where
    F: Fn(f64) -> f64
{
    let mut lut = Vec::with_capacity(count);

    for i in 0..count {
        lut.push((f)((i as f64 * step_size).to_radians()))
    }

    lut
}

#[inline(always)]
/// https://stackoverflow.com/questions/17035464/a-fast-method-to-round-a-double-to-a-32-bit-int-explained
fn fast_round(mut x: f64) -> usize {
    x += 6755399441055744.0;
    (unsafe { std::mem::transmute::<f64, (u32, u32)>(x) }).0 as usize
}

#[test]
fn idx_test() {
    let fs = FastSin::build(2);

    assert_eq!(fs.get_idx(0.0), 0);
    assert_eq!(fs.get_idx(180.0),  18000);
    assert_eq!(fs.get_idx(27.872), 2787);
    assert_eq!(fs.get_idx(98.81871), 9882);
}

#[test]
fn fast_round_test() {
    assert_eq!(32.001_f64.round() as usize, fast_round(32.001));
    assert_eq!(11.5_f64.round() as usize, fast_round(11.5));
    assert_eq!(87123.8178_f64.round() as usize, fast_round(87123.8178));
}
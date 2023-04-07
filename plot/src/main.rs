use std::fs::File;
use std::io::{BufWriter, Write};
use fastsin::FastSin;

fn main() {
    let fs = FastSin::build(2);

    let file = File::create("./data.csv").unwrap();
    let mut wtr = BufWriter::new(file);
    write!(wtr, "angle, fsin, fcos, sin, cos, dsin, dcos").unwrap();
    wtr.write(&[0x0A]).unwrap();

    for i in 0_usize..(360 * 10_usize.pow(2)) {
        let angle = i as f64 / (10.0_f64.powf(2.0));

        let fsin = fs.sin(angle);
        let fcos = fs.cos(angle);
        let ssin = f64::sin(angle.to_radians());
        let scos = f64::cos(angle.to_radians());

        write!(wtr, "{}, {}, {}, {}, {}, {}, {}/n", angle, fsin, fcos, ssin, scos, fsin - ssin, fcos - scos).unwrap();
        wtr.write(&[0x0A]).unwrap();
    }

    wtr.flush().unwrap();
}

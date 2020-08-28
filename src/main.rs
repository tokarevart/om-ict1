use std::ops::Range;
use std::fs::File;
use std::io::Write;

fn apply_3_methods(range: Range<f64>, eps: f64, 
    f: impl Fn(f64) -> f64, der: impl Fn(f64) -> f64, der2: impl Fn(f64) -> f64) {

    let delta = f32::EPSILON as f64;
    let x = om_bis::search(range.clone(), delta, eps, |x| f(x));
    println!("bisection search");
    println!("x                : {}", x);
    println!("x^2+3*x*(ln(x)-1): {}", f(x));
    println!("");

    let ltz = 5.0;
    let x = om_pl::search(range.clone(), ltz, eps, |x| f(x));
    println!("polyline search");
    println!("x                : {}", x);
    println!("x^2+3*x*(ln(x)-1): {}", f(x));
    println!("");

    let init = 0.5 * (range.start + range.end);
    let x = om_nt::search(range.clone(), init, eps, der, der2);
    println!("newton search");
    println!("x                : {}", x);
    println!("x^2+3*x*(ln(x)-1): {}", f(x));
    println!("");
}

fn apply_bis_on_3_ranges(ranges: [Range<f64>; 3], eps: f64, f: impl Fn(f64) -> f64) {
    let delta = f32::EPSILON as f64;
    for i in 0..3 {
        let x = om_bis::search(ranges[i].clone(), delta, eps, |x| f(x));
        println!("bisection search, section {}", i + 1);
        println!("x                : {}", x);
        println!("x^2+3*x*(ln(x)-1): {}", f(x));
        println!("");
    }
}

fn apply_nt_on_3_ranges(ranges: [Range<f64>; 3], eps: f64,
    f: impl Fn(f64) -> f64, der: impl Fn(f64) -> f64, der2: impl Fn(f64) -> f64) {

    for i in 0..3 {
        let init = 0.5 * (ranges[i].start + ranges[i].end);
        let x = om_nt::search(ranges[i].clone(), init, eps, |x| der(x), |x| der2(x));
        println!("newton search, section {}", i + 1);
        println!("x                : {}", x);
        println!("x^2+3*x*(ln(x)-1): {}", f(x));
        println!("");
    }
}

fn before_comparison() {
    let f    = |x: f64| x * x + 3.0 * x * (x.ln() - 1.0);
    let der  = |x: f64| 2.0 * x + 3.0 * x.ln();
    let der2 = |x: f64| 2.0 + 3.0 / x;
    let range = 0.5..1.0;
    let eps = 1e-4;
    apply_3_methods(range, eps, f, der, der2);

    let f    = |x: f64| 2.0 * x * x.cos().powi(2) + 2.0;
    let der  = |x: f64| 2.0 * x.cos().powi(2) - 2.0 * x * (2.0 * x).sin();
    let der2 = |x: f64| -4.0 * ((2.0 * x).sin() + x * (2.0 * x).cos());
    let range = -1.5..6.0;
    let ranges = [
        -1.5..0.6532711870944031,
        0.6532711870944031..3.2923100212820864,
        3.2923100212820864..6.0,
    ];
    let eps = 1e-5;
    apply_bis_on_3_ranges(ranges.clone(), eps, |x| f(x));
    let ranges = [
        -1.1444648640517021..0.0,
        1.1444648640517021..2.543492547051135,
        4.048081801611461..5.586352934164992,
    ];
    apply_nt_on_3_ranges(ranges, eps, f, der, der2);
    let ltz = 15.0;
    let x = om_pl::search(range, ltz, eps, f);
    println!("polyline search");
    println!("x                : {}", x);
    println!("x^2+3*x*(ln(x)-1): {}", f(x));
    println!("");
}

fn main() {
    // before_comparison();

    let f    = |x: f64| x * x + 3.0 * x * (x.ln() - 1.0);
    let der  = |x: f64| 2.0 * x + 3.0 * x.ln();
    let der2 = |x: f64| 2.0 + 3.0 / x;
    let range = 0.5..1.0;

    let etalon_x = 0.6488441332979994;
    let etalon_y = -2.3675311092092306;
    let mut bis_file = File::create("bis.txt").unwrap();
    let mut pl_file = File::create("pl.txt").unwrap();
    let mut nt_file = File::create("nt.txt").unwrap();
    for n in 1..=30 {
        let delta = f32::EPSILON as f64;
        let x = om_bis::search_with_n(range.clone(), delta, n, |x| f(x));
        writeln!(&mut bis_file, "{}\t{}", n, (x - etalon_x).abs()).unwrap();
        // writeln!(&mut bis_file, "{}\t{}", n, (f(x) - etalon_y).abs()).unwrap();

        let ltz = 2.05;
        let x = om_pl::search_with_n(range.clone(), ltz, n, |x| f(x));
        writeln!(&mut pl_file, "{}\t{}", n, (x - etalon_x).abs()).unwrap();
        // writeln!(&mut pl_file, "{}", (f(x) - etalon_y).abs()).unwrap();

        let init = range.start + f32::EPSILON as f64;
        let x = om_nt::search_with_n(range.clone(), init, n, |x| der(x), |x| der2(x));
        writeln!(&mut nt_file, "{}\t{}", n, (x - etalon_x).abs()).unwrap();
        // writeln!(&mut nt_file, "{}", (f(x) - etalon_y).abs()).unwrap();
    }
}

fn main() {
    let f = |x: f64| x * x + 3.0 * x * (x.ln() - 1.0);
    let range = 0.5..1.0;
    let eps = 1e-4;

    let delta = f32::EPSILON as f64;
    let x = om_bis::search(range, delta, eps, f);
    println!("x                : {}", x);
    println!("x^2+3*x*(ln(x)-1): {}", x);
}

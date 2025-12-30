use clap::Parser;
use ndarray::array;
use ndarray_linalg::Determinant;

/**
 * This calculates the currents in a simple electrical circuit with two unknown currents.
 * The circuit is defined by two equations based on Kirchhoff's laws.
 */
fn main() {
    let args = Args::parse();

    let d_x: ndarray::Array2<f64> = array![
        [args.r1 + args.r3, -args.v1],
        [args.r3, -args.v2]
    ];
    let d_y: ndarray::Array2<f64> = array![
        [args.r3, -args.v1],
        [args.r2 + args.r3, -args.v2]
    ];
    let d: ndarray::Array2<f64> = array![
        [args.r1 + args.r3, args.r3],
        [args.r3, args.r2 + args.r3]
    ];

    println!("D:\n{}", d);
    println!("Dx:\n{}", d_x);
    println!("Dy:\n{}", d_y);

    let determinant_dx = -d_x.det().unwrap();
    let determinant_dy = d_y.det().unwrap();
    let determinant_d = d.det().unwrap();
    
    println!("I1: {}", determinant_dx / determinant_d);
    println!("I2: {}", determinant_dy / determinant_d);
    println!("I3: {}", (determinant_dx + determinant_dy) / determinant_d);
}

#[derive(clap::Parser)]
struct Args {
    // Voltage source 1
    #[arg(long, value_parser = clap::value_parser!(f64))]
    v1: f64,
    // Voltage source 2
    #[arg(long, value_parser = clap::value_parser!(f64))]
    v2: f64,
    // Resistance source 1
    #[arg(long, value_parser = clap::value_parser!(f64))]
    r1: f64,
    // Resistance source 2
    #[arg(long, value_parser = clap::value_parser!(f64))]
    r2: f64,
    // Resistance source 3 to ground
    #[arg(long, value_parser = clap::value_parser!(f64))]
    r3: f64,
}
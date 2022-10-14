use physics_solver::{Point, Solver, Vec3};

fn main() {
    let mut points = [Point {
        pos: Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        vel: Vec3 {
            x: 1.,
            y: 100.,
            z: 1.,
        },
    }; 1];

    let solver = Solver::new(9.8);

    // println!("{:?}", points);
    for _ in 0..100 {
        solver.solve(&mut points);
        // println!("{:?}", points);
    }
    // println!("{:?}", points);

    println!("Done!");
}

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Hail {
    x: i128,
    y: i128,
    z: i128,
    vx: i128,
    vy: i128,
    vz: i128,
}

pub fn soln() -> (u64, i64) {
    let input = include_str!("../input/24.txt");
    let hailstones = input
        .lines()
        .map(|line| {
            let (positions, velocities) = line.split_once(" @ ").unwrap();
            let (x, y, z) = positions
                .split(", ")
                .map(|s| s.trim().parse::<i128>().unwrap())
                .tuples()
                .exactly_one()
                .unwrap();
            let (vx, vy, vz) = velocities
                .split(", ")
                .map(|s| s.trim().parse::<i128>().unwrap())
                .tuples()
                .exactly_one()
                .unwrap();
            Hail {
                x,
                y,
                z,
                vx,
                vy,
                vz,
            }
        })
        .collect::<Vec<_>>();
    let range = (200000000000000f64, 400000000000000f64);
    let p1 = hailstones
        .iter()
        .combinations(2)
        .filter(|p| {
            let (h1, h2) = (p.first().unwrap(), p.last().unwrap());
            let x_intersection = ((h1.vx * h2.vx) * (h2.y - h1.y) + h1.vy * h2.vx * h1.x
                - h2.vy * h1.vx * h2.x) as f64
                / (h1.vy * h2.vx - h2.vy * h1.vx) as f64;
            let t_1 = (x_intersection - h1.x as f64) / h1.vx as f64;
            let t_2 = (x_intersection - h2.x as f64) / h2.vx as f64;
            let y_intersection = h1.y as f64 + h1.vy as f64 * t_1;
            t_1 >= 0.0
                && t_2 >= 0.0
                && x_intersection >= range.0
                && x_intersection <= range.1
                && y_intersection >= range.0
                && y_intersection <= range.1
        })
        .count();

    let p2 = {
        use z3::ast::{Ast, Int};

        let cfg = z3::Config::new();
        let ctx = z3::Context::new(&cfg);
        let solver = z3::Solver::new(&ctx);

        let x = Int::new_const(&ctx, "x");
        let y = Int::new_const(&ctx, "y");
        let z = Int::new_const(&ctx, "z");
        let vx = Int::new_const(&ctx, "vx");
        let vy = Int::new_const(&ctx, "vy");
        let vz = Int::new_const(&ctx, "vz");

        for h in hailstones {
            let hx = Int::from_i64(&ctx, h.x as i64);
            let hy = Int::from_i64(&ctx, h.y as i64);
            let hz = Int::from_i64(&ctx, h.z as i64);
            let hvx = Int::from_i64(&ctx, h.vx as i64);
            let hvy = Int::from_i64(&ctx, h.vy as i64);
            let hvz = Int::from_i64(&ctx, h.vz as i64);

            let t = Int::fresh_const(&ctx, "t");

            solver.assert(&t.ge(&Int::from_i64(&ctx, 0)));
            solver.assert(&(&hx + &hvx * &t)._eq(&(&x + &vx * &t)));
            solver.assert(&(&hy + &hvy * &t)._eq(&(&y + &vy * &t)));
            solver.assert(&(&hz + &hvz * &t)._eq(&(&z + &vz * &t)));
        }

        solver.check();
        let model = solver.get_model().unwrap();
        let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
        let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
        let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();
        x + y + z
    };

    (p1 as u64, p2)
}

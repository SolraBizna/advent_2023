use advent_2023::Point3f;
use rand::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Trajectory {
    position: Point3f,
    velocity: Point3f,
}

impl Trajectory {
    fn from_string(mut i: String) -> Trajectory {
        i = i.replace(' ', "");
        let (position, velocity) = i.split_once('@').unwrap();
        Trajectory {
            position: Point3f::from_str_or_panic(position),
            velocity: Point3f::from_str_or_panic(velocity),
        }
    }
}

fn min_dsq(pos1: Point3f, vel1: Point3f, pos2: Point3f, vel2: Point3f) -> f64 {
    println!("-----------------------");
    println!("-----------------------");
    println!("-----------------------");
    println!("-----------------------");
    println!("-----------------------");
    println!("-----------------------");
    println!("-----------------------");
    println!("-----------------------");
    let mut w = 1.0;
    let mut w_inc = 1.0;
    let mut dsq = ((pos1 + vel1 * w) - (pos2 + vel2 * w)).magnitude_squared();
    loop {
        dbg!(w, w_inc, dsq);
        let new_w = w + w_inc;
        let new_dsq = ((pos1 + vel1 * new_w) - (pos2 + vel2 * new_w))
            .magnitude_squared();
        if new_dsq >= dsq {
            break;
        } else {
            w_inc *= 2.0;
            w = new_w;
            dsq = new_dsq;
        }
    }
    w_inc *= 2.0;
    while w_inc >= 1.0 {
        dbg!(w, w_inc, dsq);
        for mul in [-1.0, 1.0] {
            let new_w = w + w_inc * mul;
            let new_dsq = ((pos1 + vel1 * new_w) - (pos2 + vel2 * new_w))
                .magnitude_squared();
            if new_dsq < dsq {
                w = new_w;
                dsq = new_dsq;
                break;
            }
        }
        w_inc /= 2.0;
    }
    dbg!(w, w_inc, dsq);
    dsq
}

fn rock_errors(
    rock_pos: Point3f,
    rock_vel: Point3f,
    trajectories: &'_ [Trajectory],
) -> impl '_ + Clone + Iterator<Item = f64> {
    trajectories.iter().map(move |trajectory| {
        min_dsq(rock_pos, rock_vel, trajectory.position, trajectory.velocity)
    })
}

fn perturbations() -> impl Iterator<Item = Point3f> {
    (-1..=1).flat_map(|x| {
        (-1..=1).flat_map(move |y| {
            (-1..=1)
                .map(move |z| (x, y, z))
                .filter(|(x, y, z)| *x != 0 || *y != 0 || *z != 0)
                .map(|(x, y, z)| Point3f {
                    x: x as f64,
                    y: y as f64,
                    z: z as f64,
                })
        })
    })
}

fn variation(wat: impl Clone + Iterator<Item = f64>) -> f64 {
    let count = wat.clone().count() as f64;
    let sum = wat.clone().sum::<f64>();
    let average = sum / count;
    wat.map(|x| {
        let d = x - average;
        d * d
    })
    .sum()
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let trajectories: Vec<Trajectory> =
        lines.map(Trajectory::from_string).collect();
    assert!(trajectories
        .iter()
        .all(|trajectory| trajectory.velocity.x != 0.0));
    let test_range = if trajectories[0].position.x < 1000000.0 {
        7.0..=27.0
    } else {
        200000000000000.0..=400000000000000.0
    };
    let mut total = 0;
    for a_index in 0..trajectories.len() {
        let a = &trajectories[a_index];
        for b_index in a_index + 1..trajectories.len() {
            let b = &trajectories[b_index];
            let a_slope = a.velocity.y / a.velocity.x;
            let b_slope = b.velocity.y / b.velocity.x;
            let a_intercept = a.position.y - a.position.x * a_slope;
            let b_intercept = b.position.y - b.position.x * b_slope;
            if a_slope == b_slope {
                assert_ne!(a_intercept, b_intercept);
                continue;
            }
            let intersection_x =
                (a_intercept - b_intercept) / (b_slope - a_slope);
            if (intersection_x - a.position.x).signum()
                != a.velocity.x.signum()
            {
                // past
                continue;
            }
            if (intersection_x - a.position.x).abs() < 1.0 {
                continue;
            }
            if (intersection_x - b.position.x).signum()
                != b.velocity.x.signum()
            {
                // past
                continue;
            }
            if (intersection_x - b.position.x).abs() < 1.0 {
                continue;
            }
            let intersection_y = intersection_x * a_slope + a_intercept;
            let intersection_y2 = intersection_x * b_slope + b_intercept;
            if test_range.contains(&intersection_x)
                && (test_range.contains(&intersection_y)
                    || test_range.contains(&intersection_y2))
            {
                eprintln!("{a_index} and {b_index}; y = {a_slope} x + {a_intercept}, y = {b_slope} x + {b_intercept}, intersect at {intersection_x},{intersection_y}");
                total += 1;
            }
        }
    }
    println!("Part 1 answer: {total}");
    let mut rng = thread_rng();
    let mut rock_pos = Point3f {
        x: 19.0,
        y: 13.0,
        z: 30.0,
    };
    let mut rock_vel = Point3f {
        x: -3.0,
        y: 1.0,
        z: 2.0,
    };
    /* Find a good velocity */
    let mut prev_error: f64 =
        variation(rock_errors(rock_pos, rock_vel, &trajectories));
    loop {
        println!(
            "{:10} ← {},{},{}",
            prev_error as i64,
            rock_vel.x as i64,
            rock_vel.y as i64,
            rock_vel.z as i64
        );
        println!(
            "{:?}",
            rock_errors(rock_pos, rock_vel, &trajectories)
                .collect::<Vec<f64>>()
        );
        let mut best_error = prev_error;
        let mut best_vel = rock_vel;
        for perturbation in perturbations() {
            if best_error == 0.0 {
                break;
            }
            let error = variation(rock_errors(
                rock_pos,
                rock_vel + perturbation,
                &trajectories,
            ));
            if error < best_error {
                best_error = error;
                best_vel = rock_vel + perturbation;
            }
        }
        if rock_vel == best_vel {
            panic!("Stuck!");
        }
        prev_error = best_error;
        rock_vel = best_vel;
    }
    println!("Velocity: {rock_vel:?}");
    return;
    let mut prev_error: f64 =
        rock_errors(rock_pos, rock_vel, &trajectories).sum();
    loop {
        /*
        println!(
            "Pos: {},{},{} Vel: {} {} {} Err: {}",
            rock_pos.x as i64,
            rock_pos.y as i64,
            rock_pos.z as i64,
            rock_vel.x as i64,
            rock_vel.y as i64,
            rock_vel.z as i64,
            prev_error as i64,
        );
        */
        eprint!("Err: {}           \r", prev_error);
        if prev_error == 0.0 {
            break;
        }
        let mut best_error = prev_error;
        let mut best_pos = rock_pos;
        let mut best_vel = rock_vel;
        for perturbation in perturbations() {
            if best_error == 0.0 {
                break;
            }
            let error =
                rock_errors(rock_pos + perturbation, rock_vel, &trajectories)
                    .sum();
            if error < best_error {
                best_error = error;
                best_pos = rock_pos + perturbation;
                best_vel = rock_vel;
            }
            let error =
                rock_errors(rock_pos, rock_vel + perturbation, &trajectories)
                    .sum();
            if error < best_error {
                best_error = error;
                best_pos = rock_pos;
                best_vel = rock_vel + perturbation;
            }
        }
        if best_error < prev_error {
            prev_error = best_error;
            rock_pos = best_pos;
            rock_vel = best_vel;
        } else if best_error == prev_error {
            // give up!
            //println!();
            rock_pos = Point3f {
                x: rng.gen_range(test_range.clone()).floor(),
                y: rng.gen_range(test_range.clone()).floor(),
                z: rng.gen_range(test_range.clone()).floor(),
            };
            rock_vel = Point3f {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
            prev_error = rock_errors(rock_pos, rock_vel, &trajectories).sum();
        }
    }
    println!("Part 2 answer: {}", rock_pos.x + rock_pos.y + rock_pos.z);
}

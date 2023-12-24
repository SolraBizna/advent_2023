use advent_2023::Point3f;

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
            if a_index == 0 && b_index == 1 {
                eprintln!("{a_index} and {b_index}; y = {a_slope} x + {a_intercept}, y = {b_slope} x + {b_intercept}");
            }
            if a_slope == b_slope {
                assert_ne!(a_intercept, b_intercept);
                continue;
            }
            if (a_intercept - a.position.x).signum() != a.velocity.x.signum() {
                // past
                continue;
            }
            if (a_intercept - a.position.x).abs() < 1.0 {
                continue;
            }
            if (b_intercept - b.position.x).signum() != b.velocity.x.signum() {
                // past
                continue;
            }
            if (b_intercept - b.position.x).abs() < 1.0 {
                continue;
            }
            let intersection_x =
                (a_intercept - b_intercept) / (b_slope - a_slope);
            let intersection_y = intersection_x * a_slope + a_intercept;
            let intersection_y2 = intersection_x * b_slope + b_intercept;
            /*if (intersection_y - intersection_y2).abs() > 0.01 {
                eprintln!(
                    "{a_index},{b_index} = {intersection_y}, {intersection_y2}"
                );
            }*/
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
}

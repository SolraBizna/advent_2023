use advent_2023::Point3;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Brick {
    ends: [Point3; 2],
}

impl Brick {
    fn from_str_or_panic(input: &str) -> Brick {
        let (leftbrick, rightbrick) = input.split_once('~').unwrap();
        Brick {
            ends: [
                Point3::from_str_or_panic(leftbrick),
                Point3::from_str_or_panic(rightbrick),
            ],
        }
    }
    fn fall(&self) -> Brick {
        Brick {
            ends: self.ends.map(|end| Point3 {
                z: end.z - 1,
                ..end
            }),
        }
    }
    fn cells(&self) -> impl '_ + Iterator<Item = Point3> {
        let mut つぎ = Some(self.ends[0]);
        let dir = (self.ends[1] - self.ends[0]).unit();
        std::iter::from_fn(move || {
            let ret = つぎ;
            if ret == Some(self.ends[1]) {
                つぎ = None;
            } else {
                つぎ = つぎ.map(|x| x + dir);
            }
            ret
        })
    }
    fn overlaps_with(&self, other_brick: &Brick) -> bool {
        // TODO: if this isn't fast enough, do a good job :(
        self.cells().any(|mycellf| {
            other_brick.cells().any(|othercell| mycellf == othercell)
        })
    }
    fn min_z(&self) -> i32 {
        self.ends[0].z.min(self.ends[1].z)
    }
    fn max_z(&self) -> i32 {
        self.ends[0].z.max(self.ends[1].z)
    }
}

#[derive(Clone)]
struct Brickstore {
    bricks: Vec<Brick>,
    z_levels: Vec<Vec<usize>>,
}

impl Brickstore {
    fn with_bricks(bricks: Vec<Brick>) -> Brickstore {
        let mut z_levels = vec![];
        for (n, brick) in bricks.iter().enumerate() {
            for z in brick.min_z()..=brick.max_z() {
                while z_levels.len() <= z as usize {
                    z_levels.push(vec![]);
                }
                z_levels[z as usize].push(n);
            }
        }
        Brickstore { bricks, z_levels }
    }
    fn brick_can_fall(
        &self,
        brick_index: usize,
        ignored_index: Option<usize>,
    ) -> bool {
        let brick = &self.bricks[brick_index];
        let min_z = brick.ends[0].z.min(brick.ends[1].z);
        if min_z == 0 {
            // GROUND!!!
            return false;
        }
        let below_z = (min_z - 1) as usize;
        let fallen_brick = brick.fall();
        for other_brick in self.z_levels[below_z].iter().copied() {
            if Some(other_brick) == ignored_index {
                continue;
            }
            if self.bricks[other_brick].overlaps_with(&fallen_brick) {
                return false;
            }
        }
        true
    }
    /// Returns true if any bricks fell.
    fn perform_fall(&mut self) -> bool {
        let mut bricks_that_may_fall = Vec::with_capacity(128);
        let mut any_fell = false;
        for z in 0..self.z_levels.len() {
            bricks_that_may_fall.clear();
            for brick_index in self.z_levels[z].iter().copied() {
                if self.brick_can_fall(brick_index, None) {
                    bricks_that_may_fall.push(brick_index);
                }
            }
            for brick_index in bricks_that_may_fall.drain(..) {
                let brick = &mut self.bricks[brick_index];
                let gonez = brick.max_z();
                *brick = brick.fall();
                let comez = brick.min_z();
                self.z_levels[gonez as usize].retain(|x| *x != brick_index);
                self.z_levels[comez as usize].push(brick_index);
                any_fell = true;
            }
        }
        while self.z_levels.last().map(|x| x.is_empty()).unwrap_or(false) {
            self.z_levels.pop();
        }
        any_fell
    }
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let all_bricks: Vec<Brick> =
        lines.map(|x| Brick::from_str_or_panic(&x)).collect();
    assert!(all_bricks.iter().all(|brick| {
        let eq = (brick.ends[0].x == brick.ends[1].x) as u8
            + (brick.ends[0].y == brick.ends[1].y) as u8
            + (brick.ends[0].z == brick.ends[1].z) as u8;
        eq >= 2 && brick.ends[0].z.min(brick.ends[1].z) >= 0
    }));
    let mut brickstore = Brickstore::with_bricks(all_bricks.clone());
    while brickstore.perform_fall() {}
    println!(
        "Part 1 answer: {}",
        (0..brickstore.bricks.len())
            .filter(|candidate_index| {
                let candidate_index = *candidate_index;
                let candidate = &brickstore.bricks[candidate_index];
                let abovez = (candidate.max_z() + 1) as usize;
                if abovez < brickstore.z_levels.len() {
                    // There can be no bricks above us.
                    for other_brick in
                        brickstore.z_levels[abovez].iter().copied()
                    {
                        if brickstore
                            .brick_can_fall(other_brick, Some(candidate_index))
                        {
                            return false;
                        }
                    }
                }
                // If we get here, we don't allow any other brick to fall.
                true
            })
            .count()
    );
    println!(
        "Part 2 answer: {}",
        (0..brickstore.bricks.len())
            .map(|candidate_index| {
                let mut teststore = brickstore.clone();
                // Zap the test brick!
                teststore.bricks[candidate_index] = Brick {
                    ends: [
                        Point3 {
                            x: -999,
                            y: -999,
                            z: 0,
                        },
                        Point3 {
                            x: -999,
                            y: -999,
                            z: 0,
                        },
                    ],
                };
                while teststore.perform_fall() {}
                teststore
                    .bricks
                    .iter()
                    .zip(brickstore.bricks.iter())
                    .filter(|(a, b)| *a != *b)
                    .count()
                    - 1
            })
            .sum::<usize>()
    );
}

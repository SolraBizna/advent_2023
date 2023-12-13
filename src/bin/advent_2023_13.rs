use std::collections::HashSet;

use advent_2023::{Point, Tilemap};

fn get_all_reflections(
    slice: &[bool],
    ignored_candidate: Option<usize>,
) -> HashSet<usize> {
    (1..slice.len())
        .filter(|candidate_x| {
            if Some(*candidate_x) == ignored_candidate {
                return false;
            }
            for x in 0..*candidate_x {
                let reflected_x = candidate_x + (candidate_x - x) - 1;
                if reflected_x >= slice.len() {
                    continue;
                }
                if slice[reflected_x] != slice[x] {
                    return false;
                }
            }
            true
        })
        .collect()
}

fn get_h_reflection(
    tilemap: &Tilemap<bool>,
    ignored_candidate: Option<usize>,
) -> Option<usize> {
    let mut reflections: Option<HashSet<usize>> = None;
    for y in 0..tilemap.get_height() {
        let new_reflections = get_all_reflections(
            tilemap.get_row(y).unwrap(),
            ignored_candidate,
        );
        if let Some(reflections) = reflections.as_mut() {
            reflections.retain(|x| new_reflections.contains(x));
        } else {
            reflections = Some(new_reflections);
        }
    }
    reflections
        .filter(|x| x.len() == 1)
        .map(|reflections| *reflections.iter().next().unwrap())
}

fn get_v_reflection(
    tilemap: &Tilemap<bool>,
    ignored_candidate: Option<usize>,
) -> Option<usize> {
    get_h_reflection(&tilemap.transpose(), ignored_candidate)
}

fn main() {
    let lines = std::io::stdin().lines().map(|x| x.unwrap());
    let mut buf = Vec::with_capacity(32); // 32 is a nice number.
    let mut tilemaps: Vec<Tilemap<bool>> = vec![Tilemap::new_empty()];
    for line in lines {
        if line.is_empty() {
            tilemaps.push(Tilemap::new_empty());
        } else {
            buf.clear();
            buf.extend(line.chars().map(|x| x == '#'));
            tilemaps.last_mut().unwrap().add_row(&buf);
        }
    }
    println!(
        "Puzzle 1 answer: {}",
        tilemaps
            .iter()
            .map(|x| {
                get_h_reflection(x, None).unwrap_or(0)
                    + get_v_reflection(x, None).map(|x| x * 100).unwrap_or(0)
            })
            .sum::<usize>()
    );
    println!(
        "Puzzle 2 answer: {}",
        tilemaps
            .iter_mut()
            .map(|tilemap| {
                let mut different_answer = None;
                let mut pamelit = tilemap.transpose();
                let h_orig = get_h_reflection(tilemap, None);
                let v_orig = get_h_reflection(&pamelit, None);
                let same_answer =
                    h_orig.unwrap_or(0) + v_orig.map(|x| x * 100).unwrap_or(0);
                'outer: for y in 0..tilemap.get_height() {
                    for x in 0..tilemap.get_width() {
                        *tilemap.get_tile_mut(Point { x, y }).unwrap() ^= true;
                        let h_answer =
                            get_h_reflection(tilemap, h_orig).unwrap_or(0);
                        *tilemap.get_tile_mut(Point { x, y }).unwrap() ^= true;
                        *pamelit
                            .get_tile_mut(Point { y: x, x: y })
                            .unwrap() ^= true;
                        let v_answer = get_h_reflection(&pamelit, v_orig)
                            .map(|x| x * 100)
                            .unwrap_or(0);
                        *pamelit
                            .get_tile_mut(Point { y: x, x: y })
                            .unwrap() ^= true;
                        let answer = h_answer + v_answer;
                        if answer != 0 && answer != same_answer {
                            different_answer = Some(answer);
                            break 'outer;
                        }
                    }
                }
                different_answer.unwrap()
            })
            .sum::<usize>()
    );
}

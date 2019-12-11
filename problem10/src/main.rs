use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashSet, BTreeMap};
use num::integer::gcd;
use std::cmp::Ordering;

use std::f64::consts::PI;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn distance(&self, other: &Point) -> i64 {
        let delta_x = self.x as i64 - other.x as i64;
        let delta_y = self.y as i64 - other.y as i64;

        delta_x * delta_x + delta_y * delta_y
    }

    fn cmp_distance(&self, other: &Point, target: &Point) -> Ordering {
        self.distance(target).cmp(&other.distance(target))
    }
}

fn main() {
    let input = read_input();
    let laser = part1(&input);
    let adjusted_asteroids = part2(&input, laser);

    let Point {x, y} = adjusted_asteroids[199].1;
    let result_x = x;
    let result_y = y;
    println!("part 2: {:?}", 100 * result_x + result_y);
}

fn part1(points: &Vec<Point>) -> Point {
    let mut best_point = Point { x: 0, y: 0 };
    let mut best_count: usize = 0;
    for point1 in points {
        let mut visibles: HashSet<(i16, i16)> = HashSet::new();
        for point2 in points {
            if point1.x != point2.x || point1.y != point2.y {
                let delta_x = point1.x as i16 - point2.x as i16;
                let delta_y = point1.y as i16 - point2.y as i16;
                let delta_gcd = gcd(delta_x, delta_y);
                visibles.insert((delta_x / delta_gcd, delta_y / delta_gcd));
            }
        }
        if visibles.len() > best_count {
            best_count = visibles.len();
            best_point = *point1;
        }
    }

    println!("part 1: {:?}", best_count);

    best_point
}

const HASHABLE_FACTOR: f64 = 10000.0;

fn hashable_angle(theta: f64) -> usize {
    (theta * HASHABLE_FACTOR).round() as usize
}

fn deltas_to_angle(delta_x: f64, delta_y: f64) -> f64 {
    let ratio = delta_x/delta_y;
    let theta = ratio.atan();

    let delta_x_pos= delta_x >= 0.0;
    let delta_y_pos= delta_y >= 0.0;

    // convert to 0->2PI
    if !delta_y_pos {
        theta + PI
    } else if !delta_x_pos {
        theta + 2.0 * PI
    } else {
        theta
    }
}

fn part2(points: &Vec<Point>, laser: Point) -> Vec<(usize, Point)> {
    let mut asteroids = BTreeMap::new();

    //let angle_tallies: HashMap<i64, u8> = HashMap::new();

    for point in points {

        if point.x == laser.x && point.y == laser.y {
            continue;
        }
        let delta_x = point.x as f64 - laser.x as f64;
        let delta_y = laser.y as f64 - point.y as f64;

        let theta = deltas_to_angle(delta_x, delta_y);

        //println!("({},{}) -> ({},{}) -> {}", point.x, point.y, delta_x, delta_y, theta);

        asteroids.entry(hashable_angle(theta))
            .and_modify(|v: &mut Vec<Point>|v.push(*point))
            .or_insert(vec!(*point));
    }

    for v in asteroids.values_mut() {
        v.sort_by(|p1, p2| p1.cmp_distance(&p2, &laser));
    }

    let mut adjusted_asteroids: Vec<(usize, Point)> = asteroids.into_iter()
        .flat_map(move |(angle, v)|
            v.into_iter()
                .enumerate()
                .map(move |(i, p)|
                    (angle + hashable_angle((i * 2) as f64 * PI), p)).into_iter()
        )
        .collect();

    adjusted_asteroids.sort_by(|i1,i2| i1.0.cmp(&i2.0));

    //println!("{:?}", adjusted_asteroids);

    adjusted_asteroids
}

fn read_input() -> Vec<Point> {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let points: Vec<Point> = reader.lines().enumerate()
        .flat_map(move |(line_no, line)| line.expect("couldn't parse string").chars().enumerate()
            .filter_map(move |(char_no, c)| match c {
                '#' => Some(Point{ x: char_no, y: line_no }),
                _ => None
            }).collect::<Vec<Point>>().into_iter()
        )
        .collect();

    points
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let input = vec!(
            ".#....#####...#..",
            "##...##.#####..##",
            "##...#...#.#####.",
            "..#.....#...###..",
            "..#.#.....#....##"
        );

        let mut all_points = vec!();
        for (line_no, line) in input.iter().enumerate() {
            for (char_no, c) in line.chars().enumerate() {
                if c == '#' {
                    all_points.push(Point {x: char_no as usize, y: line_no as usize })
                }
            }
        }

        println!("{:?}", all_points);

        let laser = Point {x: 8, y: 3};
        let p = part2(&all_points, laser);

        println!("{:?}",p);
//        .#....###24...#..
//        ##...##.13#67..9#
//        ##...#...5.8####.
//        ..#.....X...###..
//        ..#.#.....#....##
        assert_eq!(p[1-1].1, Point{x: 8, y: 1});
        assert_eq!(p[2-1].1, Point{x: 9, y: 0});
        assert_eq!(p[3-1].1, Point{x: 9, y: 1});
        assert_eq!(p[4-1].1, Point{x: 10, y: 0});
        assert_eq!(p[5-1].1, Point{x: 9, y: 2});
        assert_eq!(p[6-1].1, Point{x: 11, y: 1});
        assert_eq!(p[7-1].1, Point{x: 12, y: 1});
        assert_eq!(p[8-1].1, Point{x: 11, y: 2});
        assert_eq!(p[9-1].1, Point{x: 15, y: 1});
//        .#....###.....#..
//        ##...##...#.....#
//        ##...#......1234.
//        ..#.....X...5##..
//        ..#.9.....8....76
        assert_eq!(p[8+1].1, Point{x: 12, y: 2});
        assert_eq!(p[8+2].1, Point{x: 13, y: 2});
        assert_eq!(p[8+3].1, Point{x: 14, y: 2});
        assert_eq!(p[8+4].1, Point{x: 15, y: 2});
        assert_eq!(p[8+5].1, Point{x: 12, y: 3});
        assert_eq!(p[8+6].1, Point{x: 16, y: 4});
        assert_eq!(p[8+7].1, Point{x: 15, y: 4});
        assert_eq!(p[8+8].1, Point{x: 10, y: 4});
        assert_eq!(p[8+9].1, Point{x: 4, y: 4});
//        .8....###.....#..
//        56...9#...#.....#
//        34...7...........
//        ..2.....X....##..
//        ..1..............
        assert_eq!(p[17+1].1, Point{x: 2, y: 4});
        assert_eq!(p[17+2].1, Point{x: 2, y: 3});
        assert_eq!(p[17+3].1, Point{x: 0, y: 2});
        assert_eq!(p[17+4].1, Point{x: 1, y: 2});
        assert_eq!(p[17+5].1, Point{x: 0, y: 1});
        assert_eq!(p[17+6].1, Point{x: 1, y: 1});
        assert_eq!(p[17+7].1, Point{x: 5, y: 2});
        assert_eq!(p[17+8].1, Point{x: 1, y: 0});
        assert_eq!(p[17+9].1, Point{x: 5, y: 1});
//        ......234.....6..
//        ......1...5.....7
//        .................
//        ........X....89..
//        .................
        assert_eq!(p[26+1].1, Point{x: 6, y: 1});
        assert_eq!(p[26+2].1, Point{x: 6, y: 0});
        assert_eq!(p[26+3].1, Point{x: 7, y: 0});
        assert_eq!(p[26+4].1, Point{x: 8, y: 0});
        assert_eq!(p[26+5].1, Point{x: 10, y: 1});
        assert_eq!(p[26+6].1, Point{x: 14, y: 0});
        assert_eq!(p[26+7].1, Point{x: 16, y: 1});
        assert_eq!(p[26+8].1, Point{x: 13, y: 3});
        assert_eq!(p[26+9].1, Point{x: 14, y: 3});
    }

    #[test]
    fn big_test() {
        let input = vec!(
            ".#..##.###...#######",
            "##.############..##.",
            ".#.######.########.#",
            ".###.#######.####.#.",
            "#####.##.#.##.###.##",
            "..#####..#.#########",
            "####################",
            "#.####....###.#.#.##",
            "##.#################",
            "#####.##.###..####..",
            "..######..##.#######",
            "####.##.####...##..#",
            ".#####..#.######.###",
            "##...#.##########...",
            "#.##########.#######",
            ".####.#.###.###.#.##",
            "....##.##.###..#####",
            ".#.#.###########.###",
            "#.#.#.#####.####.###",
            "###.##.####.##.#..##"
        );

        let mut all_points = vec!();
        for (line_no, line) in input.iter().enumerate() {
            for (char_no, c) in line.chars().enumerate() {
                if c == '#' {
                    all_points.push(Point {x: char_no as usize, y: line_no as usize })
                }
            }
        }

        println!("{:?}", all_points);

        let laser = Point {x: 11, y: 13};

        let adjusted_asteroids = part2(&all_points, laser);
        assert_eq!(adjusted_asteroids[0].1, Point{x: 11, y: 12});
        assert_eq!(adjusted_asteroids[1].1, Point{x: 12, y: 1});
        assert_eq!(adjusted_asteroids[2].1, Point{x: 12, y: 2});
        assert_eq!(adjusted_asteroids[9].1, Point{x: 12, y: 8});
        assert_eq!(adjusted_asteroids[19].1, Point{x: 16, y: 0});
        assert_eq!(adjusted_asteroids[49].1, Point{x: 16, y: 9});
        assert_eq!(adjusted_asteroids[99].1, Point{x: 10, y: 16});
        assert_eq!(adjusted_asteroids[198].1, Point{x: 9, y: 6});
        assert_eq!(adjusted_asteroids[199].1, Point{x: 8, y: 2});
        assert_eq!(adjusted_asteroids[200].1, Point{x: 10, y: 9});


    }
}

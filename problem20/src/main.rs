use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead};

struct Map {
    start: (usize, usize),
    end: (usize, usize),
    grid: Vec<Vec<char>>,
    teleports_inner: HashMap<(usize, usize), (usize, usize)>,
    teleports_outer: HashMap<(usize, usize), (usize, usize)>
}

fn main() {
    let map = parse_input();
    part1(&map);
    part2(&map);
}

fn part1(map: &Map) {
    let steps = steps_to_end(map, false);
    println!("part 1: {}", steps);
}

fn part2(map: &Map) {
    let steps = steps_to_end(map, true);
    println!("part 2: {}", steps);
}

fn steps_to_end(map: &Map, in_out_matters: bool) -> usize {
    let mut steps: usize = 0;

    let mut explore_from: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut newly_visited = HashSet::new();
    let mut visited = HashSet::new();

    visited.insert((map.start.0, map.start.1, 0));
    explore_from.insert((map.start.0, map.start.1, 0));


    loop {
        steps += 1;

        for (x,y,depth) in explore_from {
            let mut accessible = vec!();
            accessible.push((x+1,y,depth));
            accessible.push((x-1,y,depth));
            accessible.push((x,y+1,depth));
            accessible.push((x,y-1,depth));

            if depth > 0 {
                match map.teleports_outer.get(&(x, y)) {
                    Some(p) => {
                        if in_out_matters {
                            if !visited.contains(&(p.0, p.1, depth - 1)) {
                                newly_visited.insert((p.0, p.1, depth - 1));
                                visited.insert((p.0, p.1, depth - 1));
                            }
                        } else {
                            if !visited.contains(&(p.0, p.1, 0)) {
                                newly_visited.insert((p.0, p.1, 0));
                                visited.insert((p.0, p.1, 0));
                            }
                        }
                    },
                    None => ()
                }
            }

            match map.teleports_inner.get(&(x,y)) {
                Some(p) => {
                    if in_out_matters {
                        if !visited.contains(&(p.0, p.1, depth+1)) {
                            newly_visited.insert((p.0, p.1, depth+1));
                            visited.insert((p.0, p.1, depth+1));
                        }
                    } else {
                        if !visited.contains(&(p.0, p.1, 0)) {
                            newly_visited.insert((p.0, p.1, 0));
                            visited.insert((p.0, p.1, 0));
                        }
                    }
                },
                None => ()
            }

            for p in accessible {
                if p == (map.end.0, map.end.1, 0) {
                    return steps;
                }
                if map.grid[p.1][p.0] == '.' {
                    if !visited.contains(&(p.0, p.1, depth)) {
                        newly_visited.insert((p.0, p.1, depth));
                        visited.insert((p.0, p.1, depth));
                    }
                }
            }
        }


        explore_from = newly_visited;
        newly_visited = HashSet::new();


//        println!("visiting next turn via portal: {:?}", newly_visited);
//        println!("explore_from {:?}", explore_from);

//        for (line_ix, line) in map.grid.iter().enumerate() {
//            println!("{}",line.iter().enumerate().map(|(ix,c)| if visited.contains(&(ix, line_ix)) { 'Y' } else if explore_from.contains(&(ix, line_ix)) {'*'} else {*c}).collect::<String>());
//        }

//        println!("visited {} with {} to explore from after {} steps\n\n", visited.len(), explore_from.len(), steps);
        if explore_from.len() == 0 && newly_visited.len() == 0 {
            steps = 0;
            break;
        }
    }

    return steps;
}

fn parse_input() -> Map {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let grid: Vec<String> = reader.lines().map(|line| line.expect("failed to parse line").to_string()).collect();

    for line in &grid {
        println!("{}", line);
    }

    make_map(grid)
}

fn make_map(sgrid: Vec<String>) -> Map {
    let grid: Vec<Vec<char>> = sgrid.iter().map(|line|line.chars().collect()).collect();
    let width = grid[2].len();
    let height = grid.len();

//    let mut labels = vec!();
    let mut inner_portals = HashMap::new();
    let mut outer_portals = HashMap::new();

    let mut hole_x1: usize = 0;
    let mut hole_x2: usize = 0;
    let mut hole_y1: usize = 0;
    let mut hole_y2: usize = 0;

    for (line_ix, line) in grid.iter().enumerate() {
        for (char_ix, c) in line.iter().enumerate() {
            if *c != '.' && *c!= '#' && line_ix > 1 && line_ix < height-2 && char_ix > 1 && char_ix < width {
                if hole_x1 == 0 {
                    hole_x1 = char_ix;
                }
                hole_x2 = hole_x2.max(char_ix);

                if hole_y1 == 0 {
                    hole_y1 = line_ix;
                }
                hole_y2 = hole_y2.max(line_ix);
            }
        }
    }

//    println!("hole {},{} to {},{}", hole_x1, hole_y1, hole_x2, hole_y2);

    for (x, (c1, c2)) in grid[0].iter().zip(grid[1].iter()).enumerate() {
        if *c1 != ' ' {
            outer_portals.insert([*c1, *c2].iter().cloned().collect::<String>(),(x,2));
        }
    }

//    println!("labels: {}", labels.len());

    for (x, (c1, c2)) in grid[height-2].iter().zip(grid[height-1].iter()).enumerate() {
        if *c1 != ' ' {
            outer_portals.insert([*c1, *c2].iter().cloned().collect::<String>(),(x,height-3));
        }
    }
//    println!("labels: {}", labels.len());

    for y in 0..height {
        if grid[y][0] != ' ' {
            outer_portals.insert([grid[y][0], grid[y][1]].iter().cloned().collect::<String>(),(2, y));
        }
    }
//    println!("labels: {}", labels.len());

    for y in 0..height {
        if grid[y].len() > width {
            outer_portals.insert([grid[y][width], grid[y][width+1]].iter().cloned().collect::<String>(), (width-1, y));
        }
    }
    //println!("labels: {}", labels.len());

    for (x, (c1, c2)) in grid[hole_y1].iter().zip(grid[hole_y1+1].iter()).enumerate() {
        if *c1 != ' ' && x >= hole_x1 && x <= hole_x2 {
            inner_portals.insert([*c1, *c2].iter().cloned().collect::<String>(), (x,hole_y1-1));
        }
    }
    //println!("labels: {}", labels.len());

    for (x, (c1, c2)) in grid[hole_y2-1].iter().zip(grid[hole_y2].iter()).enumerate() {
        if *c1 != ' ' && x >= hole_x1 && x <= hole_x2 {
            inner_portals.insert([*c1, *c2].iter().cloned().collect::<String>(), (x,hole_y2+1));
        }
    }
    //println!("labels: {}", labels.len());

    for y in hole_y1..=hole_y2 {
        let c1 = grid[y][hole_x1];
        let c2 = grid[y][hole_x1+1];
        if c1 != ' ' {
//            println!("push1");
            inner_portals.insert([c1, c2].iter().cloned().collect::<String>(), (hole_x1-1,y));
        }

        let c3 = grid[y][hole_x2-1];
        let c4 = grid[y][hole_x2];
        if c3 != ' ' {
//            println!("push2");
            inner_portals.insert([c3, c4].iter().cloned().collect::<String>(), (hole_x2+1,y));
        }
    }
    println!("inner_portals: {}", inner_portals.len());
    println!("outer_portals: {}", outer_portals.len());

    //println!("{:?}", labels);

    let mut teleports_outer: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut teleports_inner: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut start = (0,0);
    let mut end = (0,0);

    for (label, outer_location) in &outer_portals {
        if label == "AA" {
            start = outer_location.clone();
        } else if label == "ZZ" {
            end = outer_location.clone()
        } else {
            let inner_location = inner_portals.get(label).expect("found inner without outer");
            teleports_outer.insert(outer_location.clone(), inner_location.clone());
            teleports_inner.insert(inner_location.clone(), outer_location.clone());
        }
    }

    Map {
        start,
        end,
        grid,
        teleports_inner,
        teleports_outer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = vec!(
        "         A",
        "         A",
        "  #######.#########",
        "  #######.........#",
        "  #######.#######.#",
        "  #######.#######.#",
        "  #######.#######.#",
        "  #####  B    ###.#",
        "BC...##  C    ###.#",
        "  ##.##       ###.#",
        "  ##...DE  F  ###.#",
        "  #####    G  ###.#",
        "  #########.#####.#",
        "DE..#######...###.#",
        "  #.#########.###.#",
        "FG..#########.....#",
        "  ###########.#####",
        "             Z",
        "             Z"
        ).iter().map(|st| st.to_string()).collect();

        let map = make_map(input);
        let steps = steps_to_end(&map);
        assert_eq!(steps, 23);
    }





    #[test]
    fn test2() {
        let input = vec!(
            "                   A",
            "                   A",
            "  #################.#############",
            "  #.#...#...................#.#.#",
            "  #.#.#.###.###.###.#########.#.#",
            "  #.#.#.......#...#.....#.#.#...#",
            "  #.#########.###.#####.#.#.###.#",
            "  #.............#.#.....#.......#",
            "  ###.###########.###.#####.#.#.#",
            "  #.....#        A   C    #.#.#.#",
            "  #######        S   P    #####.#",
            "  #.#...#                 #......VT",
            "  #.#.#.#                 #.#####",
            "  #...#.#               YN....#.#",
            "  #.###.#                 #####.#",
            "DI....#.#                 #.....#",
            "  #####.#                 #.###.#",
            "ZZ......#               QG....#..AS",
            "  ###.###                 #######",
            "JO..#.#.#                 #.....#",
            "  #.#.#.#                 ###.#.#",
            "  #...#..DI             BU....#..LF",
            "  #####.#                 #.#####",
            "YN......#               VT..#....QG",
            "  #.###.#                 #.###.#",
            "  #.#...#                 #.....#",
            "  ###.###    J L     J    #.#.###",
            "  #.....#    O F     P    #.#...#",
            "  #.###.#####.#.#####.#####.###.#",
            "  #...#.#.#...#.....#.....#.#...#",
            "  #.#####.###.###.#.#.#########.#",
            "  #...#.#.....#...#.#.#.#.....#.#",
            "  #.###.#####.###.###.#.#.#######",
            "  #.#.........#...#.............#",
            "  #########.###.###.#############",
            "           B   J   C",
            "           U   P   P"
        ).iter().map(|st| st.to_string()).collect();

        let map = make_map(input);
        let steps = steps_to_end(&map);
        assert_eq!(steps, 58);
    }
}
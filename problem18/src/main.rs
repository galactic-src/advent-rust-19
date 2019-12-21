use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
struct Map {
    edges: HashMap<char, Vec<Dest>>
}

#[derive(Debug)]
struct Dest {
    node: char,
    steps: usize,
    needs_keys: BTreeSet<char>
}

fn main() {
    //part1();
    part2();
}

fn part1() {
    let map = parse_input();
    let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
    let mut remaining_keys = keys.clone();
    remaining_keys.remove(&'@');
    let min_steps = visit_all_nodes(&map, &remaining_keys, vec!('@'));
    println!("part 1: {}", min_steps);
}

fn visit_all_nodes(map: &Map, remaining_keys: &BTreeSet<char>, current_nodes: Vec<char>) -> usize {

    let mut shortest_paths: HashMap<(Vec<char>, BTreeSet<char>), usize> = HashMap::new();
    shortest_paths.insert((current_nodes.clone(), remaining_keys.clone()), 0);
    let mut to_check_from: HashMap<(Vec<char>, BTreeSet<char>), usize> = HashMap::new();
    to_check_from.insert((current_nodes.clone(), remaining_keys.clone()), 0);

    loop {
        if to_check_from.len() == 0 {
            return *shortest_paths.iter()
                .filter(|((_c, keys), _steps)| keys.len() == 0)
                .map(|((_c, _keys), steps)|steps)
                .min()
                .expect("couldn't find overall min");
        }

        let first_key: (Vec<char>, BTreeSet<char>) = to_check_from.keys().nth(0).expect("should be something left").clone();

        let ((next_srcs, next_remaining_keys), next_steps) = to_check_from.remove_entry(&first_key).expect("only just saw this!");

        //println!("I am at {} with {} to check", next_srcs, to_check_from.len());
        //println!("We still need {} keys after {} steps: {:?}", next_remaining_keys.len(), next_steps, next_remaining_keys);
        //println!("We still need {} keys after {} steps", next_remaining_keys.len(), next_steps);

        let reachable = get_reachable(next_srcs, map, &next_remaining_keys);

        //println!("I can get to {:?}", reachable.keys().map(|c|*c).collect::<Vec<char>>());

        for (dests, additional_steps) in reachable {
            let mut new_remaining_keys = next_remaining_keys.clone();
            for dest in &dests {
                new_remaining_keys.remove(&dest);
            }
            let key = (dests.clone(), new_remaining_keys.clone());
            let mut total_steps = next_steps + additional_steps;

            shortest_paths.entry(key.clone())
                .and_modify(|existing_record| {
                    if existing_record > &mut total_steps {
                        //println!("replacing {} with {} for {:?}", existing_record, total_steps, key);
                        to_check_from.insert((dests.clone(), new_remaining_keys.clone()), total_steps);
                        *existing_record = total_steps;
                    } //else {
                        //println!("no need to replace ({} < {}): {:?}", existing_record, total_steps, key)
                    //}
                })
                .or_insert_with(|| {
                    //println!("inserting {} for {:?}", total_steps, key);
                    to_check_from.insert((dests, new_remaining_keys), total_steps);
                    total_steps
                });
        }
    }

//    if remaining_keys.len() == 0 {
//        return steps;
//    }
//
//    let reachable = get_reachable(current_node, map, remaining_keys);
//
//    reachable.iter()
//        .map(|(dest_node, dest_steps)| {
//            let mut reduced_remaining_keys = remaining_keys.clone();
//            reduced_remaining_keys.remove(&dest_node);
//            visit_all_nodes(map, &reduced_remaining_keys, steps+dest_steps, *dest_node)
//        }).min().expect("couldn't get a min")
}

fn parse_input() -> Map {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let grid: Vec<Vec<char>> = reader.lines().map(|line| line.expect("failed to parse line").chars().collect()).collect();

    make_map(grid)
}

fn make_map(grid: Vec<Vec<char>>) -> Map {
    let features: Vec<(usize, usize, char)> = grid.iter().enumerate()
        .flat_map(|(y, row)| row.iter().enumerate()
            .map(move |(x, c)|(x, y, *c)))
        .filter(|p|p.2 != '#' && p.2 != '.')
        .collect();
    //println!("{:?}", features);

    let nodes: Vec<char> = features.iter().filter(move|k| k.2.is_lowercase() || k.2 == '@' || k.2 == '*' || k.2 == '~' || k.2 == '?').map(|k|k.2).collect();
    //println!("{:?}", nodes);

    //let start = features.iter().filter(|n|n.2 == '@').map(|(x,y,_c)|(x,y)).nth(0).expect("couldn't find start");

    //println!("found start at {:?}\n", start);
    let mut map = Map {edges: HashMap::new()};

    for node in nodes {
        let (x,y) = features.iter().find(|feature|feature.2 == node).map(|(x,y,_c)|(x,y)).expect("node");

        let mut adjacent_keys: Vec<(char, usize, BTreeSet<char>)> = vec!();
        let mut visited = BTreeSet::new();
        visited.insert((*x,*y));
        //println!("\n\nfinding nearest keys for {}", node);
        find_nearest_keys(&mut adjacent_keys, *x, *y, &grid, 0, vec!(), &mut visited);

        let mut nearest_keys: HashMap<char, HashMap<BTreeSet<&char>, usize>> = HashMap::new();
        for adjacent_key in &adjacent_keys {
            let required_keys = &adjacent_key.2;
            nearest_keys.entry(adjacent_key.0).and_modify(
                |key_map| {key_map.entry(required_keys.iter().collect())
                    .and_modify(move |steps| *steps = adjacent_key.1.min(*steps))
                    .or_insert(adjacent_key.1);})
                .or_insert({
                    let mut map = HashMap::new();
                    map.insert(required_keys.iter().collect(), adjacent_key.1);
                    map
                });
        }

        let mut dests = vec!();

        for (dest, routes) in nearest_keys {
            for (needs_keys, steps) in routes {
                dests.push(Dest{
                    node: dest,
                    steps,
                    needs_keys: needs_keys.iter().map(|c|**c).collect()
                })
            }

        }

        map.edges.insert(node, dests);
    }

    //println!("{:?}", map.edges);

    map
}

fn get_reachable(from: Vec<char>, map: &Map, not_got_keys: &BTreeSet<char>) -> HashMap<Vec<char>, usize> {

    let mut best_distances = HashMap::new();
    best_distances.insert(from.clone(), 0);

    let mut to_check_from = HashMap::new();
    to_check_from.insert(from, 0);

    loop {
        if to_check_from.len() == 0 {
            break;
        }

        //println!("to_check_from({}): {:?}", to_check_from.len(), to_check_from);
        //println!("best_distances so far: {:?}", best_distances);

        let next_key = to_check_from.keys().nth(0).expect("shouldn't hit").clone();
        let (next_srcs, steps) = to_check_from.remove_entry(&next_key).expect("shouldn't hit");

        for (i, src) in next_srcs.iter().enumerate() {
            //println!("checking reachables for {}", src);
            for reachable_dest in &map.edges[&src] {
                if reachable_dest.needs_keys.intersection(&not_got_keys).count() > 0 {
                    continue;
                }

                let reachable_node = reachable_dest.node;
                let mut total_steps = steps + reachable_dest.steps;

                let mut new_srcs = next_srcs.clone();
                new_srcs[i] = reachable_node;

                if best_distances.contains_key(&new_srcs) &&
                    total_steps >= *best_distances.get(&new_srcs).expect("") {
                    continue;
                }

                best_distances.entry(new_srcs.clone())
                    .and_modify(|record| *record = *record.min(&mut (total_steps)))
                    .or_insert({
                        to_check_from.insert(new_srcs.clone(), total_steps);
                        total_steps
                    });
            }
        }

    }

    best_distances
}


fn find_nearest_keys(adjacent_keys: &mut Vec<(char, usize, BTreeSet<char>)>, node_x: usize, node_y: usize, grid: &Vec<Vec<char>>,
                     steps: usize, keys_required: Vec<char>, visited: &mut BTreeSet<(usize, usize)>){
    let mut x = node_x;
    let mut y = node_y;
    let mut steps = steps;
    let mut keys_required = keys_required.clone();
    let mut visited = visited.clone();


    let c = grid[y][x];
    println!("current char: {}", c);
    if visited.contains(&(x, y)){}
    else if c.is_lowercase() {
        // key
        adjacent_keys.push((c, steps, keys_required.iter().map(|c|*c).collect::<BTreeSet<char>>()));
        println!("found key {}", c);
        return;
    } else if c.is_uppercase() {
        keys_required.push((c as u8 - 'A' as u8 + 'a' as u8) as char);
//        println!("Requires key {}", c.to_lowercase());
    }

    loop {
        println!("at ({},{})", x, y);

        visited.insert((x,y));
        let next = vec!((x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1));
        println!("i have visited {:?}", visited);
        let next = next.into_iter()
            .filter(|(x, y)| grid[*y][*x] != '#' && !visited.contains(&(*x, *y)))
            .map(|(x,y)| (x,y,grid[y][x]))
            .collect::<Vec<(usize, usize,char)>>();

        println!("trying {:?} after {} steps", next, steps);

        if next.len() == 1 {
            x = next[0].0;
            y = next[0].1;
            steps += 1;
            visited.insert((x,y));

            let c = grid[y][x];
            println!("current char: {}", c);
            if c.is_lowercase() {
                // key
                println!("adding dest {} after {} steps requiring keys {:?}", c, steps, keys_required);
                adjacent_keys.push((c, steps, keys_required.iter().map(|c|*c).collect::<BTreeSet<char>>()));
                println!("found key {}", c);
                return;
            } else if c.is_uppercase() {
                println!("adding required key {}", c);
                keys_required.push((c as u8 - 'A' as u8 + 'a' as u8) as char);
                println!("Requires key {}", c.to_lowercase());
            }
        } else {
            for n in next {
                find_nearest_keys(adjacent_keys, n.0, n.1, grid, steps+1, keys_required.clone(), &mut visited);
            }
            return;
        }
    }
}

fn part2() {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = reader.lines().map(|line| line.expect("failed to parse line").chars().collect()).collect();

//    for (i, line) in grid.iter().enumerate() {
//        println!("{}: {}", i, line.iter().collect::<String>());
//    }
    grid[39][39] = '*';
    grid[39][40] = '#';
    grid[39][41] = '@';
    grid[40][39] = '#';
    grid[40][40] = '#';
    grid[40][41] = '#';
    grid[41][39] = '~';
    grid[41][40] = '#';
    grid[41][41] = '?';

    let map = make_map(grid);

    let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
    let mut remaining_keys = keys.clone();
    remaining_keys.remove(&'@');
    remaining_keys.remove(&'*');
    remaining_keys.remove(&'~');
    remaining_keys.remove(&'?');
    let min_steps = visit_all_nodes(&map, &remaining_keys, vec!('@', '*', '~', '?'));

    println!("part 2: {}", min_steps);
}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn test1() {
//        let lines = vec!(
//            "#########",
//            "#b.A.@.a#",
//            "#########"
//        );
//        for line in &lines {
//            println!("{}", line);
//        }
//
//        let grid: Vec<Vec<char>> = lines.iter().map(|l|l.chars().collect()).collect();
//        let map = make_map(grid);
//
//        println!("{:?}", map);
//
//        let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
//        let mut remaining_keys = keys.clone();
//        remaining_keys.remove(&'@');
//
//        let min_steps = visit_all_nodes(&map, &remaining_keys, '@');
//
//        assert_eq!(min_steps, 8);
//    }
//
//    #[test]
//    fn test2() {
//        let lines = vec!(
//            "########################",
//            "#f.D.E.e.C.b.A.@.a.B.c.#",
//            "######################.#",
//            "#d.....................#",
//            "########################"
//        );
//        for line in &lines {
//            println!("{}", line);
//        }
//
//        let grid: Vec<Vec<char>> = lines.iter().map(|l|l.chars().collect()).collect();
//        let map = make_map(grid);
//
//        println!("{:?}", map);
//
//        let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
//        let mut remaining_keys = keys.clone();
//        remaining_keys.remove(&'@');
//
//        let min_steps = visit_all_nodes(&map, &remaining_keys, '@');
//
//        assert_eq!(min_steps, 86);
//    }
//
//    #[test]
//    fn test3() {
//        let lines = vec!(
//            "########################",
//            "#...............b.C.D.f#",
//            "#.######################",
//            "#.....@.a.B.c.d.A.e.F.g#",
//            "########################"
//        );
//        for line in &lines {
//            println!("{}", line);
//        }
//
//        let grid: Vec<Vec<char>> = lines.iter().map(|l|l.chars().collect()).collect();
//        let map = make_map(grid);
//
//        println!("{:?}", map);
//
//        let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
//        let mut remaining_keys = keys.clone();
//        remaining_keys.remove(&'@');
//
//        let min_steps = visit_all_nodes(&map, &remaining_keys, '@');
//
//        assert_eq!(min_steps, 132);
//    }
//
//    #[test]
//    fn test5() {
//        let lines = vec!(
//        "########################",
//        "#@..............ac.GI.b#",
//        "###d#e#f################",
//        "###A#B#C################",
//        "###g#h#i################",
//        "########################");
//        for line in &lines {
//            println!("{}", line);
//        }
//
//        let grid: Vec<Vec<char>> = lines.iter().map(|l|l.chars().collect()).collect();
//        let map = make_map(grid);
//
//        println!("{:?}", map);
//
//        let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
//        let mut remaining_keys = keys.clone();
//        remaining_keys.remove(&'@');
//
//        let min_steps = visit_all_nodes(&map, &remaining_keys, '@');
//
//        assert_eq!(min_steps, 81);
//    }
//
//    #[test]
//    fn test4() {
//        let lines = vec!(
//            "#################",
//            "#i.G..c...e..H.p#",
//            "########.########",
//            "#j.A..b...f..D.o#",
//            "########@########",
//            "#k.E..a...g..B.n#",
//            "########.########",
//            "#l.F..d...h..C.m#",
//            "#################"
//        );
//        for line in &lines {
//            println!("{}", line);
//        }
//
//        let grid: Vec<Vec<char>> = lines.iter().map(|l|l.chars().collect()).collect();
//        let map = make_map(grid);
//
//        println!("{:?}", map);
//
//        let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
//        let mut remaining_keys = keys.clone();
//        remaining_keys.remove(&'@');
//
//        let min_steps = visit_all_nodes(&map, &remaining_keys, '@');
//
//        assert_eq!(min_steps, 136);
//    }

    #[test]
    fn test_part2() {
        let lines = vec!(
            "#######",
            "#a.#Cd#",
            "##@#*##",
            "#######",
            "##?#~##",
            "#cB#Ab#",
            "#######"
        );

        for line in &lines {
            println!("{}", line);
        }
        let grid: Vec<Vec<char>> = lines.iter().map(|l|l.chars().collect()).collect();
        let map = make_map(grid);

        println!("{:?}",map);

        let keys: BTreeSet<char> = map.edges.keys().map(|c|*c).collect();
        let mut remaining_keys = keys.clone();
        remaining_keys.remove(&'@');
        remaining_keys.remove(&'*');
        remaining_keys.remove(&'~');
        remaining_keys.remove(&'?');
        let min_steps = visit_all_nodes(&map, &remaining_keys, vec!('@', '*', '~', '?'));
        assert_eq!(min_steps, 8);
    }
}
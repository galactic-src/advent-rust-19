use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

extern crate nom;

use nom::{IResult,
          combinator::{map_res, opt},
          bytes::complete::{take_while_m_n, tag},
          multi::many1};

#[derive(Debug)]
struct ChemCount {
    chem: String,
    count: u32
}

type RecipeBook = HashMap<String, (u32, Vec<ChemCount>)>;

fn main() {
    let recipes: RecipeBook = read_input();
    let ore_required = get_ore_required(&recipes);
    //println!("{:?}", recipes);

    println!("part 1: {}", ore_required);

    let max = get_max_fuel(1000000000000, &recipes);
    println!("part2: {}", max);
}

fn get_ore_required(recipes: &RecipeBook) -> u32 {

    let mut required: HashMap<&str, u32> = HashMap::new();
    let mut spares: HashMap<&str, u32> = HashMap::new();
    let mut total_ore = 0;

    required.entry("FUEL").or_insert(1);

    loop {
        println!("\nrequired: {:?}", required);
        if required.len() == 0 {
            break;
        }

        let chem = required.keys().nth(0).expect("none left");
        let selected_chem = String::from(*chem);
        let (chem, required_count) = required.remove_entry(&selected_chem as &str).expect("wha");
        let mut still_required = required_count;

        //println!("{:?}", to_resolve);

        if chem == "ORE" {
            println!("adding {} to total_ore", still_required);
            total_ore += still_required;
            continue;
        }

        let recipe = &recipes[chem];

        println!("spares: {:?}", spares);

        spares.entry(&chem)
            .and_modify(|spare_count| if *spare_count >= still_required {
                    println!("looks like we can make it all from spares");
                    *spare_count -= still_required;
                    still_required = 0;
                } else {
                    println!("we can make {} from spares", *spare_count);
                    still_required -= *spare_count;
                    *spare_count = 0;
                });

        if still_required == 0 {
            continue;
        }

        let batches = (still_required as f64 / recipe.0 as f64).ceil() as u64;
        println!("to make {} {} if the recipe makes {}, we need {} batches", still_required, chem, recipe.0, batches);
        let produced = batches * recipe.0 as u64;
        if produced > still_required as u64 {
            spares.entry(&chem)
                .and_modify(|spare_count|
                    *spare_count += produced as u32 - still_required)
                .or_insert(produced as u32 - still_required);
        }
        for chem_count in &recipe.1 {
            println!("adding {:?}, {}, {}", chem_count, batches, chem_count.count * batches as u32);
            required.entry(&chem_count.chem)
                .and_modify(|already_required| *already_required += chem_count.count * batches as u32)
                .or_insert(chem_count.count * batches as u32);
            println!("{:?}", required);
        }
    }

    total_ore
}

fn get_max_fuel(mut ore_available: i64, recipes: &RecipeBook) -> u64 {
    let mut fuel_made: u64 = 0;
    let mut required: HashMap<&str, u32> = HashMap::new();
    let mut spares: HashMap<&str, u32> = HashMap::new();

    while ore_available > 0 {
        required.entry("FUEL").and_modify(|current|*current += 1).or_insert(1);
        fuel_made += 1;

        if fuel_made %1000000 == 0 {
            println!("{}", fuel_made);
        }

        loop {
            //println!("\nrequired: {:?}", required);
            if required.len() == 0 {
                break;
            }

            let chem = required.keys().nth(0).expect("none left");
            let selected_chem = String::from(*chem);
            let (chem, required_count) = required.remove_entry(&selected_chem as &str).expect("wha");
            let mut still_required = required_count;

            //println!("{:?}", to_resolve);

            if chem == "ORE" {
                //println!("adding {} to total_ore", still_required);
                ore_available -= still_required as i64;
                continue;
            }

            let recipe = &recipes[chem];

            //println!("spares: {:?}", spares);

            spares.entry(&chem)
                .and_modify(|spare_count| if *spare_count >= still_required {
                    //println!("looks like we can make it all from spares");
                    *spare_count -= still_required;
                    still_required = 0;
                } else {
                    //println!("we can make {} from spares", *spare_count);
                    still_required -= *spare_count;
                    *spare_count = 0;
                });

            if still_required == 0 {
                continue;
            }

            let batches = (still_required as f64 / recipe.0 as f64).ceil() as u64;
            //println!("to make {} {} if the recipe makes {}, we need {} batches", still_required, chem, recipe.0, batches);
            let produced = batches * recipe.0 as u64;
            if produced > still_required as u64 {
                spares.entry(&chem)
                    .and_modify(|spare_count|
                        *spare_count += produced as u32 - still_required)
                    .or_insert(produced as u32 - still_required);
            }
            for chem_count in &recipe.1 {
                //println!("adding {:?}, {}, {}", chem_count, batches, chem_count.count * batches as u32);
                required.entry(&chem_count.chem)
                    .and_modify(|already_required| *already_required += chem_count.count * batches as u32)
                    .or_insert(chem_count.count * batches as u32);
                //println!("{:?}", required);
            }
        }
    }

    fuel_made-1
}

fn read_input() -> RecipeBook {
    let file = File::open("src/input").unwrap();
    let reader: BufReader<File> = BufReader::new(file);

    reader.lines().map(|line| parse_recipe(&line.unwrap()).expect("recipe").1).collect()
}

fn is_al(c: char) -> bool {
    c.is_alphabetic()
}
fn is_digit(c: char) -> bool {
    c.is_digit(10)
}

fn parse_chem_count(input: &str) -> IResult<&str, ChemCount> {
    let (input, count) = parse_int(input)?;
    let (input, _) = tag(" ")(input)?;
    let (input, name) = take_while_m_n(1, 5, is_al)(input)?;
    let (input, _) = opt(tag(", "))(input)?;

    Ok((input, ChemCount { chem: String::from(name), count }))
}

fn parse_recipe(input: &str) -> IResult<&str, (String, (u32, Vec<ChemCount>))> {
    let (input, chem_counts) = many1(parse_chem_count)(input)?;
    let (input, _) = tag(" => ")(input)?;
    let (input, chem_info) = parse_chem_count(input)?;

    Ok((input, (chem_info.chem, (chem_info.count, chem_counts))))
}

fn parse_u32(input: &str) -> Result<u32, std::num::ParseIntError> {
    input.parse::<u32>()
}

fn parse_int(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(1, 3, is_digit),
        parse_u32
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let mut input = RecipeBook::new();
        input.insert("A".to_string(), (10u32, vec!(ChemCount {chem: "ORE".to_string(), count: 10})));
        input.insert("B".to_string(), (1, vec!(ChemCount {chem: "ORE".to_string(), count: 1})));
        input.insert("C".to_string(), (1, vec!(ChemCount {chem: "A".to_string(), count: 7}, ChemCount {chem: "B".to_string(), count: 1})));
        input.insert("D".to_string(), (1, vec!(ChemCount {chem: "A".to_string(), count: 7}, ChemCount {chem: "C".to_string(), count: 1})));
        input.insert("E".to_string(), (1, vec!(ChemCount {chem: "A".to_string(), count: 7}, ChemCount {chem: "D".to_string(), count: 1})));
        input.insert("FUEL".to_string(), (1, vec!(ChemCount {chem: "A".to_string(), count: 7}, ChemCount {chem: "E".to_string(), count: 1})));
        let ore_required = get_ore_required(&input);
        assert_eq!(ore_required, 31);
    }

    #[test]
    fn test_example2() {
        let mut input = RecipeBook::new();
        input.insert("A".to_string(), (2u32, vec!(ChemCount {chem: "ORE".to_string(), count: 9})));
        input.insert("B".to_string(), (3, vec!(ChemCount {chem: "ORE".to_string(), count: 8})));
        input.insert("C".to_string(), (5, vec!(ChemCount {chem: "ORE".to_string(), count: 7})));
        input.insert("AB".to_string(), (1, vec!(ChemCount {chem: "A".to_string(), count: 3}, ChemCount {chem: "B".to_string(), count: 4})));
        input.insert("BC".to_string(), (1, vec!(ChemCount {chem: "B".to_string(), count: 5}, ChemCount {chem: "C".to_string(), count: 7})));
        input.insert("CA".to_string(), (1, vec!(ChemCount {chem: "C".to_string(), count: 4}, ChemCount {chem: "A".to_string(), count: 1})));
        input.insert("FUEL".to_string(), (1, vec!(ChemCount {chem: "AB".to_string(), count: 2}, ChemCount {chem: "BC".to_string(), count: 3},
        ChemCount{chem: "CA".to_string(), count: 4})));
        let ore_required = get_ore_required(&input);
        assert_eq!(ore_required, 165);

    }
}
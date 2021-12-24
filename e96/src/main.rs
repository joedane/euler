use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io::{BufReader, BufRead};
use std::iter;
use std::fs::File;
use utf8_chars::BufReadCharsExt;

fn cross(a: &str, b: &str) -> Vec<String> {
    a.chars()
        .cartesian_product(b.chars())
        .map(|(a, b)| vec![a, b].into_iter().collect())
        .collect()
}

struct Config {
    rows: String,
    cols: String,
    digits: String,
    squares: Vec<String>,
    unitlist: Vec<Vec<String>>,
    units: HashMap<String, Vec<Vec<String>>>,
    peers: HashMap<String, HashSet<String>>,
}

impl Config {
    fn new() -> Self {
        let rows = "ABCDEFGHI";
        let digits = "123456789";
        let cols = digits.clone();
        let squares = cross(rows, cols);

        let mut unitlist_b: Vec<Vec<String>> =
            (0..9).map(|i| cross(rows, &cols[i..i + 1])).collect();
        unitlist_b.append(&mut (0..9).map(|i| cross(&rows[i..i + 1], cols)).collect());
        unitlist_b.append(
            &mut vec!["ABC", "DEF", "GHI"]
                .into_iter()
                .cartesian_product(vec!["123", "456", "789"].into_iter())
                .map(|(r, c)| cross(r, c))
                .collect(),
        );
        let unitlist = unitlist_b;

        let units: HashMap<String, Vec<Vec<String>>> = squares
            .iter()
            //  .cloned()
            .map(|s| {
                (
                    s.clone(),
                    unitlist
                        .iter()
                        .filter(|l| l.contains(&s))
                        .map(|l| l.to_owned())
                        .collect::<Vec<Vec<String>>>(),
                )
            })
            .collect();

        let peers: HashMap<String, HashSet<String>> = squares
            .iter()
            .map(|s| {
                (
                    s.clone(),
                    units
                        .get(s)
                        .unwrap()
                        .iter()
                        .flatten()
                        .cloned()
                        .filter(|s1| s1 != s)
                        .collect(),
                )
            })
            .collect();

   //     println!("sq: {:?}", squares);
        Config {
            rows: rows.to_string(),
            cols: cols.to_string(),
            digits: digits.to_string(),
            squares,
            unitlist,
            units,
            peers,
        }
    }
}

fn read_grid<I>(mut iter: I) -> Vec<char>
where
    I: Iterator<Item = char>,
{
    let mut v = vec![];
    while let Some(c) = iter.next() {
        if v.len() == 81 {
            panic!("invalid grid")
        }
        if c.is_digit(10) {
            v.push(c)
        }
        if c == '.' {
            v.push('0')
        }
    }
    if v.len() != 81 {
        panic!("invalid grid")
    }
    v
}

fn parse_grid(
    config: &Config,
    grid: HashMap<String, char>,
) -> Option<HashMap<String, String>> {
    let mut values: HashMap<String, String> = config
        .squares
        .iter()
        .map(|s| (s.clone(), config.digits.to_owned()))
        .collect();
    for (s, c) in grid {
    //    println!("parse {} at {}", c, s);
        if c != '0' {
            if !assign(config, &mut values, &s, c) {
                return None;
            }
        }
    }
    return Some(values);
}

fn assign(
    config: &Config,
    values: &mut HashMap<String, String>,
    square: &String,
    value: char,
) -> bool {
    // JAD addition below
    if let Some(v) = values.get(square) {
        if v.len() == 1 && v.chars().nth(0).unwrap() == value {
        //    println!("already assigned {} at {}", value, square);
            return true;
        }
    }
   //  println!("assigning {} at {}", value, square);
   // display(config, values);
    let other_values = values.get(square).unwrap().replace(value, "");
    other_values
        .chars()
        .map(|c| eliminate(config, values, square, c))
        .all(|b| b)
}

fn eliminate(
    config: &Config,
    values: &mut HashMap<String, String>,
    square: &String,
    value: char,
) -> bool {
   //  println!("eliminate {} at {}", value, square);
//    println!("values was {:?}", values);
    let current_values = values.get_mut(square).unwrap();
    if !current_values.contains(value) {
       // println!("already removed");
        return true;
    }
    *current_values = current_values.replace(value, "");
 //   println!("values now: {:?}", current_values);
    if current_values.len() == 0 {
       //  println!("contradiction: removed all possible values");
        return false;
    } else if current_values.len() == 1 {
        let d2 = current_values.chars().nth(0).unwrap();
       //  println!("single value remaining, eliminate {} from peers", d2);
        for p in config.peers.get(square).unwrap().iter() {
            if !eliminate(config, values, p, d2) {
              //  println!("failed to eliminate {} from peer {}", d2, p);
                return false;
            }
        }
        /*
        if !config
            .peers
            .get(square)
            .unwrap()
            .iter()
            .map(|p| eliminate(config, values, p, d2))    // all() short circuits, so we need the map() here 
            .all(|b| b) { return false; }
        */
    }
    for u in config.units.get(square).unwrap() {
        let dplaces: Vec<String> = u.iter().filter(|&s| values.get(s).unwrap().contains(value)).cloned().collect();
        if dplaces.len() == 0 {
         //   println!("contradiction: no place for {} in unit {:?}", value, u);
            return false;
        } else if dplaces.len() == 1 {
        //    println!("set peer {} to {}", dplaces[0], value);
            if !assign(config, values, &dplaces[0], value) {
            //    println!("can't assign {} at {}", value, dplaces[0]);
                return false;
            }
        }
    }
    return true;
}

fn search(config: &Config, maybe_values: Option<&mut HashMap<String, String>>) -> Option<HashMap<String, String>> {
    if maybe_values.is_none() { return None }
    let values = maybe_values.unwrap();
    if config.squares.iter().map(|s| values.get(s).unwrap().len() == 1).all(|b| b) {
        return Some(values.clone());
    }
    let s = config.squares.iter().filter(|&s| values.get(s).unwrap().len() > 1).min_by(|x, y| x.len().cmp(&y.len())).unwrap();
    for d in values.get(s).unwrap().chars() {
        let mut new_values = values.clone();
      //  println!("searching ... try {} at {}", d, s);
        if assign(config, &mut new_values, s, d) {
            if let Some(v) = search(config, Some(&mut new_values)) {
                return Some(v);
            }
        }
    }
    return None;
}


fn lpad(width: usize, word_len: usize) -> String {
    let mut pad = (width - word_len) / 2;
    if (width - word_len) % 2 == 1 {
        pad += 1;
    }
    iter::repeat(" ").take(pad).collect()
}

fn rpad(width: usize, word_len: usize) -> String {
    let pad = (width - word_len) / 2;
    iter::repeat(" ").take(pad).collect()
}

fn display(config: &Config, values: &HashMap<String, String>) {
    let width = 1 + config
        .squares
        .iter()
        .map(|s| values.get(s).unwrap().len())
        .max()
        .unwrap();
    let fmtstr = format!("{{:^{}}}", width);
    let line = iter::repeat("-".repeat(width * 3)).take(3).join("");
    for r in config.rows.chars() {
        println!("{}", 
            config
            .cols
            .chars()
            .map(|c| (c, format!("{}{}", r, c)))
            .map(|(c, rc)| (c, values.get(&rc).unwrap()))
            .map(|(c, this_values)| format!(
                "{}{}{}{}",
                lpad(width, this_values.len()),
                this_values,
                rpad(width, this_values.len()),
                if "36".contains(c) { "|" } else { "" }
            )).join(""));

        if "CF".contains(r) {
            println!("{}", line);
        }
    }
}

fn solve(config: &Config, grid: HashMap<String, char>) -> Option<HashMap<String, String>> {
    if let Some(mut values) = parse_grid(config, grid) {
        return search(config, Some(&mut values));
    } else {
        return None;
    }
}

fn main() {
    let config = Config::new();

    /*
    let grid_str = "400000805
            030000000
            000700000
            020000060
            000080400
            000010000
            000603070
            500200000
            104000000";
    */
    // let grid_str = "003020600900305001001806400008102900700000008006708200002609500800203009005010300";

    // let grid_str = "4.....8.5.3..........7......2.....6.....8.4......1.......6.3.7.5..2.....1.4......";

    let file = File::open("p096_sudoku.txt").unwrap();
    let lines: Vec<String> = BufReader::new(file).lines().map(|l| l.unwrap()).collect();

    let answer:i32 = 
        lines.iter().batching(|it| {
        match it.next() {
            None => None,
            Some(name) => {
                println!("reading puzzle {}", name);
                Some(it.take(9).join(""))
            }
        }
    }).map(|g| {
        let grid:HashMap<String, char> = config.squares.iter().cloned().zip(read_grid(g.chars())).collect();
        if let Some(solved) = solve(&config, grid) {
            let code = vec![solved.get("A1").unwrap(), solved.get("A2").unwrap(), solved.get("A3").unwrap()].into_iter().join("");
           // display(&config, &solved);
           // println!("code: {}", code);
            code.parse::<i32>().unwrap()
        } else {
            panic!("no solution");
        }
    }).sum();
    println!("Answer: {}", answer);
/*
    let grid: HashMap<String, char> = config
        .squares
        .iter()
        .cloned()
        .zip(read_grid(
            BufReader::new(grid_str.as_bytes())
                .chars()
                .map(|c| c.unwrap()),
        ))
        .collect();
    if let Some(solved) = solve(&config, grid) {
        display(&config, &solved);
    } else {
        println!("no solution");
    }
*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = Config::new();
        println!("{:?}", config.unitlist);
        assert_eq!(81, config.squares.len());
        assert_eq!(27, config.unitlist.len());
        assert!(config.squares.iter().all(|s| config.units.get(s).unwrap().len() == 3));
        assert!(config.squares.iter().all(|s| config.peers.get(s).unwrap().len() == 20));
        println!("peers[C2] = {:?}", config.peers.get("C2").unwrap());
        println!("units: {:?}", config.units.get("C2").unwrap());
    }
}


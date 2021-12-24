#![feature(int_log)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};


fn maybe_assign_square(squares: &Vec<u64>, w1: &Vec<char>, w2: &Vec<char>) -> Option<(u64, u64)> {
    assert!(w1.len() == w2.len());

    None
}

#[derive(Debug)]
struct GramCode<'a, T> {
    val: &'a T,
    as_string: String,
    code: String
}

impl<'a, T> Hash for GramCode<'a, T> {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.code.hash(state);
    }
}

impl<'a, T> PartialEq for GramCode<'a, T> {

    fn eq(&self, other: &Self) -> bool {
        self.code.eq(&other.code)
    }
}

impl<'a, T> Eq for GramCode<'a, T> {}

/*
 *
 */
fn get_gram_code<T>(w: &T) -> GramCode<T>
where
    T: ToString,
{
    let s = w.to_string();
    assert!(s.len() > 0);
    let codes = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm'];
    let mut code = String::new();
    let mut last_idx = 0;
    let mut last_char: Option<char> = None;
    s.chars().sorted().for_each(|c| {
        match last_char {
            None => {}
            Some(x) if x != c => {
                last_idx += 1;
            }
            Some(_) => {}
        }
        last_char = Some(c);
        code.push(codes[last_idx]);
    });
    GramCode { val: w, as_string: s, code: code } 
}
/**
 */
fn get_anagrams<T>(l: &[T]) -> HashMap<String, Vec<Vec<GramCode<T>>>>
where
    T: ToString + std::fmt::Debug,
{
    let mut anagram_map = HashMap::new();

    let code_map: HashMap<String, Vec<GramCode<T>>> = l
        .iter()
        .map(|w| {
            let c = get_gram_code(w);
            (c.code.clone(), c)
        })
        .into_group_map();
    for (code, grams) in code_map.into_iter() {
        let mut vals = vec![];
        let m: HashMap<String, Vec<GramCode<T>>> = grams.into_iter().map(|g| {
            (g.as_string.chars().sorted().collect(), g)
        }).into_group_map();
        for (k, v) in m.into_iter() {
            if v.len() > 1 {
                vals.push(v);
            }
        }
        if vals.len() > 0 {
            anagram_map.insert(code, vals);
        }
    }
    anagram_map
}

fn get_xform(s1: &String, s2: &String) -> Vec<usize> {
    let mut v = vec![];
    let mut used = HashSet::new();
    's1: for (i, c1) in s1.chars().enumerate() {
        's2: for (j, c2) in s2.chars().enumerate() {
            if c1 == c2 {
                if used.contains(&j) {
                    continue 's2;
                } else {
                    v.push(j);
                    used.insert(j);
                    continue 's1;
                }
            }
        }
        panic!("can't get xform for {} and {}", s1, s2);
    }
    v
}

fn main() -> Result<(), std::io::Error> {
    let words: Vec<String> = fs::read_to_string("p098_words.txt")?
        .split(',')
        .map(|s| s.trim_matches('"').to_string())
        .collect();

    println!("{} words", words.len());

    let mut map: HashMap<usize, Vec<&str>> = HashMap::new();

    words.iter().for_each(|w| {
        let vec = map.entry(w.len()).or_insert(vec![]);
        vec.push(w);
    });

    map.iter()
        .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        .for_each(|p| {
            println!("len {:2} has {} words", p.0, p.1.len());
        });

    for k in (0..=*map.keys().max().unwrap()).rev() {
        if let Some(ws) = map.get(&k) {
            let anagrams = get_anagrams(ws);
            if anagrams.len() == 0 {
                continue;
            }
            let squares:Vec<u64> = ((10f64.powf((k as f64 - 1f64)/2f64) as u64)..=(10f64.powf(k as f64/2f64) as u64)).map(|n| n*n).collect();
            let sq_grams = get_anagrams(&squares[..]);
            let mut max = 0u64;
            for (code, anagram_sets) in anagrams.into_iter() {
                if let Some(sq_gram_sets) = sq_grams.get(&code) {
                    for anagram_set in anagram_sets.into_iter() {
                        for i in 0..anagram_set.len()-1 {
                           for j in i+1..anagram_set.len() {
                                let xform = get_xform(&anagram_set[i].as_string, &anagram_set[j].as_string);
                                for sq_gram_set in sq_gram_sets {
                                    for si in 0..sq_gram_set.len()-1 {
                                        for sj in si+1..sq_gram_set.len() {
                                            if xform == get_xform(&sq_gram_set[si].as_string, &sq_gram_set[sj].as_string) {
                                                println!("match: [{}, {}] -> [{}, {}]", anagram_set[i].as_string, anagram_set[j].as_string, sq_gram_set[si].as_string, sq_gram_set[sj].as_string);
                                            }
                                        }
                                    }
                                }
                           }
                        }
                    }
                }
            }
/*
            for sq_gram in sq_grams {
                println!("sq_gram: {:?}", sq_gram);
                for s in sq_gram {
                    for pair in &pairs {
                        for a in pair {
                            println!("word: {} ({}), sq: {} ({})", a.val, a.code, s.val, s.code);
                            if s.code == a.code && *s.val > max {
                                println!("max!");
                                max = *s.val;
                            }
                        }
                    }
                }
            }
*/
            println!("max: {}", max);
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_codes() {
        assert_eq!("abc", get_gram_code(&"abc".to_string()).code);
        assert_eq!("abc", get_gram_code(&"123".to_string()).code);
        assert_eq!("abb", get_gram_code(&"Foo".to_string()).code);
        assert_eq!("aabcdeee", get_gram_code(&"11356999".to_string()).code);
    }

    #[test]
    fn test_xform() {
        assert_eq!(get_xform(&"1234".to_string(), &"1423".to_string()), vec![0, 2, 3, 1]);
        assert_eq!(get_xform(&"INTRODUCE".to_string(), &"REDUCTION".to_string()), vec![6, 8, 5, 0, 7, 2, 3, 4, 1]);
    }

}

use std::collections::HashMap;
use std::collections::HashSet;

use euler_rust::*;


/*
fn divisors(n:u64, cache:&mut HashMap<u64, Vec<u64>>) -> Vec<u64> {
    if cache.contains_key(&n) {
        return cache.get(&n).unwrap().to_vec();
    }
        
    let mut d = euler_rust::divisors(n).collect::<Vec<_>>();

    /*
    while d*2 <= n {
        if n % d == 0 {
            divisors.push(d);
        }
        d +=1;
    }
     */
    cache.insert(n, d.clone());
//    println!("divisors of {}: {:?}", n, d);
    d
}
 */

fn factor(n: u64, primes: &[u64]) -> HashMap<u64, u8> {
    let mut factors = HashMap::new();
    let mut tmp = n;
//    println!("factoring {}", n);
    for factor in primes {
////        println!("testing {}", factor);
        if factor * 2 > n {
//            println!("pau");
            break;
        }
        while tmp % factor == 0 {
//            println!("{} is a factor", factor);
            let entry = factors.entry(*factor).or_insert(0u8);
            *entry += 1;
            tmp /= factor;
        }
    }
    factors
}

fn divisor_sum(n: u64, primes: &[u64]) -> u64 {
    //divisors(n, cache).iter().fold(0, |acc, &x| acc + x)
    let factors = factor(n, primes);
    let mut result = 1;
//    println!("divisor_sum: {}", n);
    for (p, e) in factors.iter() {
//        println!("p: {}\te:{}", p, e);
        let step = ((p.pow((e+1).into()) - 1) / (p - 1));
//        println!("step: {}", step);
        result = result * step;
//        println!("result: {}", result);
    }
    result - n
}

fn vec_contains(v:&Vec<u64>, n:u64) -> bool {
    for &i in v {
        if i == n {
            return true;
        }
    }
    return false;
}

fn trim_chain(v:&Vec<u64>, n:u64) -> Vec<u64> {
    let start = v.iter().position(|&i| i == n).unwrap();
    v[start..v.len()].to_vec()
}

fn is_new_chain(chains:&Vec<Vec<u64>>, this_chain:&Vec<u64>) -> bool {
    for i in 0..chains.len() {
        if vec_contains(&chains[i], this_chain[0]) {
            return false;
        }
    }
    return true;
}


fn main() {
    let mut n = 2u64;
    let MAX:u64 = 100000;
    let mut seen = HashSet::new();
    let mut chains = Vec::new();
//    let mut divisor_cache = HashMap::new();
    let primes = euler_rust::primes().take_while(|x| *x <= MAX/2).collect::<Vec<u64>>();
    
    while n <= MAX  {
//        println!("test {}", n);
        if seen.contains(&n) {
//            println!("skipping {}, seen size is {}", n, seen.len());
            n += 1;
            continue;
        }
        let mut this_chain = vec![n];
        let mut next = n;
        loop {
            next = divisor_sum(next, &primes);
//            println!("next: {}", next);
//            println!("this_chain: {:?}", this_chain);
            if next == 1 {
                // prime
                break;
            }
            if next == this_chain[this_chain.len()-1] {
                // perfect number
                for i in &this_chain {
                    seen.insert(i.clone());
                }
                break;
            }
            else if next > MAX   {
                // not part of a chain
                for i in &this_chain {
                    // clone the u64, because otherwise the borrow of this_chain lasts too long
                    seen.insert(i.clone());
                }
                break;
            } else if vec_contains(&this_chain, next) {
                this_chain.push(next);
                for i in &this_chain {
                    seen.insert(i.clone());
                }
                // have we see this chain before?
                let trimmed_chain = trim_chain(&this_chain, next);
                if is_new_chain(&chains, &trimmed_chain) {
                    chains.push(trimmed_chain);
                }
                break;
            } else {
                this_chain.push(next);
            }
        }
        n += 1;
    }
    for v in chains {
        println!("{:?}", v);
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    /*
    #[test]
    fn test_div1() {
        assert_eq!(divisors(7), vec![1]);
        assert_eq!(divisors(12), vec![1, 2, 3, 4, 6]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14]);
        assert_eq!(divisors(25), vec![1, 5]);
    }
     */

    /*
    #[test]
    fn test_perfect1() {
        assert!(is_perfect(28));
        assert!(!is_perfect(12));
    }
     */
    
    #[test]
    fn test_150() {
        //        println!("150: {:?}", divisors(150));
        println!("72 sum: {:?}", divisor_sum(72));
        println!("28 sum: {:?}", divisor_sum(28));
        println!("220 sum: {:?}", divisor_sum(220));
        println!("284 sum: {:?}", divisor_sum(284));
        println!("150 sum: {:?}", divisor_sum(150));
        println!("222 sum: {:?}", divisor_sum(222));
        println!("222 sum: {:?}", divisor_sum(234));
        println!("312 sum: {:?}", divisor_sum(312));
    }

}
        

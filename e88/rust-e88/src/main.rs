use std::collections::HashSet;
use std::{collections::HashMap, convert::TryInto};

fn first_factor(n: u32) -> u32 {
    if n % 2 == 0 {
        return 2;
    }
    for f in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= n) {
        if n % f == 0 {
            return f;
        }
    }
    return n;
}

/**
 * return a HashMap factorization of n, with prime factors as keys
 */
fn factor(n: u32) -> HashMap<u32, u8> {
    let mut factors = HashMap::new();
    let mut cur = n;
    loop {
        let f = first_factor(cur);
        if f == n {
            let this_factor = factors.entry(f).or_insert(0);
            *this_factor = 1;
            break;
        }
        let this_factor = factors.entry(f).or_insert(0);
        *this_factor += 1;
        if cur == f {
            break;
        }
        cur /= f;
    }
    factors
}

fn product_equals_p(a: &Vec<i32>, n: i32) -> bool {
    unimplemented!();
}

fn is_product_sum_old(n: i32, k: usize) -> Option<Vec<i32>> {
    // Knuth "Algorithm H" from Fasicle 3b
    let mut a = vec![1i32; k + 1];
    a[0] = n as i32 - k as i32 + 1;
    a[k] = -1;
    let mut tries = 1u32;

    loop {
        println!("try {}", tries);
        tries = tries + 1;
        if product_equals_p(&a, n) {
            return Some(a);
        }
        if a[1] < a[0] - 1 {
            a[0] = a[0] - 1;
            a[1] = a[1] + 1;
            continue;
        }
        let mut j = 2;
        let mut s = a[0] + a[1] - 1;
        loop {
            if a[j] < a[0] - 1 {
                break;
            }
            s = s + a[j];
            j = j + 1;
        }
        if j + 1 > k {
            return None;
        }
        let x = a[j] + 1;
        a[j] = x;
        j = j - 1;
        loop {
            if j == 0 {
                break;
            }
            a[j] = x;
            s = s - x;
            j = j - 1;
        }
        a[0] = s;
    }
}

/**
* Knuth's Algorithm P from Fascile 3b
*/
fn make_all_partitions(n: u8) -> Vec<Vec<u8>> {
    let mut n_mut = n;
    let mut v = Vec::new();
    let mut a = vec![0u8];
    let mut m = 1;
    'p2: loop {
        if m == a.len() {
            a.push(0);
        }
        a[m] = n_mut;
        let mut q = m;
        if n_mut == 1 {
            q -= 1;
        }
        'p3: loop {
            v.push(a[1..(m + 1)].to_vec());
            if a[q] == 2 {
                a[q] = 1;
                q -= 1;
                m += 1;
                if m == a.len() {
                    a.push(0);
                }
                a[m] = 1;
                continue 'p3;
            }
            if q == 0 {
                break 'p2;
            }
            let mut x = a[q] - 1;
            a[q] = x;
            n_mut = (m - q + 1) as u8;
            m = q + 1;
            'p6: loop {
                if n_mut <= x {
                    continue 'p2;
                } else {
                    a[m] = x;
                    m += 1;
                    n_mut -= x;
                }
            }
        }
    }
    v
}

fn _analyze_factors() {
    let mut factors = HashMap::new();
    for n in 2..12000 {
        factors.insert(n, factor(n));
    }
    let mut max_primes = 0;
    let mut max_n = 0;
    let mut max_factors = 0;
    
    for (n, n_factors) in factors.iter() {
        if n_factors.len() > max_primes {
            max_primes = n_factors.len();
        }
        // if n_factors.len() == 5 {
        //     println!("{} has 5 factors", n);
        // }
        let mut this_factors = 0;
        for (p, p_n) in n_factors.iter() {
            if *p_n > max_n {
                max_n = *p_n;
            }
            // if *p_n == 14 {
            //     println!("{} has a p_n == 14 ({})", n, p);
            // }
            this_factors += p_n;
        }
        println!("{} has {} total factors", n, this_factors);
        if this_factors > max_factors {
            max_factors = this_factors;
        }

    }
    println!("max primes: {}", max_primes);
    println!("max p_n: {}", max_n);
    println!("max total factors: {}", max_factors);
}

struct PartitionEnumerate<'a> {
    i: usize,
    _p: &'a Vec<Vec<u8>>,
}

impl<'a> PartitionEnumerate<'a> {
    fn new(p: &'a Vec<Vec<u8>>) -> Self {
        PartitionEnumerate { i: 0, _p: p }
    }
}

impl<'a> Iterator for PartitionEnumerate<'a> {
    type Item = &'a Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self._p.len() {
            let v = Some(&self._p[self.i]);
            self.i += 1;
            return v;
        } else {
            return None;
        }
    }
}

#[derive(Debug)]
struct FactorConfiguration<'a> {
    p: u32,
    partition: &'a Vec<u8>,
}

impl<'a> FactorConfiguration<'a> {
    fn sum(&self) -> u32 {
        let mut sum = 0u32;
        for power in self.partition {
            sum += self.p.pow(*power as u32);
        }
        sum
    }

    fn num_factors(&self) -> u32 {
        self.partition.len().try_into().unwrap()
    }
}

#[derive(Debug)]
struct Configuration<'a> {
    config: Vec<FactorConfiguration<'a>>,
}

impl<'a> Configuration<'a> {
    fn sum(&self, k: u32) -> u32 {
        let mut sum = 0;
        let mut num_factors = 0;

        for fc in &self.config {
            sum += fc.sum();
            num_factors += fc.num_factors();
        }

        if num_factors < k {
            sum += k - num_factors;
        }
        sum
    }
}

#[derive(Debug)]
struct ConfigurationEnumerate<'a> {
    configs: Vec<(u32, &'a Vec<Vec<u8>>)>,
    indexes: Vec<usize>,
}

impl<'a> ConfigurationEnumerate<'a> {
    fn new(factors: &'a HashMap<u32, u8>, partitions: &'a HashMap<u8, Vec<Vec<u8>>>) -> Self {
        let mut configs = Vec::new();
        let mut indexes = Vec::new();
        indexes.resize(factors.len(), 0 as usize);

        for (factor, power) in factors {
            //            println!("factor:{}, power:{}", factor, power);
            configs.push((*factor, partitions.get(power).unwrap()));
        }
        ConfigurationEnumerate {
            configs: configs,
            indexes: indexes,
        }
    }
}

impl<'a> Iterator for ConfigurationEnumerate<'a> {
    type Item = Configuration<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.indexes[0] == self.configs[0].1.len() {
            return None;
        }
        assert_eq!(self.indexes.len(), self.configs.len());

        let mut c = Vec::new();
        for j in 0..self.indexes.len() {
            c.push(FactorConfiguration {
                p: self.configs[j].0,
                partition: &self.configs[j].1[self.indexes[j]],
            });
        }

        for i in (0..self.indexes.len()).rev() {
            if self.indexes[i] < self.configs[i].1.len() - 1 {
                self.indexes[i] += 1;
                for j in i + 1..self.indexes.len() {
                    self.indexes[j] = 0;
                }
                break;
            } else if i == 0 {
                self.indexes[i] += 1;
            }
        }
        return Some(Configuration { config: c });
    }
}

fn make_configuration_iterator<'a>(
    factors: &'a HashMap<u32, u8>,
    all_partitions: &'a HashMap<u8, Vec<Vec<u8>>>,
) -> ConfigurationEnumerate<'a> {
    ConfigurationEnumerate::new(factors, all_partitions)
}

fn is_product_sum2(
    factors: &HashMap<u32, HashMap<u32, u8>>,
    partitions: &HashMap<u8, Vec<Vec<u8>>>,
    n: u32,
    k: u32,
) -> bool {
    let n_factors = factors.get(&n).unwrap();

    for config in make_configuration_iterator(n_factors, partitions) {
        println!("testing {:?}", config);
        if config.sum(k) == n {
            return true;
        }
    }
    false
}

fn main() {
    _analyze_factors();

    /*
    let mut partitions = HashMap::new();
    for n in 1..15 {
        let p = make_all_partitions(n);
        println!("{} has {} partitions", n, p.len());
        partitions.insert(n, p);
    }

    let mut factors = HashMap::new();
    for n in 2..25000 {
        factors.insert(n, factor(n));
    }

    let mut min_product_sums = HashSet::new();

    'k: for k in 2..7 {
        if k % 1000 == 0 {
            println!("k:{}", k);
        }
        for n in k.. {
            println!("testing n:{}, k:{}", n, k);
            if is_product_sum2(&factors, &partitions, n, k) {
                min_product_sums.insert(n);
                continue 'k;
            }
        }
    }
    println!("mins: {:?}", min_product_sums);
    //    println!("mins: {:?}", mins);
    println!("sum: {}", min_product_sums.iter().fold(0, |acc, x| acc + x));
    */
}

mod test {

    use super::*;

    #[test]
    fn test_factor1() {
        let mut l = factor(12);
        assert_eq!(l.len(), 2);
        assert_eq!(*l.get(&2).unwrap(), 2);
        assert_eq!(*l.get(&3).unwrap(), 1);

        l = factor(25);
        assert_eq!(l.len(), 1);
        assert_eq!(*l.get(&5).unwrap(), 2);

        l = factor(13);
        assert_eq!(l.len(), 1);
        assert_eq!(*l.get(&13).unwrap(), 1);
    }

    #[test]
    fn test_config_iter1() {
        let mut partitions = HashMap::new();
        for n in 1..15 {
            let p = make_all_partitions(n);
            partitions.insert(n, p);
        }

        let mut factors = HashMap::new();
        for n in 2..25000 {
            factors.insert(n, factor(n));
        }
        let n = 24;
        let mut it = make_configuration_iterator(factors.get(&n).unwrap(), &partitions);
        for i in it {
            println!("config: {:?}", i);
        }
    }

    #[test]
    fn test_isPS1() {
        let mut partitions = HashMap::new();
        for n in 1..15 {
            let p = make_all_partitions(n);
            partitions.insert(n, p);
        }

        let mut factors = HashMap::new();
        for n in 2..25000 {
            factors.insert(n, factor(n));
        }

        assert!(is_product_sum2(&factors, &partitions, 12, 6));
        assert!(!is_product_sum2(&factors, &partitions, 7, 3));
    }
}

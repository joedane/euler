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

struct RGS {
    a: Vec<u8>,
    b: Vec<u8>,
    m: u8,
    started: bool,
}

impl RGS {
    fn new(size: usize) -> Self {
        RGS {
            a: vec![0u8; size],
            b: vec![1u8; size - 1],
            m: 1u8,
            started: false,
        }
    }

    /*
     *  Why not make this a standard Iterator?  Because we want to return references to
     *  a value that we're modifying at each iteration, which the Iterator trait
     *  does not allow.  See
     * https://stackoverflow.com/questions/25702909/can-i-write-an-iterator-that-mutates-itself-and-then-yields-a-reference-into-its
     */

    fn next(&mut self) -> Option<&[u8]> {
        if !self.started {
            self.started = true;
            return Some(&self.a);
        }

        let n = self.a.len();

        if self.a[n - 1] != self.m {
            self.a[n - 1] += 1;
            return Some(&self.a);
        }
        let mut j = n - 1;
        while self.a[j - 1] == self.b[j - 1] {
            j -= 1;
        }
        if j == 1 {
            return None;
        }
        self.a[j - 1] += 1;
        self.m = self.b[j - 1] + if self.a[j - 1] == self.b[j - 1] { 1 } else { 0 };
        j += 1;
        while j < n {
            self.a[j - 1] = 0;
            self.b[j - 1] = self.m;
            j += 1;
        }
        self.a[n - 1] = 0;
        return Some(&self.a);
    }
}

fn factors_to_list(factors: HashMap<u32, u8>) -> Vec<u32> {
    let mut f = vec![];
    for (&p, &p_n) in factors.iter() {
        for n in 0..p_n {
            f.push(p);
        }
    }
    return f;
}

fn partition_count(p: &Vec<u8>) -> usize {
    let mut c = 0;
    for &v in p {
        if v > c {
            c = v;
        }
    }
    return (c + 1).into();
}

fn partition_sum(factors: &Vec<u32>, partition: &Vec<u8>, count: usize) -> u32 {
    let mut terms = vec![1u32; count];
    // println!(
    //     "count: {}, factors: {:?}, partition: {:?}",
    //     count, factors, partition
    // );
    for p_i in 0..partition.len() {
        terms[partition[p_i] as usize] *= factors[p_i];
    }
    //    println!("final terms: {:?}", terms);
    return terms.iter().fold(0, |acc, &x| acc + x);
}


fn main() {
    let mut partition_map = HashMap::new();
    let mut min_product_sums = HashSet::new();

    println!("pre-computing partitions");
    for n in 2..14 {
        let mut partitions = vec![];
        let mut rgs = RGS::new(n);
        while let Some(a) = rgs.next() {
            partitions.push(a.to_vec());
        }
        partition_map.insert(n, partitions);
    }

    println!("pre-computing factorizations");
    let mut factors = HashMap::new();
    for n in 2..25000 {
        factors.insert(n, factors_to_list(factor(n)));
    }

    'k: for k in 2..12001 {
        //println!("k: {}", k);
        'n: for n in k.. {
            //            println!("n: {}", n);
            let n_factors = factors.get(&n).unwrap();
            if n_factors.len() == 1 {
                continue 'n;
            }
            //            println!("n_factors: {}", n_factors.len());
            let partitions = partition_map.get(&n_factors.len()).unwrap();
            for p in partitions {
                let c = partition_count(p);
                if c <= k.try_into().unwrap() {
                    // println!(
                    //     "n: {}, k: {}, c: {}, sum: {}",
                    //     n,
                    //     k,
                    //     c,
                    //     partition_sum(n_factors, p, c)
                    // );
                    if partition_sum(n_factors, p, c) + (k - c as u32) == n {
                        println!("{} is a product-sum number for k = {}", n, k);
                        min_product_sums.insert(n);
                        continue 'k;
                    }
                }
            }
        }
    }
    println!("sum: {}", min_product_sums.iter().fold(0, |acc, x| acc + x));

}


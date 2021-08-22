#[allow(non_snake_case)]
use std::ops::{Mul, Div, Sub, Neg, Rem, AddAssign};
use std::collections::HashMap;
use std::hash::Hash;
use std::convert::TryInto;

use num_bigint::BigInt;
use num_integer::{Roots};
use num_traits::identities::{One, Zero};

fn is_prime<A>(n: &A) -> bool
where
    for<'a> &'a A: Mul<&'a A, Output = A>,
for<'a> &'a A: Sub<&'a A, Output = A>,
for<'a> &'a A: Rem<&'a A, Output = A>,
for<'a> A: AddAssign<&'a A>,
    A: One,
    A: Zero,
    A: Roots,
    A: From<u8>,
    A: std::cmp::PartialEq,

{
    if *n == A::one() || *n == A::zero() {
        return false;
    }
    if *n == A::from(2) || *n == A::from(3) {
        return true;
    }
    if n.is_multiple_of(&A::from(2)) ||
        n.is_multiple_of(&A::from(3)) {
            return false;
        }
    
    let stop = n.sqrt();
    let mut div = A::from(5);
    let mut increment = A::from(2);
    let six = A::from(6);
    while div <= stop {
        if n.rem(&div) == A::zero() {
            return false;
        }
        div += &increment;
        increment = &six - &increment;
    }
    true
        
}


fn _is_prime_bigint(n: &BigInt) -> bool {
    if *n == BigInt::zero() {
        return false;
    }
    if *n == BigInt::one() {
        return false;
    }
    if *n < BigInt::zero() {
        return is_prime(&n.neg());
    }
    if *n == BigInt::from(2) {
        return true;
    }
    if *n == BigInt::from(3) {
        return true;
    }

    if n.rem(BigInt::from(2)) == BigInt::zero() {
        return false;
    }

    if n.rem(BigInt::from(3)) == BigInt::zero() {
        return false;
    }

    let stop = n.sqrt();
    let mut div = BigInt::from(5);
    let mut increment = BigInt::from(2);
    let six = BigInt::from(6);

    while div <= stop {
        if n.rem(&div) == BigInt::zero() {
            return false;
        }
        div += &increment;
        increment = &six - &increment;
    }
    true
}

pub struct PrimesIterator<A> {
    last: A,
}

impl<A> Iterator for PrimesIterator<A> where
    for<'a> &'a A: Mul<&'a A, Output = A>,
for<'a> &'a A: Sub<&'a A, Output = A>,
for<'a> &'a A: Rem<&'a A, Output = A>,
for<'a> A: AddAssign<&'a A>,
    A: One,
    A: Zero,
    A: Roots,
    A: From<u8>,
    A: std::cmp::PartialEq,
    A: Clone
{
    type Item = A;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.last += &A::one();
            if is_prime(&self.last) {
                return Some(self.last.clone());
            }
        }
    }
}

pub fn primes<T>() -> PrimesIterator<T>
where T: From<u8> {
    PrimesIterator { last: T::from(1u8) }
}

pub struct DivisorIterator {
    powers: Vec<u64>,
    factors: Vec<u64>,
    multiplicities: Vec<u8>
}

impl DivisorIterator {

    fn new(p: Vec<u64>, m: Vec<u8>) -> Self {
        let mut this_powers = vec![0; p.len()];
        if p.len() > 0 {
            this_powers[0] = 1;
        }
        DivisorIterator {
            powers: this_powers,
            factors:p,
            multiplicities: m
        }
    }
}


impl Iterator for DivisorIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
//        println!("primes: {:?}\t\tmults: {:?}\t\tdigits: {:?}", self.factors, self.multiplicities, self.powers);
        let mut this_digit : Option<usize> = None;
        for i in 0..self.powers.len() {
            if self.powers[i] < self.multiplicities[i].into() {
                this_digit = Some(i);
                break;
            }
        }
        match this_digit {
            None => None,
            Some(d) => {
                let divisor = self.factors.iter().
                    zip(&self.powers).
                    map(|(f, &p)| f.pow(p.try_into().unwrap())).
                    fold(1, |acc, x| acc * x);
                self.powers[d] += 1;
                for i in 0..d {
                    self.powers[i] = 0;
                }
                Some(divisor)
            }
        }
    }
}

pub fn divisors(n: u64) -> DivisorIterator {
    let factors = factor(n);
    let mut primes = Vec::new();
    let mut mults = Vec::new();
    for (p, m) in factors.iter() {
        primes.push(*p);
        mults.push(*m);
    }
    DivisorIterator::new(primes, mults)
}

pub fn factor(n: u64) -> HashMap<u64, u8> {
    let mut factors = HashMap::new();
    let mut tmp = n;
//    println!("factoring {}", n);
    for factor in primes::<u64>() {
////        println!("testing {}", factor);
        if factor * 2 > n {
//            println!("pau");
            break;
        }
        while tmp % factor == 0 {
//            println!("{} is a factor", factor);
            let entry = factors.entry(factor).or_insert(0);
            *entry += 1;
            tmp /= factor;
        }
    }
    factors
}

/*
fn factor<T>(n: T) -> HashMap<T, u8>
where T: Div, T:Rem, T:Mul, T:Eq, T:Hash, T: From<u8>,
<T as Mul>::Output: PartialOrd<T>,
<T as Rem>::Output: PartialOrd<T>,
      T:std::ops::DivAssign, T:std::ops::AddAssign {
    let mut factors = HashMap::new();
    for factor in primes::<T> {
        if factor * factor > n {
            break;
        }
        if n % factor == T::from(0) {
            n /= factor;
            let entry = factors.entry(&factor).or_insert(T::from(0u8));
            *entry += T::from(1);
        }
    }
    factors
}
 */

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_primes() {
        assert!(is_prime(&BigInt::from(2)));
        assert!(is_prime(&BigInt::from(5)));
        assert!(is_prime(&BigInt::from(13)));
        assert!(!is_prime(&BigInt::from(4)));
        assert!(!is_prime(&BigInt::from(25)));
        assert!(is_prime(&BigInt::from(7789)));
        assert!(!is_prime(&BigInt::from(7791)));
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_primes2() {
        assert!(is_prime(&2));
        assert!(is_prime(&5));
        assert!(is_prime(&13));
        assert!(!is_prime(&4));
        assert!(!is_prime(&25));
        assert!(is_prime(&7789));
        assert!(!is_prime(&7791));
        assert!(!is_prime(&BigInt::from(7791)));
    }

    #[test]
    fn test_prime2_list() {
        let mut primes = PrimesIterator { last: 1u64 };
        let mut count = 0;

        while primes.next().unwrap() < 1000000 {
            count += 1;
        }
        println!("count: {}", count);
    }

    #[test]
    fn test_factor1() { 
        println!("factors of 100: {:?}", factor(100));
        println!("factors of 2: {:?}", factor(2));
        println!("factors of 13: {:?}", factor(13));
        println!("factors of 12345678: {:?}", factor(12345678));
        println!("factors of 66494: {:?}", factor(66494));

    }

    #[test]
    fn test_divisors1() {
        println!("Divisors of 24: {:?}", divisors(24).collect::<Vec<_>>());
        println!("Divisors of 66494: {:?}", divisors(66494).collect::<Vec<_>>());
    }
        
}

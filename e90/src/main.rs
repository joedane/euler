use bitintr::Popcnt;

const ZERO: u32 = 1;
const ONE: u32 = 1 << 1;
const TWO: u32 = 1 << 2;
const THREE: u32 = 1 << 3;
const FOUR: u32 = 1 << 4;
const FIVE: u32 = 1 << 5;
const SIX: u32 = 1 << 6;
const SEVEN: u32 = 1 << 7;
const EIGHT: u32 = 1 << 8;
const NINE: u32 = 1 << 9;

macro_rules! has {
    ($in:ident, $pos:ident, $val:ident) => {
        ($in[$pos] & $val) != 0
    };
}

#[inline(always)]
fn works(p: &Vec<u32>, d1: usize, d2: usize) -> bool {
    ((has!(p, d1, ZERO) && has!(p, d2, ONE)) || (has!(p, d2, ZERO) && has!(p, d1, ONE)))
        && ((has!(p, d1, ZERO) && has!(p, d2, FOUR)) || (has!(p, d2, ZERO) && has!(p, d1, FOUR)))
        && ((has!(p, d1, ZERO) && (has!(p, d2, NINE) || has!(p, d2, SIX)))
            || (has!(p, d2, ZERO) && (has!(p, d1, SIX) || has!(p, d1, NINE))))
        && ((has!(p, d1, ONE) && (has!(p, d2, SIX) || has!(p, d2, NINE)))
            || (has!(p, d2, ONE) && (has!(p, d1, SIX) || has!(p, d1, NINE))))
        && ((has!(p, d1, TWO) && has!(p, d2, FIVE)) || (has!(p, d2, TWO) && has!(p, d1, FIVE)))
        && ((has!(p, d1, THREE) && (has!(p, d2, SIX) || has!(p, d2, NINE)))
            || (has!(p, d2, THREE) && (has!(p, d1, SIX) || has!(p, d1, NINE))))
        && ((has!(p, d1, FOUR) && (has!(p, d2, NINE) || has!(p, d2, SIX)))
            || (has!(p, d2, FOUR) && (has!(p, d1, NINE) || has!(p, d1, SIX))))
        && (((has!(p, d1, SIX) || has!(p, d1, NINE)) && has!(p, d2, FOUR))
            || ((has!(p, d2, SIX) || has!(p, d2, NINE)) && has!(p, d1, FOUR)))
        && ((has!(p, d1, EIGHT) && has!(p, d2, ONE)) || (has!(p, d2, EIGHT) && has!(p, d1, ONE)))
}

fn setup_patterns() -> Vec<u32> {
    let mut patterns = Vec::new();

    for i in 0x3Fu32..(1u32 << 10) {
        if i.popcnt() == 6 {
            patterns.push(i);
        }
    }
    patterns
}

fn main() {
    let patterns = setup_patterns();

    let mut ok_patterns = 0u32;

    for d1 in 0..patterns.len() {
        for d2 in 0..patterns.len() {
            if works(&patterns, d1, d2) {
                ok_patterns += 1;
            }
        }
    }

    println!("{}", ok_patterns / 2); // divide by two because the answer assumes the dice aren't distinguishable
}

mod test {

    use super::*;

    #[test]
    fn test1() {
        let patterns = vec![
            ZERO | FIVE | SIX | SEVEN | EIGHT | NINE,
            ONE | TWO | THREE | FOUR | EIGHT | NINE,
        ];
        assert!(works(&patterns, 0, 1));
    }

    #[test]
    fn test2() {
        let patterns = vec![
            ZERO | FIVE | SIX | SEVEN | EIGHT | NINE,
            ONE | TWO | THREE | FOUR | SIX | SEVEN,
        ];
        assert!(works(&patterns, 0, 1));
    }

    #[test]
    fn test3() {
        let patterns = vec![
            ZERO | FIVE | SIX | SEVEN | EIGHT | NINE,
            TWO | THREE | FOUR | SIX | SEVEN,
            EIGHT,
        ];
        assert!(!works(&patterns, 0, 1));
    }
}

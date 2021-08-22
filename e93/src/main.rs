#![feature(try_blocks)]

use itertools::Itertools;
use num_rational::Ratio;
use num_traits::ops::checked::{CheckedAdd, CheckedDiv, CheckedMul, CheckedSub};
use std::collections::BTreeSet;
use std::error::Error;

macro_rules! mk_rat {
    ($v:expr) => {
        Ratio::new($v as i32, 1)
    };
}

macro_rules! from_rat {
    ($v:expr) => {
        if !$v.is_integer() {
            panic!("{} is not an integer", $v);
        } else {
            $v.to_integer().into()
        }
    };
}

macro_rules! maybe_answer {
    ($id:ident) => {
        if let Some(v) = $id {
            if v.is_integer() && v > Ratio::new(0, 1) {
                Some(v.to_integer().into())
            } else {
                None
            }
        } else {
            None
        }
    };
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

fn apply(op: &Op, arg1: Ratio<i32>, arg2: Ratio<i32>) -> Option<Ratio<i32>> {
    match op {
        Op::Add => arg1.checked_add(&arg2),
        Op::Sub => arg1.checked_sub(&arg2),
        Op::Mul => arg1.checked_mul(&arg2),
        Op::Div => arg1.checked_div(&arg2),
    }
}

struct Results {
    r1:Option<i32>,
    r2:Option<i32>,
    r3:Option<i32>,
    r4:Option<i32>,
    r5:Option<i32>,
}

impl Results {

    fn new(r1:Option<i32>, r2:Option<i32>, r3:Option<i32>, r4:Option<i32>, r5:Option<i32>) -> Self {
        Results { r1, r2, r3, r4, r5 }
    }
}

fn calculate_arrangement(
    nums: &Vec<&u8>,
    ops: &Vec<&Op>,
) -> Results {
    let result1:Option<Ratio<i32>> = try {
        apply(
            ops[1],
            apply(ops[0], mk_rat!(*nums[0]), mk_rat!(*nums[1]))?,
            apply(ops[2], mk_rat!(*nums[2]), mk_rat!(*nums[3]))?,
        )?
    };
    
    let result2a:Option<Ratio<i32>> = try {
        apply(
            ops[1],
            apply(
                ops[0],
                mk_rat!(*nums[0]),
                apply(ops[2], mk_rat!(*nums[1]), mk_rat!(*nums[2]))?,
            )?,
            mk_rat!(*nums[3])
        )?
    };

    let result2b:Option<Ratio<i32>> = try {
        apply(
            ops[1],
            apply(
                ops[0],
                apply(ops[2], mk_rat!(*nums[1]), mk_rat!(*nums[2]))?,
                mk_rat!(*nums[0])
            )?,
            mk_rat!(*nums[3])
        )?
    };

    let result3a:Option<Ratio<i32>> = try {
        apply(
            ops[2],
            mk_rat!(*nums[3]),
            apply(
                ops[1],
                mk_rat!(*nums[2]),
                apply(ops[0], mk_rat!(*nums[1]), mk_rat!(*nums[0]))?,
            )?,
        )?
    };

    let result3b:Option<Ratio<i32>> = try {
        apply(
            ops[2],
            mk_rat!(*nums[3]),
            apply(
                ops[1],
                apply(ops[0], mk_rat!(*nums[1]), mk_rat!(*nums[0]))?,
                mk_rat!(*nums[2])
            )?,
        )?
    };


    Results::new(
        maybe_answer!(result1),
        maybe_answer!(result2a),
        maybe_answer!(result2b),
        maybe_answer!(result3a),
        maybe_answer!(result3b),
    )
}

fn run_length(nums: [u8; 4]) -> u32 {
    let ops = [Op::Add, Op::Sub, Op::Mul, Op::Div];
    let mut results = BTreeSet::new();

    for num_arrangement in nums.iter().permutations(4) {
        for ops_arrangement in (0..3).map(|_i| ops.iter()).multi_cartesian_product() {
//            println!("checking {:?}, {:?}", num_arrangement, ops_arrangement);
            let r = calculate_arrangement(&num_arrangement, &ops_arrangement);
            if let Some(v) = r.r1 {
                results.insert(v);
            }
            if let Some(v) = r.r2 {
                results.insert(v);
            }
            if let Some(v) = r.r3 {
                results.insert(v);
            }
            if let Some(v) = r.r4 {
                results.insert(v);
            }
            if let Some(v) = r.r5 {
                results.insert(v);
            }
}
    }
//    println!("results: {:?}", results);
    let mut count = 0;
    for i in 1.. {
        if results.contains(&i) {
            count +=1;
        } else {
            break;
        }
    }
    count
}

fn main() {
    let mut max_len = 0;
    let mut vals = [0, 0, 0, 0];

    for a in 1..7 {
        for b in (a+1)..8 {
            for c in (b+1)..9 {
                for d in (c+1)..10 {
                    let len = run_length([a, b, c, d]);
                    if len > max_len {
                        max_len = len;
                        vals = [a, b, c, d];
                    }
                }
            }
        }
    }
    println!("max length is {}, with {:?}", max_len, vals);
}

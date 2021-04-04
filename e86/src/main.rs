/*
                 --------------------------
                |                         |
                |                         |
                |                         |
                |-------------------------|
                |                        F|
                |                         |3
                |                         |     3
     ------------                         -----------
    |                                                |
    |                                                |5
    |                                                |
    |                                                |
    -----------|                          |-----------
              S|                          |     3
               |                          |3
               |                          |
               |             6            |
               ----------------------------

*/

use std::collections::BTreeSet;
    
#[derive(Debug, Eq, PartialEq)]
struct Shape {
    a: u64,
    b: u64,
    c: u64
}

fn inc_shape(shape: &mut Shape, max_m: u64) -> bool {
    if shape.a == shape.b {
        if shape.b == shape.c {
            if shape.c == max_m {
                return false;
            }
            else {
                shape.c += 1;
                shape.a = 1;
                shape.b = 1;
            }
        }
        else {
            shape.b += 1;
            shape.a = 1;
        }
    }
    else {
        shape.a += 1;
    }
    return true;
}

fn main() {

    let mut squares = BTreeSet::new();
    let max_dist = 10000;
    let max_sq = max_dist * max_dist;
    
    for i in 1..max_dist+1 {
        squares.insert(i as u64 * i as u64);
    }

    let m = 1818;
    let mut solutions;
    
    let mut shape = Shape {a:1, b:1, c:1 };
    solutions = 0;
    
    'inner: loop {
        //        println!("check shape {:?}", shape);
        let sq_dist = shape.c.pow(2) + (shape.a+shape.b).pow(2);
        //        println!("{}", sq_dist);
        if sq_dist > max_sq {
            panic!("need more squares ({})", sq_dist);
        }
        if squares.contains(&sq_dist) {
            //                println!("{:?}", shape);
            solutions += 1;
        }
        if !inc_shape(&mut shape, m) {
            break 'inner;
        }
    }

    println!("found {} solutions with M = {}", solutions, m);
}

mod test {

    use super::*;
    
    #[test]
    fn test_inc1() {
        let mut s = Shape {a:1, b:1, c:1 };
        inc_shape(&mut s);
        assert_eq!(s, Shape {a:1, b:1, c:2 });
    }

    #[test]
    fn test_inc3() {
        let mut s = Shape {a:5, b:5, c:5 };
        inc_shape(&mut s);
        assert_eq!(s, Shape {a:1, b:1, c:6 });
    }

}

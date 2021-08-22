fn is_right_triangle(x1: i32, y1: i32, x2: i32, y2: i32) -> bool {
    let a = x1 * x1 + y1 * y1;
    let b = x2 * x2 + y2 * y2;
    let c = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);

    (a + b) - c == 0 || (a + c) - b == 0 || (b + c) - a == 0
}

fn main() {
    let mut n = 0u32;

    for x1 in 0..51 {
        for y1 in 0..51 {
            for x2 in 0..51 {
                for y2 in 0..51 {
                    if (x1 == 0 && y1 == 0) || (x2 == 0 && y2 == 0) {
                        continue;
                    }
                    if x1 == x2 && y1 == y2 {
                        continue;
                    }
                    if is_right_triangle(x1, y1, x2, y2) {
                        //println!("right triangle at ({}, {}), ({}, {})", x1, y1, x2, y2);
                        n += 1;
                    }
                }
            }
        }
    }
    println!("{}", n / 2);
}

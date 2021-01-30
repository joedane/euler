
const TARGET:u64 = 2_000_000;


fn main() {

    let mut rows:u64 = 1;
    let mut cols:u64;
    let mut best:(u64, u64) = (0, 0);
    let mut best_difference = std::u64::MAX;
    let mut best_rects = 0;
    
    loop {
        
        let c = (4*TARGET)/(rows*(rows+1));
        cols = ((-1.0 + ((1 + 4*c) as f64).sqrt()) / 2.0).round() as u64;
        let total_rectangles = (rows*(rows+1)) / 2 * (cols*(cols+1)) / 2;
//        println!("rows: {}, cols: {}, squares: {}", rows, cols, total_rectangles);
        let difference;
        if total_rectangles > TARGET {
            difference = total_rectangles - TARGET;
        } else {
            difference = TARGET - total_rectangles;
        }
        
        if difference < best_difference {
            best_difference = difference;
            best_rects = total_rectangles;
            best = (rows, cols);
        }
        
        if cols == 1 {
            break;
        } else {
            rows += 1;
        }
    }

    println!("best is ({}, {}), with {} rectangles", best.0, best.1, best_rects);
}

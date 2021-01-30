use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::cmp::{min, max};
use array2d::Array2D;

const SIZE:usize = 80;
const FILENAME: &str = "p081_matrix.txt";

#[derive(Debug, Clone, Copy)]
enum PathStep {
    Uninitialized,
    Right(u32),
    Down(u32),
    Finished(u32)
}

fn get_cost(ps: &PathStep) -> u32 {
    match ps {
        PathStep::Uninitialized => panic!("uninitialized"),
        PathStep::Right(v) => *v,
        PathStep::Down(v)  => *v,
        PathStep::Finished(v) => *v
    }
}

fn read_matrix<P>(filename: P) -> Array2D<u32>
where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let mut a = Array2D::filled_with(0, 80, 80);
    let mut reader = BufReader::new(file);

    for row in 0..SIZE {
        let mut line = String::new();
        let _ = reader.read_line(&mut line).unwrap();
        let row_vals = line.trim().split(',');
        let mut col = 0;
        for val_str in row_vals {
            let val:u32 = val_str.parse().unwrap(); 
            a.set(row, col, val).unwrap();
            col += 1;
        }
    }
    
    a
}

/*
 * return a square submatrix of size 'size' starting at the lower right corner of the 
 * given matrix.
*/
fn submatrix(m: &Array2D<u32>, size: usize) -> Array2D<u32> {
    let mut counter = 0;
    let m_size = m.num_rows();
    Array2D::filled_by_row_major(|| {
        let row = m_size - size + (counter/size);
        let col = m_size - size + (counter % size);
        counter += 1;
        
        *m.get(row, col).unwrap()
    }, size, size)
}

fn get_matrix() -> Array2D<u32> {
    return read_matrix(FILENAME);
    /*
    return Array2D::from_row_major(
        &vec![
            131, 673, 234, 103, 18,
            201, 96, 342, 965, 150,
            630, 803, 746, 422, 111,
            537, 699, 497, 121, 956,
            805, 732, 524, 37, 331],
        5, 5);
    */
}

fn main() {

    let matrix = get_matrix();
    let mut paths = Array2D::filled_with(PathStep::Uninitialized, SIZE, SIZE);

    for generation in 0..SIZE {
        // columns first
        let col = SIZE-1-generation;
        for row in (0..SIZE-generation).rev() {
            let step;
            if col == SIZE-1 {
//                println!("row {}, col {}", row, col);
                if row == SIZE-1 {
                    step = PathStep::Finished(matrix[(SIZE-1, SIZE-1)]);
                } else {
                    step = PathStep::Down(matrix[(row, col)] + get_cost(&paths[(row+1, col)]));
                }
            } else {
//                println!("get_cost row: {}, col: {}", row, col+1);
                let cost_right = get_cost(&paths[(row, col+1)]);
//                println!("get_cost row: {}, col: {}", row+1, col);
                let cost_down = get_cost(&paths[(row+1, col)]);
                if cost_right == cost_down {
                    panic!("Tie!");
                } else if cost_right < cost_down {
                    step = PathStep::Right(matrix[(row, col)] + get_cost(&paths[(row, col+1)]));
                }
                else {
                    step = PathStep::Down(matrix[(row, col)] + get_cost(&paths[(row+1, col)]));
                }
            }
            paths.set(row, col, step).unwrap();
        }

        let row = SIZE-1-generation;
        for col in (0..SIZE-1).rev() {
            let step;
  //          println!("row {}, col {}", row, col);
            if row == SIZE-1 {
                step = PathStep::Right(matrix[(row, col)] + get_cost(&paths[(row, col+1)]));
            } else {
//                println!("get_cost row: {}, col: {}", row, col+1);
                let cost_right = get_cost(&paths[(row, col+1)]);
//                println!("get_cost row: {}, col: {}", row+1, col);
                let cost_down = get_cost(&paths[(row+1, col)]);
                if cost_right == cost_down {
                    panic!("Tie!");
                } else if cost_right < cost_down {
                    step = PathStep::Right(matrix[(row, col)] + get_cost(&paths[(row, col+1)]));
                }
                else {
                    step = PathStep::Down(matrix[(row, col)] + get_cost(&paths[(row+1, col)]));
                }
                
            }
            paths.set(row, col, step).unwrap();
        }
    }

    println!("total cost: {}", get_cost(&paths[(0,0)]));

    let mut steps = 0;
    let (mut step_row, mut step_col) = (0,0);
    let mut at = &paths[(0, 0)];
        
    loop {
        match at {
            PathStep::Uninitialized => panic!("Bad path"),
            PathStep::Right(_) => { step_col += 1 },
            PathStep::Down(_) => { step_row += 1 },
            PathStep::Finished(_) => {break}
        }
        steps += 1;
        at = &paths[(step_row, step_col)];
    }

    println!("total steps: {}", steps);

}


#[cfg(test)]

mod tests {

    use crate::*;

    #[test]
    fn test_submatrix() {
        let m = Array2D::from_row_major(
            &vec![
                131, 673, 234, 103, 18,
                201, 96, 342, 965, 150,
                630, 803, 746, 422, 111,
                537, 699, 497, 121, 956,
                805, 732, 524, 37, 331],
            5, 5);

        let s = submatrix(&m, 2);

        assert_eq!(s, Array2D::from_row_major(
            &vec![121, 956,
                  37, 331], 2, 2));
    }
}

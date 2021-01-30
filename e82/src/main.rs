use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::cmp::{min};
use array2d::Array2D;

const FILENAME: &str = "p082_matrix.txt";

#[derive(Debug, Clone)]
enum Cell {
    Uninitialized,
    RightColumn(u32),
    TopRow(Cost),
    Internal(Cost),
    BottomRow(Cost)
}


#[derive(Debug, Clone)]
struct Cost {
    this:u32, up:Option<u32>, right:Option<u32>, down:Option<u32>
}

/**
 * minimum cost to complete from this cell, including the cost of this cell
 */
fn get_cost(cell: &Cell) -> u32 {
    match cell {
        Cell::Uninitialized => panic!("called get_cost on uninitialized cell"),
        Cell::RightColumn(v) => *v,
        Cell::TopRow(cost) => cost.this + min(cost.right.unwrap(), cost.down.unwrap()),
        Cell::BottomRow(cost) => cost.this + min(cost.right.unwrap(), cost.up.unwrap()),
        Cell::Internal(cost) => cost.this + min(cost.right.unwrap(), min(cost.down.unwrap(), cost.up.unwrap()))
    }
}

fn get_cost_from_below(cell: &Cell) -> u32 {
    match cell {
        Cell::TopRow(cost) => cost.this + cost.right.unwrap(),
        Cell::Internal(cost) => cost.this + min(cost.right.unwrap(), cost.up.unwrap()),
        _ => panic!("Can't come from below a cell of type {:?}", cell)
    }
}


fn read_matrix<P>(size:usize, filename: P) -> Array2D<u32>
where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let mut a = Array2D::filled_with(0, size, size);
    let mut reader = BufReader::new(file);

    for row in 0..size {
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

fn get_matrix() -> Array2D<u32> {
    return read_matrix(80, FILENAME);
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
    let size = matrix.num_rows();
    println!("size: {}", size);
    let mut paths = Array2D::filled_with(Cell::Uninitialized, size, size);

    for i in 0..size {
        paths.set(i, size-1, Cell::RightColumn(matrix[(i, size-1)]));
    }

    for col in (0..(size-1)).rev() {
//        println!("working on top row at column {}", col);
        paths.set(0, col, Cell::TopRow(Cost {
            this: matrix[(0, col)],
            up:None,
            right: Some(get_cost(&paths[(0, col+1)])),
            down:None
        }));
        for row in 1..(size-1) {
//            println!("working on cell ({}, {})", row, col);
            paths.set(row, col, Cell::Internal(Cost {
                this: matrix[(row, col)],
                up: Some(get_cost_from_below(&paths[(row-1, col)])),
                right: Some(get_cost(&paths[(row, col+1)])),
                down:None
            }
            ));
        }
        paths.set(size-1, col, Cell::BottomRow(Cost {
            this: matrix[(size-1, col)],
            up: Some(get_cost_from_below(&paths[(size-2, col)])),
            right: Some(get_cost(&paths[(size-1, col+1)])),
            down:None
        }));

        for row in (0..(size-1)).rev() {

            /*
            let mut cost = match &paths[(row, col)] {
                Cell::Internal(ref mut cost) => { cost }
                _ => panic!("Bad internal node")
            };
            */
  //          println!("setting down on cell ({}, {})", row, col);
            let cost_below = { // new scope to placate the borrow checker
                Some(get_cost(&paths[(row+1, col)]))
            };
            paths.get_mut(row, col).map(|cell| {
                match cell {
                    Cell::Internal(ref mut cost) => {
                        cost.down = cost_below
                    },
                    Cell::TopRow(ref mut cost) => {
                        cost.down = cost_below
                    },
                    _ => panic!("Bad internal node")
                }
            });
            //cost.down = Some(get_cost(&paths[(row+1, col)]));
            //            println!("set {:?}", paths[(row, col)]);
        }
        
    }

    let mut min_cost = std::u32::MAX;
    for row in 0..size {
        let cost = get_cost(&paths[(row, 0)]);
        if cost < min_cost {
            min_cost = cost;
        }
    }
    println!("min cost: {}", min_cost);
}
    

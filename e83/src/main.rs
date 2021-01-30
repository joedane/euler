use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::cmp::{Reverse};
use array2d::Array2D;
use priority_queue::PriorityQueue;

const FILENAME: &str = "p083_matrix.txt";


#[derive(Debug, Clone, Copy, Hash)]
struct Cell {
    row: usize,
    col: usize,
    distance: u32,
    cost: u32,
    prev: Option<(usize, usize)>
        
}


impl Cell {

    fn get_neighbor_coords(&self, size: usize) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        if self.row != 0 {
            v.push((self.row-1, self.col));
        }
        if self.col != size-1 {
            v.push((self.row, self.col+1));
        }
        if self.row != size-1 {
            v.push((self.row+1, self.col));
        }
        if self.col != 0 {
            v.push((self.row, self.col-1));
        }
        v
    }
}

/*
impl Eq for Cell {}

impl PartialEq for Cell {

    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}
 */

/*
impl Ord for (u32, u32) {
    
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Cell {

    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}
 */

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

fn initialize_paths(costs:&Array2D<u32>) -> Array2D<Cell> {

    let size = costs.num_rows();
    let (mut row, mut col) = (0, 0);
    Array2D::filled_by_row_major(|| {
        let cell = Cell {
            distance: if row == 0 && col == 0 { 0 } else { std::u32::MAX },
            cost: costs[(row, col)],
            row: row,
            col: col,
            prev: None
                
        };
        if col == size-1 {
            row += 1;
            col = 0;
        } else {
            col += 1;
        }
        cell       
    }, size, size)
}

fn initialize_queue(costs: &Array2D<u32>) -> PriorityQueue<(usize, usize), Reverse<u32>> {
    let mut q: PriorityQueue<(usize, usize), Reverse<u32>>  = PriorityQueue::new();
    for row in 0..costs.num_rows() {
        for col in 0..costs.num_columns() {
            if row == 0 && col == 0 {
                q.push((0, 0), Reverse(0));
            }
            else {
                q.push((row,col), Reverse(std::u32::MAX));
            }
        }
    }
    q
}

/*
fn remove_from_unvisited<'a>(coords: (usize, usize), paths: &'a Array2D<Cell>, q: &mut PriorityQueue<&'a Cell, u32>) {
    q.remove(&paths.get(coords.0, coords.1).unwrap());
}
 */

fn main() {

    // Djykstra's method

    let matrix = get_matrix();
    let mut paths = initialize_paths(&matrix);
    let size = matrix.num_rows();
    let mut unvisited_nodes = initialize_queue(&matrix);
    let (current_coord, _current_cost) = unvisited_nodes.pop().unwrap();
    let mut current = *paths.get_mut(current_coord.0, current_coord.1).unwrap();
    
    loop {
        
//        println!("visiting node at row {}, col {} with distance {}",
//                 current.row, current.col, current.distance);

        for neighbor_coord in current.get_neighbor_coords(size) {
//            println!("check: {:?}", neighbor_coord);
            if let Some(_v) = unvisited_nodes.get(&neighbor_coord) {
                let mut neighbor = &mut paths[neighbor_coord];
//                println!("neighbor at: {:?}", neighbor_coord);
                let new_distance = current.distance + neighbor.cost;
                if new_distance < neighbor.distance {
//                    println!("set neighbor at ({}, {}) to {}", neighbor.row, neighbor.col, new_distance);
                    neighbor.distance = new_distance;
                    unvisited_nodes.change_priority(&(neighbor.row, neighbor.col), Reverse(new_distance));
                    neighbor.prev = Some((current.row, current.col));
                }
            }
        }
        if let Some(next) = unvisited_nodes.pop() {
            current = *paths.get_mut(next.0.0, next.0.1).unwrap();
        } else {
            break;
        }
        /*
        paths.get_mut(current.0, current.1).unwrap().state = State::Visited;
        let current_distance = paths.get(current.0, current.1).unwrap().distance;
        for neighbor_coord in paths.get(current.0, current.1).unwrap().get_neighbor_coords(size) {
            let (row, col) = neighbor_coord;
            let mut neighbor = paths.get_mut(row, col).unwrap();
            let new_distance = current_distance + matrix[(row, col)];
            if new_distance < neighbor.distance {
                neighbor.distance = new_distance;
            }
        }
        if let Some(next) = unvisited_nodes.pop() {
            current = (next.0.row, next.0.col);
        }
        else {
            break;
        }
        */
    }

    let mut c = paths.get(size-1, size-1).unwrap();
    println!("{}", c.distance + paths.get(0,0).unwrap().cost);

    loop {
        println!("prev: {:?}", c.prev);
        match c.prev {
            Some(p) => { c = paths.get(p.0, p.1).unwrap(); },
            None => break
        }
    }
    /*
    current.get_neighbors_mut(&mut paths);
    */
}

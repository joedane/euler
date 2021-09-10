use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fmt;

#[derive(Debug, Copy, Clone)]
struct Item(u32);


impl From<char> for Item {

    fn from(item: char) -> Self {
        Item(item.to_digit(10).unwrap())
    }
}

impl From<u32> for Item {

    fn from(item: u32) -> Self {
        Item(item)
    }
}

impl PartialEq for Item {

    fn eq(&self, other:&Self) -> bool {
        self.0 == other.0
    }
}


type Grid = [[Item; 9]; 9];

type Coord = (usize, usize);

#[derive(Clone)]
struct Board {
    empties: Vec<(usize, usize)>,
    grid: [[Item; 9]; 9]
}

impl Board {

    fn find_empties(grid: &Grid) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if grid[i][j].0 == 0 {
                    v.push((i, j));
                }
            }
        }
        v
    }

    fn extend(&self, empty:(usize, usize), try_item:Item) -> Self {
        let mut v = Vec::new();
        for coord in &self.empties {
            if *coord != empty {
                v.push(coord.clone());
            }
        }
        let mut newgrid = self.grid.clone();
        newgrid[empty.0][empty.1] = try_item;
        Board {
            empties: v,
            grid: newgrid
        }
    }

    fn mask_for_value(item: Item) -> u32 {
        if item <= 0 || item > 9 {
            panic!("invalid item value");
        }
        1 << item
    }

    fn value_for_mask_single(mask: u32) -> Item {

        match mask {
            1 => 1,
            1 << 1 => Item::from(2),
            1 << 2 => Item::from(3),
            1 << 3 => Item::from(4),
            1 << 4 => Item::from(5),
            1 << 5 => Item::from(6),
            1 << 6 => Item::from(7),
            1 << 7 => Item::from(8),
            1 << 8 => Item::from(9),
            _ {
                panic!("invalid mask");
            }
        }
    }

    fn value_for_mask_double(mask: u32) -> (Item, Item) {
        let mut i = 1;

        if mask == 0 {
            panic!("invalid mask (zero)");
        }
        
        while (mask & 1 == 0) {
            mask >>= mask;
            i += 1;
        }
        let item1 = Item::from(i);
        mask ^= 1;
        if mask == 0 {
            panic!("invalid mask (one bit only)");
        }
        while (mask & 1 == 0) {
            mask >>= 1;
            i+=1;
        }
        (item1, Item::from(i));
    }
    

    fn try_fill_by_rows(&mut self) -> Option<Coord> {
        'row: for i in 0..9 {
            let mut tries = vec![];
            let mut mask = 0u32;
            for j in 0..9 {
                if self.grid[i][j] == 0 {
                    if tries.len() == 2 {
                        // give up
                        continue 'row;
                    }
                    else {
                        tries.push(j);
                    }
                }
                mask |= Self::mask_for_value(self.grid[i][j]);
            }
            match tries.len() {

                1 => {
                    self.grid[i][tries[0]] = value_for_mask_single(mask);
                    Some((i, j))
                    
                },
                2 => {
                    let (try1, try2) = value_for_mask_double(mask);
                    if !self.is_valid((i, tries[0]), try1) {
                        self.grid[i][tries[1]] = try1;
                        Some((i, tries[1]))
                    }
                    else if !self.is_valid((i, tries[1]), try1) {
                        self.grid[i][tries[0]] = try1;
                        Some((i, tries[0]))
                    }
                    else if !self.is_valid((i, tries[0]), try2) {
                        self.grid[i][tries[1]] = try2;
                        Some((i, tries[1]))
                    }
                    else if !self.is_valid((i, tries[1]), try2) {
                        self.grid[i][tries[0]] = try2;
                        Some((i. tries[0]))
                    }
                    else {
                        None
                    }
                },
                _ => None
            }
        }
    }
    
    
    /**
     * returns true if the given Item can be placed at the location of the empty slot
     */
    fn is_valid(&self, empty: &Coord, try_item:Item) -> bool {
        let (row, col) = *empty;
        let (box_row, box_col) = (empty.0 / 3, empty.1 / 3);
        
        !(
            self.grid[row][0] == try_item ||
                self.grid[row][1] == try_item ||
                self.grid[row][2] == try_item ||
                self.grid[row][3] == try_item ||
                self.grid[row][4] == try_item ||
                self.grid[row][5] == try_item ||
                self.grid[row][6] == try_item ||
                self.grid[row][7] == try_item ||
                self.grid[row][8] == try_item ||

                self.grid[0][col] == try_item ||
                self.grid[1][col] == try_item ||
                self.grid[2][col] == try_item ||
                self.grid[3][col] == try_item ||
                self.grid[4][col] == try_item ||
                self.grid[5][col] == try_item ||
                self.grid[6][col] == try_item ||
                self.grid[7][col] == try_item ||
                self.grid[8][col] == try_item ||
                
                self.grid[box_row][box_col] == try_item ||
                self.grid[box_row][box_col+1] == try_item ||
                self.grid[box_row][box_col+2] == try_item ||
                
                self.grid[box_row+1][box_col] == try_item ||
                self.grid[box_row+1][box_col+1] == try_item ||
                self.grid[box_row+1][box_col+2] == try_item ||
                
                self.grid[box_row+2][box_col] == try_item ||
                self.grid[box_row+2][box_col+1] == try_item ||
                self.grid[box_row+2][box_col+2] == try_item
        )
    }
    
    
    fn new(grid: [[Item; 9]; 9]) -> Self {
        Board {
            empties:Board::find_empties(&grid),
            grid:grid
        }
    }
}

impl fmt::Debug for Board {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            f.write_fmt(format_args!("{}{}{} {}{}{} {}{}{}\n",
                                     self.grid[i][0].0, 
                                     self.grid[i][1].0, 
                                     self.grid[i][2].0, 
                                     self.grid[i][3].0, 
                                     self.grid[i][4].0, 
                                     self.grid[i][5].0, 
                                     self.grid[i][6].0, 
                                     self.grid[i][7].0, 
                                     self.grid[i][8].0
            ))?;
            if i == 2 || i == 5 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}


fn read_boards<P>(filename: P) -> Result<Vec<Board>, &'static str>
where P: AsRef<Path> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut boards = Vec::new();
    let mut lines = reader.lines();

    for _i in 0..50 {
        if let Ok(board) = read_board(&mut lines) {
            boards.push(board);
        }
    }
    Ok(boards)
}

fn read_board<B: BufRead>(lines: &mut std::io::Lines<B>) -> Result<Board, &'static str> {
    let title = lines.next().unwrap().unwrap();
    let mut items: Grid = [[Item(0); 9]; 9];
    
    for i in 0..9 {
        let line = lines.next();
        for (j, c) in line.unwrap().unwrap().chars().enumerate() {
//            println!("read: {}, {}", j, c);
            items[i][j] = Item::from(c);
        }
    }
    Ok(Board::new(items))
}
    
fn solve(board: &Board) -> Option<Board> {
    if board.empties.len() == 0 {
        Some(board.clone());
    }
    // println!("solving ...");
    // println!("{:?}", *board);
    // println!("empties: {} {:?}", board.empties.len(), board.empties);
    for empty in &board.empties {
        for test in (1..10).map(|i| Item::from(i)) {
            if board.is_valid(empty, test) {
//                println!("trying {:?} at index {:?}", test, empty);
                if let Some(answer) = solve(&board.extend(*empty, test)) {
                    return Some(answer);
                }
            }
        }
    }
    None
}

fn main() {
    let boards = read_boards("p096_sudoku.txt").unwrap();
    println!("solving ...");
    println!("{:?}", boards[0]);
    match solve(&boards[0]) {
        None => { println!("No solution") },
        Some(board) => { println!("{:?}", board) }
    }
}

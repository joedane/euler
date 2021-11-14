use lazy_static::lazy_static;
use log::{error, info};
use simplelog::*;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::rc::{Rc, Weak};
use std::slice::SliceIndex;
use std::sync::Mutex;

#[derive(Debug, Copy, Clone)]
struct Item(u32);

impl Item {
    fn is_zero(&self) -> bool {
        self.0 == 0
    }

    fn next(&self) -> Option<Item> {
        if self.0 < 9 {
            Some(Item(self.0 + 1))
        } else {
            None
        }
    }
}

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

impl From<i32> for Item {
    fn from(item: i32) -> Self {
        Item(item as u32)
    }
}

impl From<u64> for Item {
    fn from(item: u64) -> Self {
        Item(item as u32)
    }
}

impl From<Item> for u64 {
    fn from(item: Item) -> Self {
        item.0 as u64
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<Item> for u32 {
    fn eq(&self, other: &Item) -> bool {
        *self == other.0
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

type Grid = [[Item; 9]; 9];

type Coord = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum Order {
    ByRow,
    ByCol,
    ByCell,
}

struct ItemIterator {
    order: Order,
    step: usize,
    next: usize,
}

impl ItemIterator {
    fn new(order: Order, step: usize) -> Self {
        ItemIterator {
            order,
            step,
            next: 0,
        }
    }

    fn make_coord(&self) -> Coord {
        match self.order {
            Order::ByRow => (self.step, self.next),
            Order::ByCol => (self.next, self.step),
            Order::ByCell => {
                let row_start = (self.step / 3) * 3;
                let col_start = (self.step % 3) * 3;
                (row_start + self.next / 3, col_start + self.next % 3)
            }
        }
    }
}

impl Iterator for ItemIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < 9 {
            let n = Some(self.make_coord());
            self.next += 1;
            n
        } else {
            None
        }
    }
}

struct SectionIterator {
    order: Order,
    next: usize,
}

impl SectionIterator {
    fn new(order: Order) -> Self {
        SectionIterator { order, next: 0 }
    }
}

impl Iterator for SectionIterator {
    type Item = ItemIterator;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next < 9 {
            let n = Some(ItemIterator::new(self.order, self.next));
            self.next += 1;
            n
        } else {
            None
        }
    }
}

fn make_iterator(order: Order) -> SectionIterator {
    SectionIterator::new(order)
}

fn make_row_iterator() -> SectionIterator {
    SectionIterator::new(Order::ByRow)
}

fn make_col_iterator() -> SectionIterator {
    SectionIterator::new(Order::ByCol)
}

fn make_cell_iterator() -> SectionIterator {
    SectionIterator::new(Order::ByCell)
}

#[derive(Clone)]
struct TransitionData {
    item: Item,
    coord: Coord,
}

enum Transition {
    None,
    One(Vec<TransitionData>),
    Invalid,
}

#[derive(Debug)]
struct TrialItem<'a> {
    coord: Coord,
    score: u8,
    board: &'a Board,
}

impl<'a> PartialOrd for TrialItem<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.score.cmp(&other.score))
    }
}

impl<'a> PartialEq<TrialItem<'a>> for TrialItem<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<'a> Ord for TrialItem<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl<'a> Eq for TrialItem<'a> {}

impl<'a> TrialItem<'a> {
    fn get_possible_items(&self) -> TrialItemIterator {
        TrialItemIterator {
            next: Some(Item::from(1)),
            coord: self.coord,
            board: self.board,
        }
    }
}
struct TrialItemIterator<'a> {
    next: Option<Item>,
    coord: Coord,
    board: &'a Board,
}

impl<'a> Iterator for TrialItemIterator<'a> {
    type Item = Item;
    fn next(&mut self) -> Option<Item> {
        let mut next = self.next;
        while next.is_some() && !self.board.check_valid(&self.coord, next.unwrap()) {
            next = next.unwrap().next();
        }
        if next.is_none() {
            return None;
        } else {
            self.next = next.unwrap().next();
            return next;
        }
    }
}

struct EmptyIterator<'a> {
    empties: BinaryHeap<TrialItem<'a>>,
}

impl<'a> Iterator for EmptyIterator<'a> {
    type Item = TrialItem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.empties.pop()
    }
}

struct DBGNode {
    code: String,
    board: Board,
    transitions: Option<Vec<TransitionData>>,
    parent: Weak<Cell<DBGNode>>,
    children: Vec<Rc<Cell<DBGNode>>>,
}

impl DBGNode {
    fn new(code: String, board: Board, parent: Weak<RefCell<Self>>) -> Self {
        DBGNode {
            code,
            board,
            parent,
            transitions: None,
            children: vec![],
        }
    }

    fn add_transitions(&mut self, mut txs: Vec<TransitionData>) {
        if self.transitions.is_none() {
            self.transitions = Some(txs)
        } else {
            self.transitions.as_mut().unwrap().append(&mut txs)
        }
    }
}

#[derive(Clone)]
struct Board {
    title: String,
    empties: Vec<(usize, usize)>,
    grid: [[Item; 9]; 9],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#Board: [{}]", self.to_digits());
        Ok(())
    }
}

const CHARS: [u8; 16] = [
    0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46,
];

impl Board {
    fn to_digits(&self) -> String {
        /**
         * write sixteen ASCII characters correspoding to a hex encoding of val, starting at pos
         */
        fn w(s: &mut Vec<u8>, pos: usize, len: usize, val: u64) {
            let mut d = val;
            let mut offset = len - 1;
            loop {
                s[pos + offset] = CHARS[(d & 0xF) as usize];
                d >>= 4;
                if offset >= 1 {
                    offset -= 1;
                } else {
                    break;
                }
            }
        }

        let mut s: Vec<u8> = vec![0x78; 16 * 4 + 5];

        for i in 0..4 {
            let mut v = 0;
            for j in 0..19 {
                let c = i * 19 + j;
                v *= 10;
                /*
                 *  rust-ism below. can this be simplified?
                 */
                v += Into::<u64>::into(self.grid[c / 9][c % 9]);
            }
            w(&mut s, i * 16, 16, v);
        }

        let mut v = Into::<u64>::into(self.grid[8][4]);
        v *= 10;
        v += Into::<u64>::into(self.grid[8][5]);
        v *= 10;
        v += Into::<u64>::into(self.grid[8][6]);
        v *= 10;
        v += Into::<u64>::into(self.grid[8][7]);
        v *= 10;
        v += Into::<u64>::into(self.grid[8][8]);
        w(&mut s, 64, 5, v);

        String::from_utf8(s).unwrap()
    }

    fn empty_iter(&self) -> EmptyIterator {
        let mut trials = BinaryHeap::new();
        for empty in &self.empties {
            trials.push(self.get_score(*empty));
        }
        EmptyIterator { empties: trials }
    }

    fn get_score(&self, empty: Coord) -> TrialItem {
        let mut c = 0;
        let mut score = 0;
        for i in 0..9 {
            if self.grid[empty.0][i].is_zero() {
                c += 1;
            }
        }
        score += 9 - c;
        if c == 1 {
            score += 5;
        }

        c = 0;
        for i in 0..9 {
            if self.grid[i][empty.1].is_zero() {
                c += 1;
            }
        }
        score += 9 - c;
        if c == 1 {
            score += 1;
        }

        c = 0;
        let row_start = (empty.0 / 3) * 3;
        let col_start = (empty.1 / 3) * 3;
        for i in 0..9 {
            let ii = row_start + (i / 3);
            let j = col_start + (i % 3);
            if self.grid[ii][j].is_zero() {
                c += 1;
            }
        }
        score += 9 - c;
        if c == 1 {
            score += 5;
        }
        TrialItem {
            coord: empty,
            score,
            board: self,
        }
    }

    /**
     * this will pick the lowest possible value, arbitrarily
     */
    fn pick_item(mask: u32) -> Item {
        let mut i = 1;
        let mut mask = mask;
        while mask & 1 == 0 {
            i += 1;
            mask >>= 1;
        }
        Item::from(i)
    }

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

    fn extend(&self, new_coord: &Coord, item: Item) -> Self {
        info!("extending with {:?} at {:?}", item, new_coord);
        let mut v = Vec::new();
        for coord in &self.empties {
            if *coord != *new_coord {
                v.push(coord.clone());
            } else {
                info!("skipping empty: {:?}", new_coord);
            }
        }
        let mut newgrid = self.grid.clone();
        newgrid[new_coord.0][new_coord.1] = item;
        Board {
            title: self.title.clone(),
            empties: v,
            grid: newgrid,
        }
    }

    fn fill(&mut self, coord: Coord, item: Item) {
        //     println!("fill {:?} at {:?}", item, coord);
        self.grid[coord.0][coord.1] = item;
        self.empties
            .swap_remove(self.empties.iter().position(|x| *x == coord).unwrap());
    }

    fn invert_mask(mask: u32) -> u32 {
        !mask & 0x1FF
    }

    fn mask_for_value(item: Item) -> u32 {
        let val = item.0;
        if val <= 0 || val > 9 {
            panic!("invalid item value: {:?}", item);
        }
        1 << (val - 1)
    }

    const MASK_1: u32 = 1u32;
    const MASK_2: u32 = 1u32 << 1;
    const MASK_3: u32 = 1u32 << 2;
    const MASK_4: u32 = 1u32 << 3;
    const MASK_5: u32 = 1u32 << 4;
    const MASK_6: u32 = 1u32 << 5;
    const MASK_7: u32 = 1u32 << 6;
    const MASK_8: u32 = 1u32 << 7;
    const MASK_9: u32 = 1u32 << 8;

    fn value_for_mask_single(mask: u32) -> Item {
        match mask {
            i if i == Board::MASK_1 => Item::from(1),
            i if i == Board::MASK_2 => Item::from(2),
            i if i == Board::MASK_3 => Item::from(3),
            i if i == Board::MASK_4 => Item::from(4),
            i if i == Board::MASK_5 => Item::from(5),
            i if i == Board::MASK_6 => Item::from(6),
            i if i == Board::MASK_7 => Item::from(7),
            i if i == Board::MASK_8 => Item::from(8),
            i if i == Board::MASK_9 => Item::from(9),
            _ => {
                panic!("invalid mask: {:b}", mask);
            }
        }
    }

    fn value_for_mask_double(mask: u32) -> (Item, Item) {
        let mut i = 1;
        let mut mask = mask;
        if mask == 0 {
            panic!("invalid mask (zero)");
        }
        while mask & 1 == 0 {
            mask >>= 1;
            i += 1;
        }
        let item1 = Item::from(i);
        mask ^= 1;
        if mask == 0 {
            panic!("invalid mask (one bit only)");
        }
        while mask & 1 == 0 {
            mask >>= 1;
            i += 1;
        }
        (item1, Item::from(i))
    }

    fn try_fill_once(&mut self) -> Transition {
        match self.try_fill_by_rows() {
            Transition::None => {
                match self.try_fill_by_cols() {
                    // gotta be a way to bubble up matches?
                    Transition::None => self.try_fill_by_cells(),
                    Transition::One(k) => Transition::One(k),
                    Transition::Invalid => Transition::Invalid,
                }
            }
            Transition::One(k) => Transition::One(k),
            Transition::Invalid => Transition::Invalid,
        }
    }

    fn try_fill_by_rows(&mut self) -> Transition {
        self.try_fill(Order::ByRow)
    }

    fn try_fill_by_cols(&mut self) -> Transition {
        self.try_fill(Order::ByCol)
    }

    fn try_fill_by_cells(&mut self) -> Transition {
        self.try_fill(Order::ByCell)
    }

    /**
     *  Attempt to find a row, column, or cell in which one or two entries
     * are "forced", i.e., required by the current layout of the board.
     *  returns None if we're unable to make any progrss, Some(k) if we
     * updated k entries, or Invalid if the current board cannot be
     * completed.
     */
    fn try_fill(&mut self, order: Order) -> Transition {
        info!("try: {:?}", order);
        //    println!("starting with:\n{:?}", self);
        'section: for section in make_iterator(order) {
            let mut tries = vec![];
            let mut mask = 0u32;
            for coord in section {
                let (i, j) = (coord.0, coord.1);
                if self.grid[i][j] == Item::from(0) {
                    if tries.len() == 2 {
                        // give up
                        continue 'section;
                    } else {
                        tries.push(coord);
                    }
                } else {
                    mask |= Self::mask_for_value(self.grid[i][j]);
                }
            }
            match tries.len() {
                1 => {
                    let value = Board::value_for_mask_single(Board::invert_mask(mask));
                    if self.check_valid(&tries[0], value) {
                        self.fill(tries[0], value);
                        info!("filled {:?} with {:?}", tries[0], value);
                        return Transition::One(vec![TransitionData {
                            item: value,
                            coord: tries[0],
                        }]);
                    } else {
                        info!("INVLIAD C");
                        return Transition::Invalid;
                    }
                }
                2 => {
                    let (try1, try2) = Board::value_for_mask_double(Board::invert_mask(mask));
                    let (c1, c2) = (tries[0], tries[1]);
                    info!("trying {:?} and {:?} at {:?} and {:?}", try1, try2, c1, c2);
                    if !self.check_valid(&c1, try1) || !self.check_valid(&c2, try2) {
                        if self.check_valid(&c1, try2) && self.check_valid(&c2, try1) {
                            self.fill(c2, try1);
                            self.fill(c1, try2);
                            //        println!("[B] Board is now:\n{:?}", self);
                            info!(
                                "filled {:?} with {:?} and {:?} with {:?}",
                                c2, try1, c1, try2
                            );
                            return Transition::One(vec![
                                TransitionData {
                                    item: try1,
                                    coord: c2,
                                },
                                TransitionData {
                                    item: try2,
                                    coord: c1,
                                },
                            ]);
                        } else {
                            info!("INVAID A");
                            return Transition::Invalid;
                        }
                    } else if !self.check_valid(&c2, try1) || !self.check_valid(&c1, try2) {
                        if self.check_valid(&c1, try1) && self.check_valid(&c2, try2) {
                            self.fill(c1, try1);
                            self.fill(c2, try2);
                            //       println!("[C] Board is now:\n{:?}", self);
                            info!(
                                "filled {:?} with {:?} and {:?} with {:?}",
                                c1, try1, c2, try2
                            );
                            return Transition::One(vec![
                                TransitionData {
                                    item: try1,
                                    coord: c1,
                                },
                                TransitionData {
                                    item: try2,
                                    coord: c2,
                                },
                            ]);
                        } else {
                            info!("INVAID B");
                            return Transition::Invalid;
                        }
                    } else {
                        continue 'section;
                    }
                }
                _ => {
                    continue 'section;
                }
            }
        }
        return Transition::None;
    }

    fn assert_valid(&self) {
        'i: for i in 0..9 {
            for j in 0..9 {
                let check_item = self.grid[i][j];
                if check_item == Item::from(0) {
                    continue 'i;
                }
                for col in 0..9 {
                    if col != j && check_item == self.grid[i][col] {
                        panic!(
                            "invalid by row: item: {:?} (i,j): ({}, {})\n{:?}",
                            check_item, i, j, self
                        );
                    }
                }
                for row in 0..9 {
                    if row != i && check_item == self.grid[row][j] {
                        panic!(
                            "invalid by col: item: {:?} (i,j): ({}, {})\n{:?}",
                            check_item, i, j, self
                        );
                    }
                }
                let row_start = (i / 3) * 3;
                let col_start = (j / 3) * 3;
                //    println!("row_start: {}, col_start: {}  ({}, {})", row_start, col_start, i, j);
                for k in 0..9 {
                    let this_i = row_start + k / 3;
                    let this_j = col_start + k % 3;
                    if !(this_i == i && this_j == j) && self.grid[this_i][this_j] == check_item {
                        panic!(
                            "invalid by cell: item: {:?} (i,j): ({}, {})  this: ({}, {})\n{:?}",
                            check_item, i, j, this_i, this_j, self
                        );
                    }
                }
            }
        }
    }

    /**
     * returns true if the given Item can be placed at the location of the empty slot
     */
    fn check_valid(&self, empty: &Coord, try_item: Item) -> bool {
        let (row, col) = *empty;
        let (box_row_start, box_col_start) = ((empty.0 / 3) * 3, (empty.1 / 3) * 3);
        //println!("checking {:?} valid at {:?}, box ({}, {})", try_item, empty, box_row_start, box_col_start);

        !(self.grid[row][0] == try_item
            || self.grid[row][1] == try_item
            || self.grid[row][2] == try_item
            || self.grid[row][3] == try_item
            || self.grid[row][4] == try_item
            || self.grid[row][5] == try_item
            || self.grid[row][6] == try_item
            || self.grid[row][7] == try_item
            || self.grid[row][8] == try_item
            || self.grid[0][col] == try_item
            || self.grid[1][col] == try_item
            || self.grid[2][col] == try_item
            || self.grid[3][col] == try_item
            || self.grid[4][col] == try_item
            || self.grid[5][col] == try_item
            || self.grid[6][col] == try_item
            || self.grid[7][col] == try_item
            || self.grid[8][col] == try_item
            || self.grid[box_row_start][box_col_start] == try_item
            || self.grid[box_row_start][box_col_start + 1] == try_item
            || self.grid[box_row_start][box_col_start + 2] == try_item
            || self.grid[box_row_start + 1][box_col_start] == try_item
            || self.grid[box_row_start + 1][box_col_start + 1] == try_item
            || self.grid[box_row_start + 1][box_col_start + 2] == try_item
            || self.grid[box_row_start + 2][box_col_start] == try_item
            || self.grid[box_row_start + 2][box_col_start + 1] == try_item
            || self.grid[box_row_start + 2][box_col_start + 2] == try_item)
    }

    fn new(title: String, grid: [[Item; 9]; 9]) -> Self {
        Board {
            title,
            empties: Board::find_empties(&grid),
            grid,
        }
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..9 {
            f.write_fmt(format_args!(
                "{}{}{} {}{}{} {}{}{}\n",
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
where
    P: AsRef<Path>,
{
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
    Ok(Board::new(title, items))
}

lazy_static! {
    static ref BOARDS: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

fn dump(prefix: String, node: &DBGNode, boards: bool) {
    let start_boards = node
        .code
        .eq("#Board: [22C2D9116AC9D5A82A6985C44DC6A6805FAC68D71E7C0D0D5B4ED23995B67D1E04653]");

    println!("{}{}", prefix, node.code);
    if boards || start_boards {
        println!("{:?}", node.board);
    }
    for child in &node.children {
        dump(
            format!("{}{}", prefix, "   "),
            &child.borrow(),
            boards || start_boards,
        );
    }
}

fn trace_boards(code: &str, root: &DBGNode) {
    /*
    let nav_root = root.navigate(None);
    let mut path_vec = vec![];
    path_vec.push(nav_root);
    */
}

fn solve(board: &mut Board, this_node: Rc<RefCell<DBGNode>>) -> Option<Board> {
    info!("solving \n{:?}", *board);
    info!("code: {}", *board);
    if board.empties.len() == 0 {
        return Some(board.clone());
    }

    {
        let mut B = BOARDS.lock().unwrap();
        let s = format!("{}", board);
        if B.contains(&s) {
            panic!("loop detected: {}", s);
            //return None;
        }
        B.insert(s);
    }

    //println!("solving ...");
    //println!("{:?}", *board);
    info!("empties: {} {:?}", board.empties.len(), board.empties);
    loop {
        match board.try_fill_once() {
            Transition::Invalid => {
                info!("FOO!");
                return None;
            }
            Transition::None => break,
            Transition::One(v) => {
                info!("filled {}", v.len());
                this_node.borrow_mut().add_transitions(v);
                //board.assert_valid();
            }
        }
    }
    info!("empties: {}", board.empties.len());
    info!("board is now\n{:?}", board);
    if board.empties.len() == 0 {
        return Some(board.clone());
    }
    for empty in board.empty_iter() {
        for item in empty.get_possible_items() {
            let mut extended_board = board.extend(&empty.coord, item);
            let mut new_node = Rc::new(RefCell::new(DBGNode::new(
                format!("{}", extended_board),
                extended_board.clone(),
                Rc::downgrade(&this_node),
            )));
            this_node.borrow_mut().children.push(new_node);
            if let Some(answer) = solve(&mut extended_board, new_node) {
                return Some(answer);
            }
        }
    }
    None
}

fn main() {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    let mut boards = read_boards("p096_sudoku.txt").unwrap();
    let mut root = DBGNode::new("ROOT".to_string(), boards[1].clone());

    if let Err(e) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        solve(&mut boards[1], &mut root);
    })) {
        dump("".to_string(), &root, false);
    }

    /*
    for mut board in boards {
        info!("solving board {}", board.title);
        match solve(&mut board) {
            None => { panic!("No solution") },
            Some(board) => { println!("{:?}", board) }
        }
    }
    */
}

#[cfg(test)]
mod tests {

    use super::*;
    use stringreader::StringReader;

    #[test]
    fn test_read1() {
        let sr = StringReader::new(
            "Test Board\n\
        123456789\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000",
        );
        read_board(&mut BufReader::new(sr).lines()).unwrap();
    }

    #[test]
    fn test_read2() {
        let sr = StringReader::new(
            "Test Board\n\
        123456780\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000\n\
        000000000",
        );
        let mut board = read_board(&mut BufReader::new(sr).lines()).unwrap();
        if let Transition::One(_) = board.try_fill_by_rows() {
            assert_eq!(9, board.grid[0][8]);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_read3() {
        let sr = StringReader::new(
            "Test Board\n\
        123450780\n\
        000000000\n\
        000000000\n\
        000006000\n\
        000000000\n\
        000000000\n\
        000000009\n\
        000000000\n\
        000000000",
        );
        let mut board = read_board(&mut BufReader::new(sr).lines()).unwrap();
        if let Transition::One(_) = board.try_fill_by_rows() {
            assert!(
                6 == board.grid[0][5]
                    || 6 == board.grid[0][8]
                    || 9 == board.grid[0][5]
                    || 9 == board.grid[0][8]
            );
        } else {
            panic!();
        }
        println!("board is now: {:?}", board);
    }

    #[test]
    fn test_read4() {
        let sr = StringReader::new(
            "Test Board\n\
        123000000\n\
        456000000\n\
        089000000\n\
        000006000\n\
        000000000\n\
        000000000\n\
        000000009\n\
        000000000\n\
        000000000",
        );
        let mut board = read_board(&mut BufReader::new(sr).lines()).unwrap();
        if let Transition::One(_) = board.try_fill_by_cells() {
            assert_eq!(7, board.grid[2][0]);
        } else {
            panic!();
        }
        println!("board is now: {:?}", board);
    }

    #[test]
    fn test_iter1() {
        let mut rows = make_row_iterator();
        if let Some(mut col) = rows.next() {
            assert_eq!(col.next().unwrap(), (0, 0));
            assert_eq!(col.next().unwrap(), (0, 1));
            assert_eq!(col.next().unwrap(), (0, 2));
            assert_eq!(col.next().unwrap(), (0, 3));
            assert_eq!(col.next().unwrap(), (0, 4));
            assert_eq!(col.next().unwrap(), (0, 5));
            assert_eq!(col.next().unwrap(), (0, 6));
            assert_eq!(col.next().unwrap(), (0, 7));
            assert_eq!(col.next().unwrap(), (0, 8));
            if let Some(last) = col.next() {
                panic!("Expected the end of a row, found {:?}", last);
            }
            if let Some(mut col) = rows.next() {
                assert_eq!(col.next().unwrap(), (1, 0));
                assert_eq!(col.next().unwrap(), (1, 1));
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn test_iter2() {
        let mut i = make_col_iterator();
        if let Some(mut j) = i.next() {
            assert_eq!(j.next().unwrap(), (0, 0));
            assert_eq!(j.next().unwrap(), (1, 0));
            assert_eq!(j.next().unwrap(), (2, 0));
            assert_eq!(j.next().unwrap(), (3, 0));
            assert_eq!(j.next().unwrap(), (4, 0));
            assert_eq!(j.next().unwrap(), (5, 0));
            assert_eq!(j.next().unwrap(), (6, 0));
            assert_eq!(j.next().unwrap(), (7, 0));
            assert_eq!(j.next().unwrap(), (8, 0));
            if let Some(last) = j.next() {
                panic!("Expected the end of a row, found {:?}", last);
            }
            if let Some(mut col) = i.next() {
                assert_eq!(col.next().unwrap(), (0, 1));
                assert_eq!(col.next().unwrap(), (1, 1));
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn test_iter3() {
        let mut i = make_cell_iterator();
        if let Some(mut j) = i.next() {
            assert_eq!(j.next().unwrap(), (0, 0));
            assert_eq!(j.next().unwrap(), (0, 1));
            assert_eq!(j.next().unwrap(), (0, 2));
            assert_eq!(j.next().unwrap(), (1, 0));
            assert_eq!(j.next().unwrap(), (1, 1));
            assert_eq!(j.next().unwrap(), (1, 2));
            assert_eq!(j.next().unwrap(), (2, 0));
            assert_eq!(j.next().unwrap(), (2, 1));
            assert_eq!(j.next().unwrap(), (2, 2));
            if let Some(last) = j.next() {
                panic!("Expected the end of a row, found {:?}", last);
            }
            if let Some(mut col) = i.next() {
                assert_eq!(col.next().unwrap(), (0, 3));
                assert_eq!(col.next().unwrap(), (0, 4));
            } else {
                panic!();
            }
        } else {
            panic!();
        }
    }

    #[test]
    fn test_display1() {
        let sr = StringReader::new(
            "Test Board\n\
                                    094756108\n\
                                    278913564\n\
                                    001042730\n\
                                    030008246\n\
                                    820134957\n\
                                    000060081\n\
                                    513087602\n\
                                    987020013\n\
                                    460391870",
        );
        let mut board = read_board(&mut BufReader::new(sr).lines()).unwrap();
        println!("Board: {}", board);
    }

    #[test]
    fn test_solve1() {
        let sr = StringReader::new(
            "Test Board\n\
        094756108\n\
        278913564\n\
        001042730\n\
        030008246\n\
        820134957\n\
        000060081\n\
        513087602\n\
        987020013\n\
        460391870",
        );
        let mut board = read_board(&mut BufReader::new(sr).lines()).unwrap();
        if let Some(b) = solve(&mut board) {
            println!("Success:\n{:?}", b);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_solve2() {
        let sr = StringReader::new(
            "Test Board\n\
        200080300\n\
        060070084\n\
        030500209\n\
        000105408\n\
        000000000\n\
        402706000\n\
        301007040\n\
        720040060\n\
        004010003",
        );
        let mut board = read_board(&mut BufReader::new(sr).lines()).unwrap();
        println!("emptyies: {}", board.empties.len());
        println!("first empty: {:?}", board.empty_iter().next().unwrap());
        println!(
            "first item: {:?}",
            board
                .empty_iter()
                .next()
                .unwrap()
                .get_possible_items()
                .next()
                .unwrap()
        );
        if let Some(b) = solve(&mut board) {
            println!("Success:\n{:?}", b);
        } else {
            panic!();
        }
    }

    #[test]
    fn test_find1() {
        let sr = StringReader::new(
            "Test Board\n\
    094756108\n\
    278913564\n\
    001042730\n\
    030008246\n\
    820134957\n\
    000060081\n\
    513087602\n\
    987020013\n\
    460391870",
        );
        let board = read_board(&mut BufReader::new(sr).lines()).unwrap();
        if let Some(b) = board.empty_iter().next() {
            println!("Success:\n{:?}", b);
            println!("first: {:?}", b.get_possible_items().next().unwrap())
        } else {
            panic!();
        }
    }
}

#![allow(dead_code)]

use std::ops::{Index, IndexMut};

use crate::{unit::*, pool::*};

#[cfg(test)]
mod test;


pub const COLS: u16 = HALF_COLS * 2;
pub const ROWS: u16 = HALF_ROWS * 2;

pub const CELL_SIZE: u16 = 100;
pub const CELL_RADIUS: f32 = 50.0;
pub const UNIT_RADIUS: f32 = 10.0;

const HALF_COLS: u16 = 10;
const HALF_ROWS: u16 = 6;
const COL_START: u16 = CELL_SIZE * HALF_COLS;
const ROW_START: u16 = CELL_SIZE * HALF_ROWS;

const CHECK_RADIUS: f32 = UNIT_RADIUS + UNIT_RADIUS;
const INV_CELL_SIZE: f32 = 1.0 / (CELL_SIZE as f32);


#[derive(Debug, Clone, Copy)]
pub struct Cell {
    head: u16,
}

impl Default for Cell {
    fn default() -> Self {

        Self {
            head: INVALID,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Cols ([Cell; COLS as usize]);


impl Default for Cols {

    fn default() -> Self {
        
        Self([Cell::default(); COLS as usize])
    }
}

impl Index<u16> for Cols {
    type Output = Cell;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Cols {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}


impl Cols {

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}


#[derive(Debug)]
pub struct Rows([Cols; ROWS as usize]);


impl Default for Rows {

    fn default() -> Self {
        
        Self([Cols::default(); ROWS as usize])
    }
}

impl Index<u16> for Rows {
    type Output = Cols;

    fn index(&self, index: u16) -> &Self::Output {
        
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for Rows {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl Rows {

    pub fn len(&self) -> u16 {
        
        self.0.len() as u16
    }
}


#[derive(Debug)]
pub struct Grid{
    list: Rows,
    pool: Pool,
}


impl Default for Grid {

    fn default() -> Self {
        
        Self{
            list: Rows::default(),
            pool: Pool::default(),
        }
    }
}

impl Index<(u16, u16)> for Grid {
    type Output = Unit;
    
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        let (i, j) = index;
        let head = self.list[i][j].head;
        &self.pool[head]
    }
}

impl IndexMut<(u16, u16)> for Grid {

    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let (i, j) = index;
        let head = self.list[i][j].head;
        &mut self.pool[head]
    }
}



impl Grid {

    pub fn insert(&mut self, id: u32, x:f32, y:f32) {

        assert!(id != INACTIVE);

        let (col, row) = pos2cell(x, y);

        let mut unit = Unit::new(id, x as i16, y as i16);

        if self.list[row][col].head != INVALID {

            unit.next = self.list[row][col].head;
        }

        let index = self.pool.insert(unit);

        self.list[row][col].head = index;

    }


    pub fn remove(&mut self, id: u32, x:f32, y:f32) {

        assert!(id != INACTIVE);

        let (col, row) = pos2cell(x, y);

        let index = self.pop_cell(id, row, col);

        self.pool.erase(index);
    }


    pub fn move_cell(&mut self, id: u32, prev_x: f32, prev_y: f32, x: f32, y: f32) {
        assert!(id != INACTIVE);

        if (prev_x as i16 == x as i16) && (prev_y as i16 == y as i16) {   
            return;
        }

        let (prev_col, prev_row) = pos2cell(prev_x, prev_y);
        let (col, row) = pos2cell(x, y);

        let index: u16;

        if prev_col == col && prev_row == row {

            index = self.find_cell(id, row, col);
        }
        else {

            index = self.pop_cell(id, prev_row, prev_col);

            self.push_cell(index, row, col);
        }

        self.pool[index].x = x as i16;
        self.pool[index].y = y as i16;

    }


    pub fn query(&self, x: f32, y: f32, omit_id: u32) -> Vec<u16> {
        let (min_col, min_row) = pos2cell(x - CHECK_RADIUS, y + CHECK_RADIUS);
        let (max_col, max_row) = pos2cell(x + CHECK_RADIUS, y - CHECK_RADIUS);

        let mut vec: Vec<u16> = Vec::new();
        let mut index: u16;
        for row in min_row..=max_row {
            for col in min_col..=max_col {

                index = self.list[row][col].head;

                while index != INVALID {
                    let unit = self.pool[index];

                    if (unit.id != omit_id) &&
                        (unit.x - x as i16).abs() <= CHECK_RADIUS as i16 && 
                        (unit.y - y as i16).abs() <= CHECK_RADIUS as i16 {
                        vec.push(index);
                    }

                    index = unit.next;
                }
            }
        }

        vec
    }


    pub fn find_cell(&mut self, id: u32, row: u16, col: u16) -> u16 {

        let mut index = self.list[row][col].head;

        loop {

            if index == INVALID {
                return INVALID;
            }

            if self.pool[index].id == id {
                break;
            }

            index = self.pool[index].next;
        }

        index
    }


    pub fn pop_cell(&mut self, id: u32, row: u16, col: u16) -> u16 {

        let mut index = self.list[row][col].head;

        let mut prev = index;
        loop {

            if index == INVALID {
                return INVALID;
            }

            if self.pool[index].id == id {
                break;
            }

            prev = index;
            index = self.pool[index].next;
        }

        if index == self.list[row][col].head {
            self.list[row][col].head = self.pool[index].next;
        }
        else {
            self.pool[prev].next = self.pool[index].next;
        }

        index
    }


    pub fn push_cell(&mut self, index: u16, row: u16, col: u16) {

        if index == INVALID {
            return;
        }

        let head = self.list[row][col].head;
        self.list[row][col].head = index;

        if head == INVALID {
            return;
        }
        
        self.pool[index].next = head;
    }


    pub fn print_units(&self, row: u16, col: u16) {

        let mut index = self.list[row][col].head;

        while index != INVALID {
            let unit = self.pool[index];

            let prev = index;
            index = unit.next;

            if !unit.is_free() {
                print!("{:4}: ", prev);
                unit.print();
            }
        }

    }


    pub fn print_cells(&self) {

        for i in 0..ROWS {
            for j in 0..COLS {
                print!("{:5} ", self.list[i][j].head)
            }
            println!()
        }
    }

    pub fn init_test_data(&mut self) {
        self.insert(100, 54.3, 29.4);
        self.insert(101, 12.3, 23.4);
        self.insert(102, -123.3, 223.4);
        self.insert(103, -323.3, -123.4);
        self.insert(104, 123.3, -123.4);
        self.insert(105, 423.3, 223.4);

        self.insert(106, 24.5, 62.3);
        self.insert(107, 35.5, 35.3);
        self.insert(108, 42.5, 43.3);
        self.insert(109, 21.5, 23.3);

    }

}


fn pos2cell(x:f32, y:f32) -> (u16, u16) {

    let col = ((COL_START as f32 + x) * INV_CELL_SIZE) as i16;
    let row = ((ROW_START as f32 - y) * INV_CELL_SIZE) as i16;

    if col < 0 || col >= COLS as i16{
        return (INVALID, INVALID);
    }

    if row < 0 || row >= ROWS as i16{
        return (INVALID, INVALID);
    }

    (col as u16, row as u16)
}


pub fn main() {
    // test_insert_remove();
    // test_move_cell();
    test_query();
}


fn test_insert_remove() {
    let mut grid = Grid::default();

    grid.init_test_data();

    grid.print_cells();
    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);
    
    grid.remove(107, 35.5, 35.3);
    grid.print_cells();
    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);

    grid.remove(109, 21.5, 23.3);
    grid.print_cells();
    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);
}


fn test_move_cell() {

    let mut grid = Grid::default();

    grid.init_test_data();

    grid.print_cells();
    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);

    grid.move_cell(107, 35.5, 35.3, 143.3, -165.4);
    grid.move_cell(106, 24.5, 62.3, 112.3, -123.4);
    grid.print_cells();
    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);
    println!("{}", grid.list[7][11].head);
    grid.print_units(7, 11);

    grid.move_cell(106, 112.3, -123.4, 24.5, 62.3);
    grid.print_cells();
    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);
    println!("{}", grid.list[7][11].head);
    grid.print_units(7, 11);

}


fn test_query() {

    let mut grid = Grid::default();
    grid.init_test_data();

    grid.insert(201, 38.5, 39.3);
    let vec = grid.query(38.5, 39.3, 201);

    println!("{}", vec.len());

    for index in vec {
        print!("{:4}: ", index);
        grid.pool[index].print();
    }

}
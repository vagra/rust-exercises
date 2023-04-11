#![allow(dead_code)]

use std::{ops::{Index, IndexMut}, mem};

use crate::{unit::*, pool::*, cells::*};

#[cfg(test)]
mod test;

mod helper;


pub const COLS: u16 = HALF_COLS * 2;
pub const ROWS: u16 = HALF_ROWS * 2;

pub const CELL_SIZE: f32 = 100.0;
pub const CELL_RADIUS: f32 = 50.0;
pub const UNIT_RADIUS: f32 = 10.0;

const HALF_COLS: u16 = 10;
const HALF_ROWS: u16 = 6;
const COL_START: f32 = CELL_SIZE * HALF_COLS as f32;
const ROW_START: f32 = CELL_SIZE * HALF_ROWS as f32;
const GRID_WIDTH: f32 = COLS as f32 * CELL_SIZE;
const GRID_HEIGHT: f32 = ROWS as f32 * CELL_SIZE;

const CHECK_RADIUS: f32 = UNIT_RADIUS + UNIT_RADIUS;
const INV_CELL_SIZE: f32 = 1.0 / CELL_SIZE;



#[derive(Debug)]
pub struct Grid{
    cells: Rows,
    pool: Pool,
}


impl Default for Grid {

    fn default() -> Self {
        
        Self{
            cells: Rows::default(),
            pool: Pool::default(),
        }
    }
}

impl Index<(u16, u16)> for Grid {
    type Output = Unit;
    
    fn index(&self, index: (u16, u16)) -> &Self::Output {
        let (i, j) = index;
        let head = self.cells[i][j].head;
        &self.pool[head]
    }
}

impl IndexMut<(u16, u16)> for Grid {

    fn index_mut(&mut self, index: (u16, u16)) -> &mut Self::Output {
        let (i, j) = index;
        let head = self.cells[i][j].head;
        &mut self.pool[head]
    }
}



impl Grid {

    pub fn insert(&mut self, id: u32, x:f32, y:f32) {

        assert!(id != INACTIVE);

        let (col, row) = pos2cell(x, y);

        let mut unit = Unit::new(id, x as i16, y as i16);

        if col == INVALID {
            unit.out = true;
            self.pool.insert(unit);
            return;
        }

        if self.cells[row][col].head != INVALID {

            unit.next = self.cells[row][col].head;
        }

        let index = self.pool.insert(unit);

        self.cells[row][col].head = index;

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

                index = self.cells[row][col].head;

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

    pub fn in_grid(&self, x: f32, y: f32) -> bool {
        let dx = COL_START + x;
        let dy = ROW_START - y;
        let l = dx - UNIT_RADIUS;
        let t = dy + UNIT_RADIUS;
        let r = dx + UNIT_RADIUS;
        let b = dy - UNIT_RADIUS;

        return l >= 0.0 &&
                r <= GRID_WIDTH&&
                b >= 0.0 &&
                t <= GRID_HEIGHT;
    }


    pub fn find_cell(&mut self, id: u32, row: u16, col: u16) -> u16 {

        let mut index = self.cells[row][col].head;

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


    pub fn in_cell(&mut self, id: u32, row: u16, col: u16) -> bool {

        let mut index = self.cells[row][col].head;

        loop {

            if index == INVALID {
                return false;
            }

            if self.pool[index].id == id {
                return true;
            }

            index = self.pool[index].next;
        }
    }


    pub fn pop_cell(&mut self, id: u32, row: u16, col: u16) -> u16 {

        let mut index = self.cells[row][col].head;

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

        if index == self.cells[row][col].head {
            self.cells[row][col].head = self.pool[index].next;
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

        let head = self.cells[row][col].head;
        self.cells[row][col].head = index;

        if head == INVALID {
            return;
        }
        
        self.pool[index].next = head;
    }


    pub fn print_units(&self, row: u16, col: u16) {

        let mut index = self.cells[row][col].head;

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
                print!("{:5} ", self.cells[i][j].head)
            }
            println!()
        }
    }

    pub fn print_pool(&self) {
        self.pool.print();
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


pub fn pos2grid(x:f32, y:f32) -> (f32, f32) {
    return (COL_START + x, ROW_START - y);
}


pub fn pos2cell(x:f32, y:f32) -> (u16, u16) {

    let (dx, dy) = pos2grid(x, y);

    if dx < 0.0 || dx >= GRID_WIDTH||
        dy < 0.0 || dy >= GRID_HEIGHT{
        return (INVALID, INVALID);
    }

    let col = (dx * INV_CELL_SIZE) as u16;
    let row = (dy * INV_CELL_SIZE) as u16;

    if col >= COLS || row >= ROWS {
        return (INVALID, INVALID);
    }

    (col, row)
}


pub fn main() {
    // helper::test_insert_remove();
    // helper::test_move_cell();
    // helper::test_query();
    // helper::test_out_bounds_insert();

    // helper::print_size();
}

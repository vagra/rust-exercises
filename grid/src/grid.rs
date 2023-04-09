#![allow(dead_code)]

use std::ops::{Index, IndexMut};

use crate::{unit::*, pool::*};

#[cfg(test)]
mod test;


pub const COLS: u16 = HALF_COLS * 2;
pub const ROWS: u16 = HALF_ROWS * 2;

pub const CELL_SIZE: u16 = 100;
pub const AGENT_RADIUS: u16 = 10;

pub const OBJ_COUNT: u16 = 100;

const HALF_COLS: u16 = 10;
const HALF_ROWS: u16 = 6;
const COL_START: u16 = CELL_SIZE * HALF_COLS;
const ROW_START: u16 = CELL_SIZE * HALF_ROWS;

const INV_CELL_SIZE: f32 = 1.0 / (CELL_SIZE as f32);


#[derive(Debug, Clone, Copy)]
pub struct Col {
    head: u16,
}

impl Default for Col {
    fn default() -> Self {

        Self {
            head: INVALID,
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Cols ([Col; COLS as usize]);


impl Default for Cols {

    fn default() -> Self {
        
        Self([Col::default(); COLS as usize])
    }
}

impl Index<u16> for Cols {
    type Output = Col;

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

        let mut index = self.list[row][col].head;
        assert!(index != INVALID);

        let mut prev = index;

        while self.pool[index].id != id {
            prev = index;

            index = self.pool[index].next;
            assert!(index != INVALID);
        }
        self.pool[prev].next = self.pool[index].next;

        self.pool.erase(index);
    }

    pub fn print_units(&self, row: u16, col: u16) {

        let mut index = self.list[row][col].head;

        while index != INVALID {
            let unit = self.pool[index];

            let prev = index;
            index = unit.next;

            if !unit.is_free() {
                print!("{}:", prev);
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
    let mut grid = Grid::default();

    grid.insert(101, 12.3, 23.4);
    grid.insert(102, -123.3, 223.4);
    grid.insert(103, -323.3, -123.4);
    grid.insert(104, 123.3, -123.4);
    grid.insert(105, 423.3, 223.4);

    grid.insert(106, 24.5, 62.3);
    grid.insert(107, 35.5, 35.3);
    grid.insert(108, 42.5, 43.3);
    grid.insert(109, 21.5, 23.3);

    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);
    grid.print_cells();

    grid.remove(107, 35.5, 35.3);
    println!("{}", grid.list[5][10].head);
    grid.print_units(5, 10);
    grid.print_cells();
}
#![allow(dead_code)]


use crate::{unit::*, pool::*};

#[cfg(test)]
mod test;


pub const COLS: usize = HALF_COLS * 2;
pub const ROWS: usize = HALF_ROWS * 2;

pub const CELL_SIZE: usize = 100;

pub const OBJ_COUNT: usize = 100;

const HALF_COLS: usize = 10;
const HALF_ROWS: usize = 5;
const COL_OFFSET: f32 = (CELL_SIZE as f32) * (HALF_COLS as f32);
const ROW_OFFSET: f32 = (CELL_SIZE as f32) * (HALF_COLS as f32);

const INV_CELL_SIZE: f32 = 1.0 / (CELL_SIZE as f32);


#[derive(Debug, Clone, Copy)]
struct Col {
    head: i16,
}

impl Col {
    pub fn new() -> Self {

        Self {
            head: -1,
        }
    }
}


#[derive(Debug, Clone, Copy)]
struct Row {
    cols: [Col; COLS],
}

impl Row {
    pub fn new() -> Self {

        Self {
            cols: [Col::new(); COLS],
        }
    }
}


#[derive(Debug)]
pub struct Grid {
    rows: [Row; ROWS],
    pool: Pool,
}


impl Grid {
    pub fn new() -> Self {

        Self {
            rows: [Row::new(); ROWS],

            pool: Pool::default(),
        }
    }

    pub fn insert(&mut self, id: u32, x:f32, y:f32) {

        let obj = Unit::new(id, x as i16, y as i16);

        if let Some((col, row)) = pos2cell(x, y) {
            if self.rows[row].cols[col].head == -1 {

                

            }
        }

    }

}


fn pos2cell(x:f32, y:f32) -> Option<(usize, usize)> {

    let col = ((COL_OFFSET + x) * INV_CELL_SIZE) as i32;
    let row = ((COL_OFFSET - y) * INV_CELL_SIZE) as i32;

    if col < 0 || col >= COLS as i32{
        return None;
    }

    if row < 0 || row >= ROWS as i32{
        return None;
    }

    Some((col as usize, row as usize))
}

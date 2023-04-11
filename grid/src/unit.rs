#![allow(dead_code)]

use std::{ops::{Index, IndexMut}};

use rand::Rng;

use crate::pool::*;


#[cfg(test)]
mod test;



pub const INACTIVE: u32 = u32::MAX;

const X_RANGE: i16 = 1000;
const Y_RANGE: i16 = 600;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit {
    pub id: u32,
    pub x: i16,
    pub y: i16,

    pub next: u16,
    pub next_free: u16,
    pub out: bool,
}

impl Default for Unit {
    fn default() -> Self {

        Self {
            id: INACTIVE,
            x: 0,
            y: 0,
            next: INVALID,
            next_free: INVALID,
            out: false,
        }
    }
}


impl Unit {

    pub fn new(id: u32, x: i16, y: i16) -> Self {

        Self {
            id: id,
            x: x,
            y: y,
            
            ..Default::default()
        }
    }

    pub fn disable(&mut self) {

        self.id = INACTIVE;
    }

    pub fn is_free(&self) -> bool {

        self.id == INACTIVE
    }

    pub fn random() -> Self {

        Self {
            id: rand::thread_rng().gen_range(0..INACTIVE), 
            x: rand::thread_rng().gen_range(-X_RANGE..X_RANGE),
            y: rand::thread_rng().gen_range(-Y_RANGE..Y_RANGE),

            ..Default::default()
        }
    }

    pub fn randxy(&mut self) {
        self.id = rand::thread_rng().gen_range(0..INACTIVE);
        self.x = rand::thread_rng().gen_range(-X_RANGE..X_RANGE);
        self.y = rand::thread_rng().gen_range(-Y_RANGE..Y_RANGE);
    }

    pub fn print(&self) {
        println!("{{id:{}, x:{}, y:{}, next:{}, next_free:{}}}", 
            self.id, self.x, self.y, self.next, self.next_free
        );
    }

}



#[derive(Debug)]
pub struct UnitList(Vec<Unit>);


impl Default for UnitList {
    fn default() -> Self {
        
        Self(Vec::new())
    }
}

impl Index<u16> for UnitList {
    type Output = Unit;

    fn index(&self, index: u16) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl IndexMut<u16> for UnitList {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.0[index as usize]
    }
}

impl Drop for UnitList {

    fn drop(&mut self) {
        self.clear();
    }
}


impl UnitList {

    pub fn push(&mut self, unit:Unit) {

        self.0.push(unit);
    }

    pub fn pop(&mut self) -> Option<Unit> {
        self.0.pop()
    }

    pub fn clear(&mut self) {

        self.0.clear();
    }

    pub fn len(&self) -> u16 {

        self.0.len() as u16
    }

    pub fn is_empty(&self) -> bool {

        self.0.is_empty()
    }

    pub fn print(&self) {
        for (i, unit) in self.0.iter().enumerate() {
            println!("{:3}: ({:3},{:3}) -> {:3}  ", i, unit.x, unit.y, unit.next);
        }

        println!();
    }
}
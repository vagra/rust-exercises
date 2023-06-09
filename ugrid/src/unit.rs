#![allow(dead_code)]

use std::{ops::{Index, IndexMut}};

use rand::Rng;

use crate::{pool::*, ugrid::*};


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
}

impl Default for Unit {
    fn default() -> Self {

        Self {
            id: INACTIVE,
            x: 0,
            y: 0,
            next: INVALID,
            next_free: INVALID,
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

    pub fn is_bump(&self, other:&Unit) -> bool {

        self.is_bump_xy(other.x, other.y)
    }

    pub fn is_bump_xy(&self, x:i16, y:i16) -> bool {

        (self.x - x as i16).abs() <= CHECK_RADIUS_I16 && 
        (self.y - y as i16).abs() <= CHECK_RADIUS_I16
    }

    pub fn is_bump_dxy(&self, dx:i16, dy:i16) -> bool {

        dx.abs() <= CHECK_RADIUS_I16 && 
        dy.abs() <= CHECK_RADIUS_I16
    }

    pub fn bump_front(&self, other:&Unit, dir:u8) -> bool {

        self.bump_front_xy(dir, other.x, other.y)
    }

    pub fn bump_front_xy(&self, dir:u8, x:i16, y:i16) -> bool {
        let dx = self.x - x;
        let dy = self.y - y;
        
        self.is_bump_dxy(dx, dy) &&
        Self::at_front_dxy(dir, dx, dy)
    }

    pub fn at_front(&self, other:&Unit, dir:u8) -> bool {

        self.at_front_xy(dir, other.x, other.y)
    }

    fn at_front_xy(&self, dir:u8, x:i16, y:i16) -> bool {

        let dx = self.x - x;
        let dy = self.y - y;
        
        Self::at_front_dxy(dir, dx, dy)
    }

    fn at_front_dxy(dir:u8, dx:i16, dy:i16) -> bool {

        match dir {
            1 => dx >= 0 && dy <= 0,
            2 => dx >= dy.abs(),
            3 => dx >= 0 && dy >= 0,
            4 => dy >= dx.abs(),
            5 => dx <= 0 && dy >= 0,
            6 => dx <= -dy.abs(),
            7 => dx <= 0 && dy <= 0,
            _ => dy <= -dx.abs(),
        }
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
        println!("{{id:{:3}, x:{:3}, y:{:3}, next:{:5}, next_free:{:5}}}", 
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

    pub fn find(&self, id:u32) -> u16 {
        for (i, unit) in self.0.iter().enumerate() {
            if unit.id == id {
                return i as u16;
            }
        }

        INVALID
    }

    pub fn print(&self) {
        for (i, unit) in self.0.iter().enumerate() {
            print!("{:3}: ", i);
            unit.print();
        }

        println!();
    }
}
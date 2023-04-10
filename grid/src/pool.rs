#![allow(dead_code)]

use std::ops::{Index, IndexMut};

use crate::unit::*;

#[cfg(test)]
mod test;


pub const INVALID: u16 = u16::MAX;
pub const POOL_SIZE: u16 = INVALID - 1;


#[derive(Debug)]
pub struct Pool{ 
    data: UnitList,
    size: u16,
    first_free: u16,
}


impl Default for Pool {
    fn default() -> Self {
        
        Self {
            data: UnitList::default(),
            size: 0,
            first_free: INVALID,
        }
    }
}

impl Index<u16> for Pool {

    type Output = Unit;

    fn index(&self, index: u16) -> &Self::Output {

        &self.data[index]
    }
}

impl IndexMut<u16> for Pool {

    fn index_mut(&mut self, index: u16) -> &mut Self::Output {

        &mut self.data[index]
    }

}

impl Drop for Pool {

    fn drop(&mut self) {
        self.clear();
    }
}


impl Pool {

    pub fn insert(&mut self, unit: Unit) -> u16 {

        if self.size >= INVALID {
            panic!("pool size overflow. max: {}", POOL_SIZE);
        }

        self.size += 1;

        if self.first_free != INVALID {
            let index = self.first_free;
            self.first_free = self.data[index].next_free;

            self.data[index] = unit;

            index
        } else {
            self.data.push(unit);

            self.data.len() - 1
        }
    }

    pub fn erase(&mut self, index: u16) {

        if index == INVALID {
            return;
        }

        if self.data.is_empty() {
            return;
        }

        if self.data[index].is_free() {
            return;
        }

        assert!(self.size > 0);

        self.data[index].disable();
        self.data[index].next_free = self.first_free;

        self.first_free = index;
        self.size -= 1;
    }

    pub fn clear(&mut self) {

        if self.data.is_empty() {
            assert_eq!(self.first_free, INVALID);
            return;
        }

        self.data.clear();
        self.first_free = INVALID;
        self.size = 0;
    }

    pub fn capacity(&self) -> u16 {

        self.data.len()
    }

    pub fn size(&self) -> u16 {

        self.size
    }


    pub fn print(&self) {

        self.data.print();
    }
}


pub fn main() {

    let pool = Pool::default();

    pool.print();

}

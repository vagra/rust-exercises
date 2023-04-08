#![allow(dead_code)]

use crate::unit::*;

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


impl Pool {

    pub fn insert(&mut self, unit: Unit) -> u16 {

        if self.size >= INVALID {
            panic!("pool size overflow. max: {}", POOL_SIZE);
        }

        self.size += 1;

        if self.first_free != INVALID {
            let index = self.first_free;
            self.first_free = self.data[index].next_free;

            self.data[index].copy(&unit);

            index
        } else {
            self.data.push(unit);

            self.data.len() - 1
        }
    }

    pub fn erase(&mut self, index: u16) {

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


    pub fn print(&self) {

        self.data.print();
    }
}


pub fn main() {

    let pool = Pool::default();

    println!("poll len: {}", pool.data.len());

    pool.print();

}



#[cfg(test)]
mod test {
    use crate::{pool::INVALID, unit::NEGATIVE};

    use super::{Pool, Unit};

    
    #[test]
    fn basics() {
        let mut pool = Pool::default();

        pool.insert(Unit::create(1, 10, 10));
        pool.insert(Unit::create(2, 20, 20));
        pool.insert(Unit::create(3, 30, 30));

        assert_eq!(pool.data[2], 
            Unit{id: 3, x:30, y:30, next:INVALID, next_free:INVALID});
        assert_eq!(pool.data[1],
            Unit{id: 2, x:20, y:20, next:INVALID, next_free:INVALID});
        assert_eq!(pool.data[0],
            Unit{id: 1, x:10, y:10, next:INVALID, next_free:INVALID});
        
        assert_eq!(pool.size, 3);
        assert_eq!(pool.data.len(), 3);
        assert_eq!(pool.first_free, INVALID);
    }

    #[test]
    fn erase_insert() {
        let mut pool = Pool::default();

        pool.insert(Unit::create(100, 10, 10));
        pool.insert(Unit::create(101, 20, 20));
        pool.insert(Unit::create(102, 30, 30));
        pool.insert(Unit::create(103, 40, 40));
        pool.insert(Unit::create(104, 50, 50));
        pool.insert(Unit::create(105, 60, 60));
        pool.insert(Unit::create(106, 70, 70));
        pool.insert(Unit::create(107, 80, 80));
        pool.insert(Unit::create(108, 90, 90));


        pool.erase(2);
        assert_eq!(pool.data[2], 
            Unit{id: NEGATIVE, x: 30, y:30, next: INVALID, next_free: INVALID}
        );
        assert_eq!(pool.first_free, 2);
        assert_eq!(pool.size, 8);

        pool.erase(0);
        assert_eq!(pool.data[0], 
            Unit{id: NEGATIVE, x: 10, y:10, next: INVALID, next_free: 2}
        );
        assert_eq!(pool.first_free, 0);
        assert_eq!(pool.size, 7);

        pool.erase(5);
        assert_eq!(pool.data[5], 
            Unit{id: NEGATIVE, x: 60, y:60, next: INVALID, next_free: 0}
        );
        assert_eq!(pool.first_free, 5);
        assert_eq!(pool.size, 6);

        let mut index = pool.insert(Unit::create(110, 10, 10));

        assert_eq!(pool.data[5], 
            Unit{id: 110, x: 10, y:10, next: INVALID, next_free: INVALID}
        );
        assert_eq!(index, 5);
        assert_eq!(pool.first_free, 0);
        assert_eq!(pool.size, 7);
        assert_eq!(pool.data[pool.first_free].next_free, 2);

        index = pool.insert(Unit::create(111, 11, 11));

        assert_eq!(pool.data[0], 
            Unit{id: 111, x: 11, y:11, next: INVALID, next_free: INVALID}
        );
        assert_eq!(index, 0);
        assert_eq!(pool.first_free, 2);
        assert_eq!(pool.size, 8);
        assert_eq!(pool.data[pool.first_free].next_free, INVALID);

        index = pool.insert(Unit::create(112, 12, 12));

        assert_eq!(pool.data[2], 
            Unit{id: 112, x: 12, y:12, next: INVALID, next_free: INVALID}
        );
        assert_eq!(index, 2);
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.size, 9);

        index = pool.insert(Unit::create(115, 15, 15));

        assert_eq!(pool.data[9], 
            Unit{id: 115, x: 15, y:15, next: INVALID, next_free: INVALID}
        );
        assert_eq!(index, 9);
        assert_eq!(pool.first_free, INVALID);
        assert_eq!(pool.size, 10);
        
    }

}
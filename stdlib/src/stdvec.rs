#![allow(unused)]

use std::mem;
use rand::Rng;

const SIZE: i16 = 10;

#[derive(Debug, Default, Clone, Copy)]
struct Point {
    x: i16,
    y: i16,
}


impl Point {

    pub fn new(x: i16, y: i16) -> Self {

        Self {
            x: x,
            y: y,
        }
    }

    pub fn random() -> Self {

        Self {
            x: rand::thread_rng().gen_range(0..SIZE),
            y: rand::thread_rng().gen_range(0..SIZE),
        }
    }
}


struct List (Vec<Point>);

impl List {

    pub fn new() -> Self {

        List(Vec::new())
    }

    pub fn random(size: usize) -> Self {

        List(
            (0..size).map(|_| Point::random()).collect()
        )
    }

    pub fn print(&self) {

        for (i, point) in self.0.iter().enumerate() {
            print!("{}:({},{})  ", i, point.x, point.y);
        }

        println!();
    }
}


pub fn main() {
    let mut list: List = List::random(SIZE as usize);
    list.print();

    let mut point = Point::new(55, 88);
    println!("insert 5:({},{})", point.x, point.y);

    list.0.insert(5, point);
    list.print();

    point = list.0.remove(3);
    println!("remove 3:({},{})", point.x, point.y);
    point = list.0.remove(3);
    println!("remove 3:({},{})", point.x, point.y);
    list.print();

    point = list.0[6];
    println!("access 6:({},{})", point.x, point.y);

    let mut ref_point = &mut list.0[6];
    ref_point.x = 22;
    ref_point.y = 33;
    println!("change 6:({},{})", ref_point.x, ref_point.y);
    list.print();

    list.0.pop();
    list.print();

    
    println!("Vec len: {}", list.0.len());
    println!("Vec capacity: {}", list.0.capacity());

    println!("mem size of Point: {}", mem::size_of::<Point>());
    println!("mem size of point: {}", mem::size_of_val::<Point>(&Point::default()));

    println!("mem size of List: {}", mem::size_of::<List>());
    println!("mem size of list: {}", mem::size_of_val::<List>(&list));

}



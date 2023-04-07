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

    pub fn randxy(&mut self) {
        self.x = rand::thread_rng().gen_range(0..SIZE);
        self.y = rand::thread_rng().gen_range(0..SIZE);
    }

    pub fn print(&self) {
        println!("({},{})", self.x, self.y);
    }
}



struct List ([Point; SIZE as usize]);

impl List {
    pub fn new() -> Self {

        List([Point::default(); SIZE as usize])
    }

    pub fn random() -> Self {

        List(
            [(); SIZE as usize].map(|_| Point::random())
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

    let mut list: List = List::new();
    list.print();

    let mut list: List = List::random();
    list.print();

    println!("array len: {}", list.0.len());

    let mut point = &mut list.0[5];
    point.x = 10;
    point.y = 20;
    println!("change 5:({}, {})", point.x, point.y);
    list.print();

    println!("mem size of Point: {}", mem::size_of::<Point>());
    println!("mem size of point: {}", mem::size_of_val::<Point>(&Point::default()));

    println!("mem size of List: {}", mem::size_of::<List>());
    println!("mem size of list: {}", mem::size_of_val::<List>(&list));

}
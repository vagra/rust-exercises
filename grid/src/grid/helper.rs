use super::*;

pub fn test_insert_remove() {
    let mut grid = Grid::default();

    grid.init_test_data();

    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_units(5, 10);
    
    grid.remove(107, 35.5, 35.3);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_units(5, 10);

    grid.remove(109, 21.5, 23.3);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_units(5, 10);
}


pub fn test_move_cell() {

    let mut grid = Grid::default();

    grid.init_test_data();

    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_units(5, 10);

    grid.move_cell(107, 35.5, 35.3, 143.3, -165.4);
    grid.move_cell(106, 24.5, 62.3, 112.3, -123.4);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_units(5, 10);
    println!("{}", grid.cells[7][11].head);
    grid.print_units(7, 11);

    grid.move_cell(106, 112.3, -123.4, 24.5, 62.3);
    grid.print_cells();
    println!("{}", grid.cells[5][10].head);
    grid.print_units(5, 10);
    println!("{}", grid.cells[7][11].head);
    grid.print_units(7, 11);

}


pub fn test_query() {

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


pub fn test_out_bounds_insert() {
    let mut grid = Grid::default();
    grid.init_test_data();

    grid.print_cells();
    grid.print_pool();

    grid.insert(201, -1000.9, 600.8);
    grid.insert(202, -1050.1, 600.8);
    grid.print_cells();
    grid.print_pool();

    //print!("{:?}", pos2grid(-999.9999, 599.9999));
    print!("{:?}\t\t", pos2cell(-999.9999, 599.9999));
    //print!("{:?}", pos2grid(-1000.0, 600.0));
    print!("{:?}\t", pos2cell(-1000.0, 600.0));
    //print!("{:?}", pos2grid(-1000.0001, 600.0001));
    println!("{:?}", pos2cell(-1000.0001, 600.0001));

    //print!("{:?}", pos2grid(999.9999, 599.9999));
    print!("{:?}\t", pos2cell(999.9999, 599.9999));
    //print!("{:?}", pos2grid(1000.0, 600.0));
    print!("{:?}\t", pos2cell(1000.0, 600.0));
    //print!("{:?}", pos2grid(1000.0001, 600.0001));
    println!("{:?}", pos2cell(1000.0001, 600.0001));

    //print!("{:?}", pos2grid(999.9999, -599.9999));
    print!("{:?}", pos2cell(999.9999, -599.9999));
    //print!("{:?}", pos2grid(1000.0, -600.0));
    print!("{:?}\t", pos2cell(1000.0, -600.0));
    //print!("{:?}", pos2grid(1000.0001, -600.0001));
    println!("{:?}", pos2cell(1000.0001, -600.0001));

    //print!("{:?}", pos2grid(-999.9999, -599.9999));
    print!("{:?}\t", pos2cell(-999.9999, -599.9999));
    //print!("{:?}", pos2grid(-1000.0, -600.0));
    print!("{:?}\t", pos2cell(-1000.0, -600.0));
    //print!("{:?}", pos2grid(-1000.0001, -600.0001));
    println!("{:?}", pos2cell(-1000.0001, -600.0001));

}


pub fn print_size() {
    let mut grid = Grid::default();

    grid.init_test_data();

    println!("size of Unit: {}", mem::size_of::<Unit>());
    println!("size of UnitList: {}", mem::size_of::<UnitList>());
    println!("size of Rows: {}", mem::size_of::<Rows>());
    println!("size of Cols: {}", mem::size_of::<Cols>());
    println!("size of Pool: {}", mem::size_of::<Pool>());
    println!("size of Grid: {}", mem::size_of::<Grid>());

    println!("size of pool: {}", mem::size_of::<Unit>() * POOL_SIZE as usize);
}
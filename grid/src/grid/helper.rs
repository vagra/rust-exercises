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
use super::*;

#[test]
fn default_work() {
    let grid = UGrid::default();

    assert_eq!(grid.pool.size(), 0);
    assert_eq!(grid.cells.len(), ROWS);
    assert_eq!(grid.cells[0].len(), COLS);
}

#[test]
fn insert_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    assert_eq!(grid.cells[3][8].head, 2);
    assert_eq!(grid.cells[3][14].head, 5);
    assert_eq!(grid.cells[5][10].head, 9);
    assert_eq!(grid.cells[7][6].head, 3);
    assert_eq!(grid.cells[7][11].head, 4);

    assert_eq!(grid.pool[9],
        Unit{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(grid.pool[8],
        Unit {id:108, x:42, y:43, next:7, ..Default::default()}
    );
    assert_eq!(grid.pool[7],
        Unit{id:107, x:35, y:35, next:6, ..Default::default()}
    );
    assert_eq!(grid.pool[6],
        Unit {id:106, x:24, y:62, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, ..Default::default()}
    )
}

#[test]
fn index_work() {
    let mut grid = UGrid::default();

    grid.insert(101, 12.3, 98.4);
    grid.insert(102, 23.3, 76.4);
    grid.insert(103, 34.3, 65.4);

    assert_eq!(grid[(5, 10)],
        Unit{id:103, x:34, y:65, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[grid[(5, 10)].next],
        Unit{id:102, x:23, y:76, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[grid.pool[grid[(5, 10)].next].next],
        Unit{id:101, x:12, y:98, ..Default::default()}
    );

}

#[test]
fn remove_work() {

    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.remove(107, 35.5, 35.3);
    grid.remove(109, 21.5, 23.3);

    assert_eq!(grid.cells[3][8].head, 2);
    assert_eq!(grid.cells[3][14].head, 5);
    assert_eq!(grid.cells[5][10].head, 8);
    assert_eq!(grid.cells[7][6].head, 3);
    assert_eq!(grid.cells[7][11].head, 4);

    assert_eq!(grid.pool[8],
        Unit {id:108, x:42, y:43, next:6, ..Default::default()}
    );
    assert_eq!(grid.pool[6],
        Unit {id:106, x:24, y:62, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, ..Default::default()}
    );

}

#[test]
fn move_cell_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.move_cell(107, 35.5, 35.3, 143.3, -165.4);
    grid.move_cell(106, 24.5, 62.3, 112.3, -123.4);
    
    assert_eq!(grid.cells[5][10].head, 9);
    assert_eq!(grid.cells[7][11].head, 6);

    assert_eq!(grid.pool[9],
        Unit{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(grid.pool[8],
        Unit{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(grid.pool[6],
        Unit{id:106, x:112, y:-123, next:7, ..Default::default()}
    );
    assert_eq!(grid.pool[7],
        Unit{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(grid.pool[4],
        Unit{id:104, x:123, y:-123, ..Default::default()}
    );

    grid.move_cell(106, 112.3, -123.4, 24.5, 62.3);

    assert_eq!(grid.cells[5][10].head, 6);
    assert_eq!(grid.cells[7][11].head, 7);

    assert_eq!(grid.pool[6],
        Unit{id:106, x:24, y:62, next:9, ..Default::default()}
    );
    assert_eq!(grid.pool[9],
        Unit{id:109, x:21, y:23, next:8, ..Default::default()}
    );
    assert_eq!(grid.pool[8],
        Unit{id:108, x:42, y:43, next:1, ..Default::default()}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, ..Default::default()}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, ..Default::default()}
    );

    assert_eq!(grid.pool[7],
        Unit{id:107, x:143, y:-165, next:4, ..Default::default()}
    );
    assert_eq!(grid.pool[4],
        Unit{id:104, x:123, y:-123, ..Default::default()}
    );


}


#[test]
fn query_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.insert(201, 38.5, 39.3);
    let vec = grid.query(38.5, 39.3, 201);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec, [9u16, 8u16, 7u16, 0u16]);
}


#[test]
fn dir_query_work() {
    let mut grid = UGrid::default();

    grid.init_test_data();

    grid.insert(201, 25.5, 45.3);
    let mut vec:Vec<u16>;

    vec = grid.dir_query(0, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [7u16]);

    vec = grid.dir_query(1, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = grid.dir_query(2, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec, [8u16, 7u16]);

    vec = grid.dir_query(3, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 0);

    vec = grid.dir_query(4, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = grid.dir_query(5, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 1);
    assert_eq!(vec, [6u16]);

    vec = grid.dir_query(6, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 0);

    vec = grid.dir_query(7, 25.5, 45.3, 201);
    assert_eq!(vec.len(), 0);
}

#[test]
fn in_grid_work() {

    let mut grid = UGrid::default();

    grid.init_test_data();

    assert!(grid.in_grid(-1000.0, 600.0));
    assert!(!grid.in_grid(-1000.001, 600.001));

    assert!(grid.in_grid(999.999, 600.0));
    assert!(!grid.in_grid(1000.0, 600.001));

    assert!(grid.in_grid(999.999, -599.999));
    assert!(!grid.in_grid(1000.0, -600.0));

    assert!(grid.in_grid(-1000.0, -599.999));
    assert!(!grid.in_grid(-1000.001, -600.0));
}


#[test]
fn in_cell_work() {

    let mut grid = UGrid::default();

    grid.init_test_data();

    assert!(grid.in_cell(108, 5, 10));
    assert!(grid.in_cell(106, 5, 10));
    assert!(grid.in_cell(101, 5, 10));


    assert!(grid.in_cell(104, 7, 11));
    assert!(!grid.in_cell(107, 7, 11));
    
}


#[test]
fn pos2cell_work() {
    assert_eq!((0, 0), pos2cell(-2000.0, 1600.0));
    assert_eq!((19, 0), pos2cell(2000.0, 1600.0));
    assert_eq!((19, 11), pos2cell(2000.0, -1600.0));
    assert_eq!((0, 11), pos2cell(-2000.0, -1600.0));

    assert_eq!((0, 3), pos2cell(-2000.0, 300.0));
    assert_eq!((19, 3), pos2cell(2000.0, 300.0));
    assert_eq!((12, 0), pos2cell(200.0, 1600.0));
    assert_eq!((12, 11), pos2cell(200.0, -1600.0));
}

#[test]
fn out_bounds_insert_work() {
    let mut grid = UGrid::default();
    grid.init_test_data();

    grid.insert(201, -2000.0, 1600.0);
    grid.insert(202, 2000.0, 1600.0);
    grid.insert(203, 2000.0, -1600.0);
    grid.insert(204, -2000.0, -1600.0);

    grid.insert(205, -2000.0, 300.0);
    grid.insert(206, 2000.0, 300.0);
    grid.insert(207, 200.0, 1600.0);
    grid.insert(208, 200.0, -1600.0);

    
    assert_eq!(grid.pool[10], 
        Unit{id:  201, x:-2000, y:  1600, ..Default::default()});
    assert_eq!(grid.pool[11], 
        Unit{id:  202, x: 2000, y:  1600, ..Default::default()});
    assert_eq!(grid.pool[12], 
        Unit{id:  203, x: 2000, y: -1600, ..Default::default()});
    assert_eq!(grid.pool[13], 
        Unit{id:  204, x:-2000, y: -1600, ..Default::default()});
    assert_eq!(grid.pool[14], 
        Unit{id:  205, x:-2000, y:   300, ..Default::default()});
    assert_eq!(grid.pool[15], 
        Unit{id:  206, x: 2000, y:   300, ..Default::default()});
    assert_eq!(grid.pool[16], 
        Unit{id:  207, x:  200, y:  1600, ..Default::default()});
    assert_eq!(grid.pool[17], 
        Unit{id:  208, x:  200, y: -1600, ..Default::default()});
    
}


#[test]
fn out_bounds_remove_work() {
    let mut grid = UGrid::default();
    grid.init_test_data();

    grid.insert(205, -2000.0, 300.0);
    grid.insert(206, 2000.0, 300.0);
    grid.insert(207, 200.0, 1600.0);
    grid.insert(208, 200.0, -1600.0);

    grid.remove(205, -2000.0, 300.0);
    grid.remove(208, 200.0, -1600.0);

    assert_eq!(grid.pool[10], 
        Unit{id:  INACTIVE, x:-2000, y:  300,  ..Default::default()});
    assert_eq!(grid.pool[11], 
        Unit{id:  206,      x: 2000, y:  300,  ..Default::default()});
    assert_eq!(grid.pool[12], 
        Unit{id:  207,      x:  200, y: 1600,  ..Default::default()});
    assert_eq!(grid.pool[13], 
        Unit{id:  INACTIVE, x:  200, y:-1600, next:INVALID, next_free:10});

}
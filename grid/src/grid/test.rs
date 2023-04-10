use super::*;

#[test]
fn default_work() {
    let grid = Grid::default();

    assert_eq!(grid.pool.size(), 0);
    assert_eq!(grid.list.len(), ROWS);
    assert_eq!(grid.list[0].len(), COLS);
}

#[test]
fn insert_work() {
    let mut grid = Grid::default();

    grid.init_test_data();

    assert_eq!(grid.list[3][8].head, 2);
    assert_eq!(grid.list[3][14].head, 5);
    assert_eq!(grid.list[5][10].head, 9);
    assert_eq!(grid.list[7][6].head, 3);
    assert_eq!(grid.list[7][11].head, 4);

    assert_eq!(grid.pool[9],
        Unit{id:109, x:21, y:23, next:8, next_free:INVALID}
    );
    assert_eq!(grid.pool[8],
        Unit {id:108, x:42, y:43, next:7, next_free:INVALID}
    );
    assert_eq!(grid.pool[7],
        Unit{id:107, x:35, y:35, next:6, next_free:INVALID}
    );
    assert_eq!(grid.pool[6],
        Unit {id:106, x:24, y:62, next:1, next_free:INVALID}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, next_free:INVALID}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, next:INVALID, next_free:INVALID}
    )
}

#[test]
fn index_work() {
    let mut grid = Grid::default();

    grid.insert(101, 12.3, 98.4);
    grid.insert(102, 23.3, 76.4);
    grid.insert(103, 34.3, 65.4);

    assert_eq!(grid[(5, 10)],
        Unit{id:103, x:34, y:65, next:1, next_free:INVALID}
    );
    assert_eq!(grid.pool[grid[(5, 10)].next],
        Unit{id:102, x:23, y:76, next:0, next_free:INVALID}
    );
    assert_eq!(grid.pool[grid.pool[grid[(5, 10)].next].next],
        Unit{id:101, x:12, y:98, next:INVALID, next_free:INVALID}
    );

}

#[test]
fn remove_work() {

    let mut grid = Grid::default();

    grid.init_test_data();

    grid.remove(107, 35.5, 35.3);
    grid.remove(109, 21.5, 23.3);

    assert_eq!(grid.list[3][8].head, 2);
    assert_eq!(grid.list[3][14].head, 5);
    assert_eq!(grid.list[5][10].head, 8);
    assert_eq!(grid.list[7][6].head, 3);
    assert_eq!(grid.list[7][11].head, 4);

    assert_eq!(grid.pool[8],
        Unit {id:108, x:42, y:43, next:6, next_free:INVALID}
    );
    assert_eq!(grid.pool[6],
        Unit {id:106, x:24, y:62, next:1, next_free:INVALID}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, next_free:INVALID}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, next:INVALID, next_free:INVALID}
    );

}

#[test]
fn move_cell_work() {
    let mut grid = Grid::default();

    grid.init_test_data();

    grid.move_cell(107, 35.5, 35.3, 143.3, -165.4);
    grid.move_cell(106, 24.5, 62.3, 112.3, -123.4);
    
    assert_eq!(grid.list[5][10].head, 9);
    assert_eq!(grid.list[7][11].head, 6);

    assert_eq!(grid.pool[9],
        Unit{id:109, x:21, y:23, next:8, next_free:INVALID}
    );
    assert_eq!(grid.pool[8],
        Unit{id:108, x:42, y:43, next:1, next_free:INVALID}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, next_free:INVALID}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, next:INVALID, next_free:INVALID}
    );

    assert_eq!(grid.pool[6],
        Unit{id:106, x:112, y:-123, next:7, next_free:INVALID}
    );
    assert_eq!(grid.pool[7],
        Unit{id:107, x:143, y:-165, next:4, next_free:INVALID}
    );
    assert_eq!(grid.pool[4],
        Unit{id:104, x:123, y:-123, next:INVALID, next_free:INVALID}
    );

    grid.move_cell(106, 112.3, -123.4, 24.5, 62.3);

    assert_eq!(grid.list[5][10].head, 6);
    assert_eq!(grid.list[7][11].head, 7);

    assert_eq!(grid.pool[6],
        Unit{id:106, x:24, y:62, next:9, next_free:INVALID}
    );
    assert_eq!(grid.pool[9],
        Unit{id:109, x:21, y:23, next:8, next_free:INVALID}
    );
    assert_eq!(grid.pool[8],
        Unit{id:108, x:42, y:43, next:1, next_free:INVALID}
    );
    assert_eq!(grid.pool[1],
        Unit{id:101, x:12, y:23, next:0, next_free:INVALID}
    );
    assert_eq!(grid.pool[0],
        Unit{id:100, x:54, y:29, next:INVALID, next_free:INVALID}
    );

    assert_eq!(grid.pool[7],
        Unit{id:107, x:143, y:-165, next:4, next_free:INVALID}
    );
    assert_eq!(grid.pool[4],
        Unit{id:104, x:123, y:-123, next:INVALID, next_free:INVALID}
    );


}


#[test]
fn query_work() {
    let mut grid = Grid::default();

    grid.init_test_data();

    grid.insert(201, 38.5, 39.3);
    let vec = grid.query(38.5, 39.3, 201);

    assert_eq!(vec.len(), 4);
    assert_eq!(vec, [9u16, 8u16, 7u16, 0u16]);
}
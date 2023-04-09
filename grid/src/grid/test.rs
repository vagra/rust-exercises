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

    grid.insert(101, 12.3, 23.4);
    assert_eq!(grid.list[5][10].head, 0);
    
    grid.insert(102, 34.3, 56.4);
    assert_eq!(grid.list[5][10].head, 1);

    assert_eq!(grid.pool[grid.list[5][10].head].next, 0);
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
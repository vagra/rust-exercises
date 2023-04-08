use super::*;

#[test]
fn default_work() {
    let unit = Unit::default();

    assert_eq!(unit,
        Unit {
            id: INACTIVE,
            x: 0,
            y: 0,
            next: INVALID,
            next_free: INVALID,
        }
    )
}

#[test]
fn new_work() {
    let unit = Unit::new(1, 20, 20);

    assert_eq!(unit,
        Unit {
            id: 1,
            x: 20,
            y: 20,
            next: INVALID,
            next_free: INVALID,
        }
    )
}

#[test]
fn disable_work() {
    let mut unit = Unit::new(1, 10, 10);

    unit.disable();

    assert_eq!(unit,
        Unit {
            id: INACTIVE,
            x: 10,
            y: 10,
            next: INVALID,
            next_free: INVALID,
        }
    )
}

#[test]
fn is_free_work() {
    let mut unit = Unit::new(1, 10, 10);
    unit.disable();

    assert!(unit.is_free());
}
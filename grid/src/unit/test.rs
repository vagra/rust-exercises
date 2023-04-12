use super::*;

#[test]
fn default_work() {
    let unit = Unit::default();

    assert_eq!(unit,
        Unit {
            id: INACTIVE,
            x: 0,
            y: 0,
            ..Default::default()
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
            ..Default::default()
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
            ..Default::default()
        }
    )
}

#[test]
fn is_free_work() {
    let mut unit = Unit::new(1, 10, 10);
    unit.disable();

    assert!(unit.is_free());
}

#[test]
fn at_front_work() {
    let unit = Unit::new(9, 4, -4);
    let unit0 = Unit::new(0, 2, -12);
    let unit1 = Unit::new(1, 8, -12);
    let unit2 = Unit::new(2, 12, -8);
    let unit3 = Unit::new(3, 10, 0);
    let unit4 = Unit::new(4, 2, 6);
    let unit5 = Unit::new(5, -4, 0);
    let unit6 = Unit::new(6, -4, -6);
    let unit7 = Unit::new(7, 0, -8);

    assert!(!unit0.at_front(&unit, 6));
    assert!(unit0.at_front(&unit, 7));
    assert!(unit0.at_front(&unit, 0));
    assert!(!unit0.at_front(&unit, 1));

    assert!(!unit1.at_front(&unit, 7));
    assert!(unit1.at_front(&unit, 0));
    assert!(unit1.at_front(&unit, 1));
    assert!(!unit1.at_front(&unit, 2));

    assert!(!unit2.at_front(&unit, 0));
    assert!(unit2.at_front(&unit, 1));
    assert!(unit2.at_front(&unit, 2));
    assert!(!unit2.at_front(&unit, 3));

    assert!(!unit3.at_front(&unit, 1));
    assert!(unit3.at_front(&unit, 2));
    assert!(unit3.at_front(&unit, 3));
    assert!(!unit3.at_front(&unit, 4));

    assert!(!unit4.at_front(&unit, 3));
    assert!(unit4.at_front(&unit, 4));
    assert!(unit4.at_front(&unit, 5));
    assert!(!unit4.at_front(&unit, 6));

    assert!(!unit5.at_front(&unit, 4));
    assert!(unit5.at_front(&unit, 5));
    assert!(unit5.at_front(&unit, 6));
    assert!(!unit5.at_front(&unit, 7));

    assert!(!unit6.at_front(&unit, 5));
    assert!(unit6.at_front(&unit, 6));
    assert!(unit6.at_front(&unit, 7));
    assert!(!unit6.at_front(&unit, 0));

    assert!(!unit7.at_front(&unit, 5));
    assert!(unit7.at_front(&unit, 6));
    assert!(unit7.at_front(&unit, 7));
    assert!(unit7.at_front(&unit, 0));
    assert!(!unit7.at_front(&unit, 1));
}
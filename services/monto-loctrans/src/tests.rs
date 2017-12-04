use std::fs::File;

use super::one_pos_to_byte;

#[test]
fn optb_1() {
    let mut f = File::open("test_data/test.txt").unwrap();
    assert_eq!(one_pos_to_byte(&mut f, 2, 2).unwrap(), 5);
}

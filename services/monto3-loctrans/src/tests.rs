use std::fs::File;

use super::one_pos_to_byte;

#[test]
fn optb_1() {
    let f = r#"a
    fxfqw"#;
    assert_eq!(one_pos_to_byte(&f, 1, 2).unwrap(), 5);
}

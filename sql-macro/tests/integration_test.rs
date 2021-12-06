use sql_macro::my_macro;

#[test]
fn test_add() {
    let value = my_macro!("hahaha");
    assert_eq!(value, "HAHAHA");
}
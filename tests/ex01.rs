use rust_basics::ex01::*;

#[test]
fn one_sol_is_one_billion_lamports() {
    assert_eq!(sol_to_lamports(1), LAMPORTS_PER_SOL);
}

#[test]
fn lamports_convert_to_whole_sol() {
    assert_eq!(lamports_to_sol(5_500_000_000), 5);
}

#[test]
fn bonus_is_added_to_the_balance() {
    assert_eq!(apply_bonus(10_000, 2_500), 12_500);
}

#[test]
fn value_is_doubled() {
    assert_eq!(double(21), 42);
}

#[test]
fn zero_balance_is_empty() {
    assert!(is_empty(0));
    assert!(!is_empty(1));
}

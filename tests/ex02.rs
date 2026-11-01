use rust_basics::ex02::*;

#[test]
fn base_fee_for_one_signature() {
    assert_eq!(base_fee(1), 5_000);
}

#[test]
fn total_sums_every_amount() {
    assert_eq!(total(&[100, 250, 650]), 1_000);
}

#[test]
fn total_of_an_empty_slice_is_zero() {
    assert_eq!(total(&[]), 0);
}

#[test]
fn count_above_is_strict() {
    assert_eq!(count_above(&[1, 4, 5, 9], 4), 2);
}

#[test]
fn transfer_is_clamped_to_the_balance() {
    assert_eq!(clamp_transfer(100, 40), 40);
    assert_eq!(clamp_transfer(30, 40), 30);
}

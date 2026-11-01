use rust_basics::ex04::*;

#[test]
fn total_reads_a_borrowed_slice() {
    assert_eq!(total(&[10, 20, 30]), 60);
}

#[test]
fn total_of_an_empty_slice_is_zero() {
    assert_eq!(total(&[]), 0);
}

#[test]
fn fee_changes_the_callers_value() {
    let mut amount = 100;
    add_fee(&mut amount, 5);
    assert_eq!(amount, 105);
}

#[test]
fn writable_accounts_are_counted() {
    assert_eq!(count_writable(&[true, false, true]), 2);
}

#[test]
fn note_is_appended_to_the_history() {
    let mut history = Vec::new();
    append_note(&mut history, "deposit");
    assert_eq!(history, vec![String::from("deposit")]);
}

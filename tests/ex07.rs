use rust_basics::ex07::*;

#[test]
fn zero_amount_is_rejected() {
    assert_eq!(
        validate("alice", "bob", 0),
        Err(TransferError::ZeroAmount)
    );
}

#[test]
fn sending_to_yourself_is_rejected() {
    assert_eq!(
        validate("alice", "alice", 10),
        Err(TransferError::SameAccount)
    );
}

#[test]
fn insufficient_funds_reports_both_numbers() {
    assert_eq!(
        transfer(100, "alice", "bob", 150),
        Err(TransferError::InsufficientFunds {
            available: 100,
            required: 150,
        })
    );
}

#[test]
fn successful_transfer_returns_the_remaining_balance() {
    assert_eq!(transfer(100, "alice", "bob", 40), Ok(60));
}

#[test]
fn pay_propagates_errors_and_succeeds() {
    assert_eq!(
        pay(100, "alice", "bob", 0),
        Err(TransferError::ZeroAmount)
    );
    assert_eq!(pay(100, "alice", "bob", 40), Ok(60));
}

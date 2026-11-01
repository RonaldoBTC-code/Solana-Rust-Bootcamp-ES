use rust_basics::ex08::{BankAccount, BankError};

#[test]
fn new_account_is_empty() {
    let account = BankAccount::new("alice");
    assert_eq!(account.owner, "alice");
    assert_eq!(account.lamports, 0);
    assert!(account.history.is_empty());
}

#[test]
fn deposit_updates_balance_and_history() {
    let mut account = BankAccount::new("alice");
    assert_eq!(account.deposit(500), Ok(()));
    assert_eq!(account.lamports, 500);
    assert_eq!(account.history, vec![String::from("deposit 500")]);

    assert_eq!(account.deposit(0), Err(BankError::ZeroAmount));
    assert_eq!(account.lamports, 500);
}

#[test]
fn withdraw_updates_balance_and_history() {
    let mut account = BankAccount::new("alice");
    account.deposit(500).unwrap();
    assert_eq!(account.withdraw(200), Ok(()));
    assert_eq!(account.lamports, 300);
    assert_eq!(
        account.history,
        vec![String::from("deposit 500"), String::from("withdraw 200")]
    );
}

#[test]
fn overdraft_is_rejected_without_changing_state() {
    let mut account = BankAccount::new("alice");
    account.deposit(500).unwrap();

    assert_eq!(
        account.withdraw(600),
        Err(BankError::InsufficientFunds {
            available: 500,
            required: 600,
        })
    );
    assert_eq!(account.lamports, 500);
    assert_eq!(account.history, vec![String::from("deposit 500")]);
}

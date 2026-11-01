use rust_basics::ex05::Account;

#[test]
fn new_account_starts_empty() {
    let account = Account::new(String::from("alice"));
    assert_eq!(account.address, "alice");
    assert_eq!(account.lamports, 0);
    assert_eq!(account.owner, "system");
}

#[test]
fn deposit_adds_lamports() {
    let mut account = Account::new(String::from("alice"));
    account.deposit(5_000);
    assert_eq!(account.lamports, 5_000);
}

#[test]
fn withdraw_removes_lamports() {
    let mut account = Account::new(String::from("alice"));
    account.deposit(1_000);
    assert!(account.withdraw(400));
    assert_eq!(account.lamports, 600);
}

#[test]
fn withdraw_never_overdraws() {
    let mut account = Account::new(String::from("alice"));
    account.deposit(100);
    assert!(!account.withdraw(101));
    assert_eq!(account.lamports, 100);
}

#[test]
fn balance_is_shown_in_whole_sol() {
    let mut account = Account::new(String::from("alice"));
    account.deposit(2_500_000_000);
    assert_eq!(account.balance_in_sol(), 2);
}

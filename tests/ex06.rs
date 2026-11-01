use rust_basics::ex06::*;

fn transfer() -> Instruction {
    Instruction::Transfer {
        from: String::from("alice"),
        to: String::from("bob"),
        lamports: 500,
    }
}

fn create() -> Instruction {
    Instruction::CreateAccount {
        address: String::from("vault"),
    }
}

fn close() -> Instruction {
    Instruction::Close(String::from("vault"))
}

#[test]
fn transfer_kind() {
    assert_eq!(kind(&transfer()), "transfer");
}

#[test]
fn create_and_close_kinds() {
    assert_eq!(kind(&create()), "create");
    assert_eq!(kind(&close()), "close");
}

#[test]
fn only_transfer_moves_lamports() {
    assert_eq!(lamports_moved(&transfer()), 500);
    assert_eq!(lamports_moved(&create()), 0);
    assert_eq!(lamports_moved(&close()), 0);
}

#[test]
fn each_variant_has_a_main_account() {
    assert_eq!(account_touched(&transfer()), "alice");
    assert_eq!(account_touched(&create()), "vault");
    assert_eq!(account_touched(&close()), "vault");
}

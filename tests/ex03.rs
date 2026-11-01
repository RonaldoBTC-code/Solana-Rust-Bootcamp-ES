use rust_basics::ex03::*;

#[test]
fn label_takes_ownership_of_the_name() {
    assert_eq!(make_label(String::from("alice")), "account:alice");
}

#[test]
fn duplicate_returns_an_owned_copy() {
    assert_eq!(duplicate("vault"), "vault");
}

#[test]
fn account_is_returned_with_its_length() {
    assert_eq!(
        take_and_return(String::from("main")),
        (String::from("main"), 4)
    );
}

#[test]
fn vector_is_consumed() {
    let accounts = vec![
        String::from("alice"),
        String::from("bob"),
        String::from("carol"),
    ];
    assert_eq!(consume_all(accounts), 3);
}

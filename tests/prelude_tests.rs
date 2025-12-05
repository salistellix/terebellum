use terebellum::prelude::*;

#[test]
fn w_deref_exposes_inner() {
    let wrapped = W(String::from("hello"));

    assert_eq!(wrapped.len(), 5);
}

#[test]
fn w_deref_mut_allows_mutation() {
    let mut wrapped = W(String::from("hello"));

    wrapped.push_str(" world");

    assert_eq!(wrapped.as_str(), "hello world");
}

#[test]
fn generic_error_matches_from_str() {
    let via_generic = Error::generic("boom");
    let via_from: Error = "boom".into();

    assert_eq!(format!("{via_generic}"), "boom");
    assert_eq!(format!("{via_from}"), "boom");
    assert!(matches!(via_generic, Error::Generic(_)));
    assert_eq!(format!("{via_generic:?}"), format!("{via_from:?}"));
}

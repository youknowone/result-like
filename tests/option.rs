use result_like::OptionLike;

#[test]
fn test_generation() {
    #[derive(OptionLike)]
    enum ValueOption {
        Some(i32),
        None,
    }

    assert_eq!(ValueOption::Some(10).into_option(), Some(10));

    #[derive(OptionLike)]
    enum GenericOption<U> {
        Some(U),
        None,
    }

    assert_eq!(GenericOption::Some("x").into_option(), Some("x"));
}

#[test]
fn test_xo() {
    #[derive(OptionLike)]
    enum XOption<T> {
        Some(T),
        None,
    }

    let xo = XOption::Some(1);

    assert!(xo.unwrap() == 1);

    let op = xo.into_option();

    assert!(op == Some(1));
}

#[test]
fn test_yo() {
    #[derive(OptionLike, is_macro::Is)]
    enum YOption<T> {
        Tone(T),
        Mome,
    }

    let xo = YOption::Tone("s");

    assert!(xo.is_tone());
    assert!(xo.unwrap() == "s");

    let op = xo.into_option();

    assert!(op == Some("s"));
}

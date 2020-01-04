#[test]
fn test_xo() {
    result_like::option_like!(XOption);

    let xo = XOption::Some(1);

    assert!(xo.is_some());
    assert!(xo.unwrap() == 1);

    let op = xo.into_option();

    assert!(op == Some(1));
}

#[test]
fn test_yo() {
    #[derive(Copy, is_macro::Is)]
    enum YOption<T> {
        Tone(T),
        Mome,
    }
    result_like::impl_option_like!(YOption, Tone, Mome);

    let xo = YOption::Tone("s");

    assert!(xo.is_tone());
    assert!(xo.unwrap() == "s");

    let op = xo.into_option();

    assert!(op == Some("s"));
}

#[test]
fn test_pub() {
    result_like::option_like!(pub Pub);
    result_like::option_like!(pub(crate) PubCrate);
}

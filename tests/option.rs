use result_like::OptionLike;

OptionLike!(XOption);
OptionLike!(YOption, Tone, Mome);

#[test]
fn test_xo() {
    let xo = XOption::Some(1);

    assert!(xo.is_some());
    assert!(xo.unwrap() == 1);

    let op = xo.into_option();

    assert!(op == Some(1));
}

#[test]
fn test_yo() {
    let xo = YOption::Tone("s");

    assert!(xo.is_tone());
    assert!(xo.unwrap() == "s");

    let op = xo.into_option();

    assert!(op == Some("s"));
}

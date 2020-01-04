use result_like::ResultLike;

ResultLike!(XResult);
ResultLike!(YResult, Okay, Errr);

#[test]
fn test_xo() {
    let xo: XResult<u32, ()> = XResult::Ok(1);

    assert!(xo.is_ok());
    assert!(xo.unwrap() == 1);

    let op = xo.into_result();

    assert!(op == Ok(1));
}

#[test]
fn test_yo() {
    let xo: YResult<&str, ()> = YResult::Okay("s");

    assert!(xo.is_okay());
    assert!(xo.unwrap() == "s");

    let op = xo.into_result();

    assert!(op == Ok("s"));
}

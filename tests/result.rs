result_like::result_like!(XResult);
result_like::result_like!(YResult, Okay, Errr);

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

#[test]
fn test_pub() {
    result_like::result_like!(pub Pub);
    result_like::result_like!(pub(crate) PubCrate);
}

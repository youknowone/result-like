use result_like::ResultLike;

#[test]
fn test_simple() {
    #[derive(ResultLike)]
    enum XResult {
        Ok(u32),
        Err(()),
    }

    let xo = XResult::Ok(1);

    assert_eq!(xo.unwrap(), 1);

    let op = xo.into_result();

    assert_eq!(op, Ok(1));

    let xo = xo.map_or(0, |v| v * 2);
    assert_eq!(xo, 2);

    assert_eq!(XResult::Err(()).as_result(), Err(&()));
}

#[test]
fn test_generic() {
    #[derive(ResultLike)]
    enum YResult<T, E> {
        Okay(T),
        Errr(E),
    }
    let mut xo: YResult<&str, ()> = YResult::Okay("s");

    assert_eq!(xo.unwrap(), "s");

    let op = xo.into_result();

    assert_eq!(op, Ok("s"));

    xo = xo.and_then(|_| YResult::Errr(()));
    assert_eq!(xo.into_result(), Err(()));
}

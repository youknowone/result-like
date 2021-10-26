use result_like::ResultLike;

#[test]
fn test_x() {
    #[derive(Clone)]
    struct A;
}

#[test]
fn test_simple() {
    #[derive(ResultLike, Clone, Copy)]
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

#[allow(dead_code)]
#[test]
fn test_gen() {
    #[derive(ResultLike)]
    enum ResultA<T> {
        Ok(T),
        Err(()),
    }

    assert_eq!(ResultA::Ok(10).map(|i| i * 2).unwrap(), 20);

    #[derive(ResultLike)]
    enum ResultB<E> {
        Ok(i32),
        Err(E),
    }

    assert_eq!(ResultB::Err(10).or::<()>(ResultB::Ok(10)).unwrap(), 10);

    #[derive(Debug)]
    struct Elem;

    #[derive(ResultLike)]
    enum ResultC<E> {
        Ok(Elem), // test no-copy
        Err(E),
    }
}

#[test]
fn test_result() {
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

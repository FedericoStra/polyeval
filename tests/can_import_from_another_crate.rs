#[test]
fn test_horner() {
    use polyeval::horner;

    let x = 7;

    assert_eq!(horner(x, &[]), 0);
    assert_eq!(horner(x, &[0]), 0);
    assert_eq!(horner(x, &[2, 3, 4]), 2 + x * (3 + x * 4));
}

#[test]
fn test_horner_array() {
    use polyeval::horner_array;

    let x = 7;

    assert_eq!(horner_array(x, &[]), 0);
    assert_eq!(horner_array(x, &[0]), 0);
    assert_eq!(horner_array(x, &[2, 3, 4]), 2 + x * (3 + x * 4));
    assert_eq!(horner_array(x, &[2, 3, 4]), 2 + x * (3 + x * 4));
    assert_eq!(horner_array::<_, 3>(x, &[2, 3, 4]), 2 + x * (3 + x * 4));
    assert_eq!(horner_array::<i32, 3>(x, &[2, 3, 4]), 2 + x * (3 + x * 4));
}

#[test]
fn test_macro_horner() {
    use polyeval::horner;

    let x = 7;

    assert_eq!(horner!(x; [0]), 0);
    assert_eq!(horner!(x; [0,]), 0);
    assert_eq!(horner!(x; 0), 0);
    assert_eq!(horner!(x; 0,), 0);

    assert_eq!(horner!(x; 2, 3, 4), 2 + x * (3 + x * 4));
    assert_eq!(horner!(x; 2, 3, 4,), 2 + x * (3 + x * 4));

    assert_eq!(horner!(x; [2, 3, 4]), 2 + x * (3 + x * 4));
    assert_eq!(horner!(x; [2, 3, 4,]), 2 + x * (3 + x * 4));
}

#[test]
fn test_macro_horner_fma() {
    use polyeval::horner_fma;

    let x = 7;

    assert_eq!(horner_fma!(x; [0]), 0);
    assert_eq!(horner_fma!(x; [0,]), 0);
    assert_eq!(horner_fma!(x; 0), 0);
    assert_eq!(horner_fma!(x; 0,), 0);

    assert_eq!(horner_fma!(x; 2, 3, 4), 2 + x * (3 + x * 4));
    assert_eq!(horner_fma!(x; 2, 3, 4,), 2 + x * (3 + x * 4));

    assert_eq!(horner_fma!(x; [2, 3, 4]), 2 + x * (3 + x * 4));
    assert_eq!(horner_fma!(x; [2, 3, 4,]), 2 + x * (3 + x * 4));
}

#[test]
fn test_macro_estrin() {
    use polyeval::estrin;

    let x = 7;

    assert_eq!(estrin!(x; [0]), 0);
    assert_eq!(estrin!(x; [0,]), 0);
    assert_eq!(estrin!(x; 0), 0);
    assert_eq!(estrin!(x; 0,), 0);

    assert_eq!(estrin!(x; 2, 3, 4), 2 + x * (3 + x * 4));
    assert_eq!(estrin!(x; 2, 3, 4,), 2 + x * (3 + x * 4));

    assert_eq!(estrin!(x; [2, 3, 4]), 2 + x * (3 + x * 4));
    assert_eq!(estrin!(x; [2, 3, 4,]), 2 + x * (3 + x * 4));
}

#[test]
fn test_macro_estrin_fma() {
    use polyeval::estrin_fma;

    let x = 7;

    assert_eq!(estrin_fma!(x; [0]), 0);
    assert_eq!(estrin_fma!(x; [0,]), 0);
    assert_eq!(estrin_fma!(x; 0), 0);
    assert_eq!(estrin_fma!(x; 0,), 0);

    assert_eq!(estrin_fma!(x; 2, 3, 4), 2 + x * (3 + x * 4));
    assert_eq!(estrin_fma!(x; 2, 3, 4,), 2 + x * (3 + x * 4));

    assert_eq!(estrin_fma!(x; [2, 3, 4]), 2 + x * (3 + x * 4));
    assert_eq!(estrin_fma!(x; [2, 3, 4,]), 2 + x * (3 + x * 4));
}

#[test]
fn horner() {
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

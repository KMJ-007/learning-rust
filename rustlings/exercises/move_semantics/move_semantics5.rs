// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


// so basically when y is initialized the borrow checker gives the mut reference of x to the y, but then z also comes and it can't give the reference because it already given to the y, so i move the y's operation before the z's initialization, compiler will do it's job

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}

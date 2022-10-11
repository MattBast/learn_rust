// confusingly this is imported a module called tests (because this
// module was built with the `cargo new tests` command). Usually the
// module to be tested would have a more functional name. If we were
// testing a module called `adder` for instance we'd use `use adder`
// instead.
use tests;

// integration tests are still decorated with `#[test]`
#[test]
fn it_adds_two() {
    // and still use assert statements like 
    assert_eq!(4, tests::add_two(2));
}
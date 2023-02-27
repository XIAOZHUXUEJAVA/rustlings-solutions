// move_semantics4.rs
// Refactor this code so that instead of passing `vec0` into the `fill_vec` function,
// the Vector gets created in the function itself and passed back to the main
// function.
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand for a hint.


fn main() {
    // 1. if we want to use vec0, we should give vec0 an explicit type
    let vec0: Vec<i32> = Vec::new();

    // 2. or use vec0.push(), just perform the vec0 to let compiler infer the type of vec0

    // let mut vec0 = Vec::new();
    // vec0.push(1);
    // let mut vec0 = vec![1, 2, 3];
    

    // 3. and delete it is ok, because in the problem we don't need it!

    let mut vec1 = fill_vec();

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = vec![];

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

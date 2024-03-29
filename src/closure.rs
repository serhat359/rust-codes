// A function which takes a closure as an argument and calls it.
fn apply<F>(f: F) where F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.
    f()
}

// A function which takes a closure and returns an `i32`.
fn apply_to<F>(f: F, i: i32) -> i32 where F: Fn(i32) -> i32 {
    f(i)
}

pub fn test() {
    use std::mem;

    let greeting = "hello";
    // A non-copy type.
    let mut farewell = "goodbye".to_string();

    // Capture 2 variables: `greeting` by reference and
    // `farewell` by value.
    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}.", greeting);

        // Mutation forces `farewell` to be captured by
        // mutable reference. Now requires `FnMut`.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep. zzzzz");

        // Manually calling drop forces `farewell` to
        // be captured by value. Now requires `FnOnce`.
        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    let double_fn = |x| 2 * x;

    println!("3 doubled: {}", apply_to(double_fn, 3));
}
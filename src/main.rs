use std::ops::Add;

fn id<T>(input: T) -> T {
    return input;
}

fn add_five<T: Add<i32, Output = T>>(input: T) -> T {
    return input + 5;
}

fn compose<T: 'static, T2: 'static, T3: 'static>(
    f: fn(T) -> T2,
    g: fn(T2) -> T3,
) -> Box<dyn Fn(T) -> T3> {
    return Box::new(move |x| g(f(x)));
}

fn test_id() {
    let input = 4;

    let first = add_five(input);
    let second = id(add_five(input));
    let third = add_five(id(input));

    let last = id(input);

    println!("id(4): {last}");
    println!("add_five(4): {first}");
    println!("id(add_five(4)): {second}");
    println!("add_five(id(4)): {third}");
}

fn test_compose() {
    let f = |x| x + 1;
    let g = |y| y - 3;

    let my_var = 5;
    let fresult = f(my_var);
    let gresult = g(my_var);
    let fgresult = f(g(my_var));
    let gfresult = g(f(my_var));

    let fg = compose(f, g);
    let gf = compose(g, f);

    let fgcomposed = fg(my_var);
    let gfcomposed = gf(my_var);

    let idbefore = fg(id(my_var));
    let idafter = id(fg(my_var));

    println!("MyVar: {my_var}");
    println!("f(myVar): {fresult}");
    println!("g(myVar): {gresult}");
    println!("fg no compose: {fgresult}");
    println!("gf no compose: {gfresult}");

    println!("fg composed: {fgcomposed}");
    println!("gf composed: {gfcomposed}");
    println!("id before compose: {idbefore}");
    println!("id after compose: {idafter}");
}

fn main() {
    test_id();

    test_compose();
}

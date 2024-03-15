fn functions_with_parameters(x: i32, unit: char) {
    print!("x: {} unit {}", x, unit)
}

fn beautiful_expression() -> i32 {
    let y = {
        let x = 10;
        x * 10
    };
    return y;
}

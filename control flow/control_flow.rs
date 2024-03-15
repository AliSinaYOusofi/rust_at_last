fn new_control_flow_statement() {
    let condition:bool = true;
    let number: i32 = if condition {5} else {10};
    print!("{}", number);
}
fn code_repetition() {
    let mut number = 0;
    loop {
        number += 1;

        if number == 10 { break }
    }
    print!("Number is now {}", number);
}

fn loop_inside_loop() {

    let mut outer_counter: i32 = 0;
    let mut inner_counter: i32 = 10;

    'outer_loop: loop {
        outer_counter += 1;

        print!(" [+]: The outer counter: {outer_counter} \n");
        'inner_loop: loop {

            inner_counter -= 1;
            outer_counter *= 2;

            if inner_counter == 0 {
                print!("The inner counter = {inner_counter} \n");
                print!("The outer counter = {outer_counter} \n");
                break 'outer_loop;
            }

            if outer_counter % 2 == 0 {
                break 'inner_loop
            }

            print!(" [-]: The inner loop_counter: {inner_counter} \n")
        }
    }
    print!("inner_counter = {inner_counter} \n");
    print!("outer_count = {outer_counter} \n");
}

fn while_loop() -> i32 {
    let mut number: i32 = 0;

    while number <= 10 {
        number+= 1;
    }
    print!("Returning number: {number}");
    return number
}

fn looping_with_for() {
    let numbers = [34, 23, 45, 34];
    for element in numbers {
        print!("The element is : {element} \n")
    }
}

fn looping_with_range() {
    for number in (1..5).rev() {
        print!("The number is {number} \n")
    }
}
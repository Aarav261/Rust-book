pub fn loops() {
    let mut count = 0;
    'counting_up: loop { // This is a label for the outer loop
        println!("count = {}", count);
        let mut remaining = 10;

        'inner_loop: loop {// This is the inner loop
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {}", count);
}
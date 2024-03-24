fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    println!("================ Loop ================");

    let mut count = 3;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 2 {
                println!("count down to 2");
                break;
            }
            remaining -= 1;
            if remaining == 5 {
                println!("count down to 5");
                break 'counting_up;
            }
        }
    }
    println!("End count = {}", count);

    println!("================ While loop ================");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!("================ For loop ================");
    
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }

    println!("================ For loop with range ================");
    for number in (1..=4).rev() {
        println!("{}!", number);
    }

}

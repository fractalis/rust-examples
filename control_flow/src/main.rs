fn main() {

    let mut number = 3;

    if number < 5 {
        println!("Number is less than 5");
    } else if number == 5 {
        println!("Number is equal to 5");
    } else {
        println!("Number is greater than 5");
    }

    loop {
        number = number - 1;
        if number == 0 {
            break;
        }
        println!("Loop...{}", number);
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    number = 3;

    while number != 0 {
        println!("Number: {}", number);
        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("Elem: {}", element);
    }

    for number in (1..4).rev() {
        println!("Number: {}", number);
    }
}

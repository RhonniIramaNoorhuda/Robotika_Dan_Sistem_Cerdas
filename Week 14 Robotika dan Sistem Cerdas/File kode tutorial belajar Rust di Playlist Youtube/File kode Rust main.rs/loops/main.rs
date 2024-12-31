fn main() {
    let mut counter = 1;
    let loop_stop = loop {
        counter *= 4;
        if counter > 100 {
            break counter;
        }
    };

    println!("Break the loop at counter = {}", loop_stop);

    let mut num = 0;
    while num < 10 {
        println!("Hello there!");
        num = num + 1;
    }

    let shopping_list = ["milk", "cheese", "bread", "apples"];

    for item in shopping_list.iter() {
        println!("The next item on my shopping is {}", item);
    }

    for number in 0..10 {
        println!("Number {}", number);
    }
}

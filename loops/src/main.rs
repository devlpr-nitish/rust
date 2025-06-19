



fn main() {

    let mut counter = 1;


    let res = loop {
        counter  += 1;

        if counter == 12 {
            break counter;
        }
    };

    println!("{}", res);



    // while loop

    let mut counter = 12;

    while counter > 0 {
        println!("{}", counter);

        counter -= 1;
    }


    
    // For loop

    let arr = [12,534,23,3232,43];

    for a in arr {
        println!("{}", a);
    }

    for num in 1..23 {
        println!("{}", num);
    }


}

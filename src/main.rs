use std::io::{self, Write};

fn main()
{
    let mut number1 = 1000;
    const NUMBER2: i128 = 7;

    while NUMBER2 <= number1
    {
        print!("{} - {} = ", number1, NUMBER2);
        io::stdout().flush().unwrap();

        number1 -= NUMBER2;

        println!("{}", number1);
    }
}


enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn value_in_cents(&self) -> i32 {
        // a match will return the last value of the branch whose
        // condition is true.
        return match &self {
            // so if the coin is a Penny it will return 1
            &Self::Penny => 1,
            // branches can do anything that fits inside an expression 
            // (curly brackets) so long as that expression returns a 
            // value
            &Self::Nickel => {
                println!("This coin is a Nickel!");
                5
            },
            // they can also call functions
            &Self::Dime => self.value_of_dime(),
            &Self::Quarter => 25,
            // it's best practice to put a catch all condition at the end
            // of the match statement so something always happens. In fact
            // the compiler will sometimes enforce it
            _ => 0
        }
    }

    fn value_of_dime(&self) -> i32 {
        return 10;
    }
}

fn main() {
    let coin1 = Coin::Penny;
    println!("Value of penny is {}.", coin1.value_in_cents());

    let coin2 = Coin::Nickel;
    println!("Value of penny is {}.", coin2.value_in_cents());

    let coin3 = Coin::Dime;
    println!("Value of penny is {}.", coin3.value_in_cents());

    let coin4 = Coin::Quarter;
    println!("Value of penny is {}.", coin4.value_in_cents());
}

// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - If Mary buys more than 40 apples, each apple only costs 1 rustbuck!
// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.

// Put your function here!
const SINGLE_APPLE_PRICE: i32 = 2;

fn calculate_price_of_apples(apple_count: i32) -> i32 {
    let final_price = if apple_count > 40 {
        apple_count
    } else {
        SINGLE_APPLE_PRICE * apple_count
    };


    return final_price
}

fn main() {
    let result = calculate_price_of_apples(34);

    print!("{result}")
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
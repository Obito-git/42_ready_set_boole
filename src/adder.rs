fn visual_adder(a: u32, b: u32) -> u32 {
    print!("\x1B[2J\x1B[1;1H");

    let mut carry = 0;
    let mut result = 0;
    let mut mask = 1;
    let mut current_pos: usize = 38;

    while mask != 0 {
        let bit_a = a & mask;
        let bit_b = b & mask;

        let sum = bit_a ^ bit_b ^ carry;
        carry = (bit_a & bit_b) | (bit_a & carry) | (bit_b & carry);

        result |= sum;

        println!("a    : {:032b}", a);
        println!("b    : {:032b}", b);
        println!("{}^", " ".repeat(current_pos));
        println!("sum  : {:032b}", sum);
        println!("carry: {:032b}", carry);
        println!("mask : {:032b}", mask);
        println!("{}\n", "_".repeat(70));
        println!("res  : {:032b}", result);
        println!("res  : {:32}", result);

        let _ = std::io::stdin().read_line(&mut String::new());
        print!("\x1B[2J\x1B[1;1H");

        mask <<= 1;
        carry <<= 1;
        current_pos -= 1;
    }

    result
}

fn main() {
    if std::env::args().count() != 3 {
        eprintln!("Usage: adder <x> <y>");
        std::process::exit(1);
    }
    let x = std::env::args().nth(1).unwrap();
    let x = x.parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Expected unsigned integer, got: {x}");
        std::process::exit(1);
    });
    let y = std::env::args().nth(2).unwrap();
    let y = y.parse::<u32>().unwrap_or_else(|_| {
        eprintln!("Expected unsigned integer, got: {y}");
        std::process::exit(1);
    });
    println!("{x} + {y} = {}", visual_adder(x, y));
}

#[cfg(test)]
mod tests {
    use ready_set_boole::adder;

    #[test]
    fn test_adder() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(1, 0), 1);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(42, 42), 84);
        assert_eq!(adder(100000, 1), 100001);
        assert_eq!(
            adder(3000000000, 3000000000),
            3000000000u32.wrapping_add(3000000000)
        );
    }
}

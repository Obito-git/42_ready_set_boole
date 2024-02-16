fn visual_multiplier(a: u32, b: u32) -> u32 {
    print!("\x1B[2J\x1B[1;1H");

    let mut res = 0;
    let mut mask = 1;
    let mut shift = 0;
    let mut current_pos: usize = 38;

    while mask != 0 {
        if (b & mask) != 0 {
            res += a << shift;
        }

        println!("a    : {:032b}", a);
        println!("b    : {:032b}", b);
        println!("{}^", " ".repeat(current_pos));
        println!("mask : {:032b}", mask);
        println!("shift: {:032b}", shift);
        println!("res  : {:032b}", res);
        println!("res  : {:32}", res);

        let _ = std::io::stdin().read_line(&mut String::new());
        print!("\x1B[2J\x1B[1;1H");

        mask <<= 1;
        shift += 1;
        current_pos -= 1;
    }
    res
}

fn main() {
    if std::env::args().count() != 3 {
        eprintln!("Usage: multiplier <x> <y>");
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
    println!("{x} * {y} = {}", visual_multiplier(x, y));
}

#[cfg(test)]
mod tests {
    use ready_set_boole::multiplier;

    #[test]
    fn test_multiplier() {
        assert_eq!(multiplier(9, 5), 45);
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(10, 10), 100);
        assert_eq!(multiplier(100, 100), 10000);
        assert_eq!(
            multiplier(1000000, 1000000),
            1000000u32.wrapping_mul(1000000u32)
        );
    }
}

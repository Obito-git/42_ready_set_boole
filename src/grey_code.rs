use ready_set_boole::gray_code;

fn main() {
    if std::env::args().count() < 2 {
        eprintln!("Usage: gray_code <n>...<n>");
        std::process::exit(1);
    }
    std::env::args().skip(1).for_each(|arg| {
        let n = arg.parse::<u32>().unwrap_or_else(|_| {
            eprintln!("Expected unsigned integer, got: {arg}");
            std::process::exit(1);
        });
        println!("{arg} -> {}", gray_code(n));
    });
}

#[cfg(test)]
mod tests {
    use ready_set_boole::gray_code;

    #[test]
    fn test_gray_code() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
    }
}

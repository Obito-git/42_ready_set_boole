pub fn adder(a: u32, b: u32) -> u32 {
    let mut carry = 0;
    let mut result = 0;
    let mut mask = 1;

    while mask != 0 {
        let bit_a = a & mask;
        let bit_b = b & mask;

        let sum = bit_a ^ bit_b ^ carry;
        carry = (bit_a & bit_b) | (bit_a & carry) | (bit_b & carry);

        result |= sum;

        mask <<= 1;
        carry <<= 1;
    }

    result
}
pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut res = 0;
    let mut mask = 1;
    let mut shift = 0;

    while mask != 0 {
        if (b & mask) != 0 {
            res = adder(res, a << shift);
        }
        mask <<= 1;
        shift += 1;
    }
    res
}

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

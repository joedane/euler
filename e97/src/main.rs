
fn double_digit_bcd(input: u64, n: usize, mut carry: bool) -> (u64, bool) {

    let mut digit = (input >> 4*n) & 0xF;
    digit = (digit * 2) + if carry { 1 } else { 0};
    if digit > 9 {
        carry = true;
        digit = digit % 10;
    } else {
        carry = false;
    }
    (digit << 4*n, carry)
}

fn main() {

    let mut n:u64 = 0x28433;
    for _i in 0..7830457 {
        let mut tmp:u64 = 0;
        let mut carry = false;

        for j in 0..16 {
            let mut t = double_digit_bcd(n, j, carry);
            tmp |= t.0;
            carry = t.1;
        }

        n = tmp;

       // println!("{:08x}", n);
        //if i % 10000 == 0 { println!("{}", i) }
    }

    println!("{:08x}", n);

}

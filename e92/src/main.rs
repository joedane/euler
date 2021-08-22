fn next_in_chain(n: u32) -> u32 {
    let mut next = 0u32;
    let mut this_n = n;

    while this_n > 0 {
        let d = this_n % 10;
        next += d * d;
        this_n /= 10;
    }
    next
}

fn main() {
    let mut count: u32 = 0;

    for n in 2..10000001 {
        let mut next = next_in_chain(n);
        while next != 89 && next != 1 {
            next = next_in_chain(next);
        }
        if next == 89 {
            count += 1;
        }
    }
    println!("{}", count);
}

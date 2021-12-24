use std::fs;
use std::str::FromStr;

fn main() -> Result<(), std::io::Error> {

    let pairs:Vec<(usize, u32, u32)> = fs::read_to_string("p099_base_exp.txt")?
        .split("\n")
        .map(|line| line.split(",").map(|s| u32::from_str(s).unwrap()).collect::<Vec<_>>())
        .enumerate()
        .map(|(lno, vals)| (lno+1, vals[0], vals[1]))
        .collect();


    println!("{:?}", pairs.into_iter()
        .map(|(lno, base, exp)| (lno, (base as f32).ln() * exp as f32))
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()));

    Ok(())

}

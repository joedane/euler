
fn eval_slow(n: i64) -> i64 {

    return 1 - n + n.pow(2) - n.pow(3) 
    + n.pow(4) - n.pow(5) + n.pow(6) 
    - n.pow(7) + n.pow(8) - n.pow(9) + n.pow(10);

}

fn eval_fast(n: i64) -> i64 {
    let mut val = 1 - n;
    for _ in 0..9 {
        val *= -n;
        val += 1;
    }
    val
}

fn make_sequence() -> impl Fn(i64) -> i64 {
    |n| {
        n.pow(3)
    }
}

fn make_differences<F: Fn(i64) -> i64>(f: F) -> impl Fn(i64) -> i64 {
  let diffs = move |n| {
        f(n+1) - f(n)
  };
  diffs
} 

fn get_diff(data: &mut [[i64; 13]; 13], level: usize, i: usize) -> i64 {
    if data[level][i] > 0 {
        return data[level][i];
    } else {
        let value = get_diff(data, level, i-1) + get_diff(data, level+1, i-1);
        data[level][i] = value;
        return value;
    }
}

fn main() {

    let mut data = [[0i64; 13]; 13];
    let seq = make_sequence();
    let correct_values = (1..=20).map(eval_fast).collect::<Vec<i64>>();
    let mut bad_values = vec![1i64];

    for n in 1..11 {
        for i in 0..=n {
            data[0][i] = eval_fast((i as i64) + 1);
        }
        for diffidx in 1..=n {
            for j in 0..=(n-diffidx) {
                data[diffidx][j] = data[diffidx-1][j+1] - data[diffidx-1][j]
            }
            if diffidx == n {
                for j in 1..data[diffidx].len() {
                    data[diffidx][j] = data[diffidx][j-1];
                }
            }
        }

        for i in 0..data[0].len() {
            if data[0][i] > 0 {
                continue;
            }
            data[0][i] = data[0][i-1] + get_diff(&mut data, 1, i-1)
        }

        if n == 3 {
            println!("{:#?}", data);
        }
        for i in 0..data[0].len() {
            if data[0][i] != correct_values[i] {
                bad_values.push(data[0][i]);
                break;
            }
        }

        for i in 0..data.len() {
            for j in 0..data[i].len() {
                data[i][j] = 0;
            }
        }
    }
    println!("bad values: {:?}", bad_values);
    println!("{}", bad_values.into_iter().sum::<i64>())
}

mod test {

    use super::*;

    #[test]
    fn test_eval() {
        assert!((0..10).all(|v| eval_slow(v) == eval_fast(v)));
    }
}
use aoc::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lines = read_lines(args.last().unwrap()).unwrap();

    let mut input = Vec::<usize>::new();

    for line in lines {
        if let Ok(line) = line {
            input.push(line.parse().unwrap());
        }
    }

    let preamble_len = 25_usize;
    let ans = first_wrong(preamble_len, &input);
    println!("{:?}", ans);

    let ans = find_range(ans.unwrap(), &input);
    println!(
        "{:?}",
        input[ans.0..ans.1].iter().min().unwrap() + input[ans.0..ans.1].iter().max().unwrap()
    );
}

fn first_wrong(preamble: usize, input: &[usize]) -> Option<usize> {
    let mut ans = None;
    for (i, n) in input.iter().enumerate() {
        if i < preamble {
            continue;
        }

        if !in_basis(&input[(i - preamble)..i], *n) {
            ans = Some(*n);
            break;
        }
    }
    ans
}

fn in_basis(basis: &[usize], query: usize) -> bool {
    let mut pass = false;
    let n = basis.len();
    for i in 0..n {
        for j in (i + 1)..n {
            pass |= query == basis[i] + basis[j];
        }
    }
    pass
}

fn find_range(query: usize, input: &[usize]) -> (usize, usize) {
    let n = input.len();
    let mut ans = (0, 0);
    for i in 0..n {
        for j in (i + 1)..n {
            let range_sum = input[i..(j + 1)].iter().sum();
            if query == range_sum {
                ans = (i, j + 1)
            }
            if range_sum > query {
                break;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_basis() {
        let basis = [1, 2, 3];

        assert!(!in_basis(&basis, 2));
        assert!(!in_basis(&basis, 1));
        assert!(in_basis(&basis, 3));
        assert!(in_basis(&basis, 5));
        assert!(!in_basis(&basis, 7));
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Constraints {
    NotEven,
    ArrNotLongEnoughOrToBig,
    ValueNegative,
}
struct Leet {}

impl Leet {
    pub fn min_set_size(arr: Vec<i32>) -> Result<i32, Constraints> {
        if !((2 <= arr.len()) && (arr.len() as u32 <= 10_u32.pow(5_u32))) {
            return Err(Constraints::ArrNotLongEnoughOrToBig);
        }
        if arr.len() % 2 != 0 {
            return Err(Constraints::NotEven);
        }

        let mut counter = [0; 50];
        let mut l = arr.len();
        let half = l / 2;

        for x in arr {
            if !(1 <= x && x as u32 <= 10_u32.pow(5_u32)) {
                return Err(Constraints::ValueNegative);
            }
            counter[x as usize] += 1
        }
        counter.sort_unstable_by_key(|&x| -x);
        println!("{:?}", counter);

        let mut res = 0;
        for n in counter {
            l -= n as usize;
            res += 1;
            if l <= half {
                break;
            }
        }
        Ok(res)
    }
}

fn main() {
    let valid_case = [
        (vec![3, 3, 3, 3, 5, 5, 5, 2, 2, 7], 2),
        (vec![3, 3, 3, 3, 5, 1, 2, 4], 1),
        (vec![7, 7, 7, 7, 7, 7], 1),
    ];

    let odd = (vec![3, 3, 3, 3, 5, 5, 5, 2, 2], 0);
    let size_to_large = (vec![-3, 3, 3, 3, 5, 5, 5, 2, 2, 7], 2);

    for data in valid_case {
        let result = Leet::min_set_size(data.0);
        println!("{}", result.unwrap());
        assert_eq!(data.1, result.unwrap());
    }

    let result = Leet::min_set_size(odd.0);
    assert_eq!(Err(Constraints::NotEven), result);

    let result = Leet::min_set_size(size_to_large.0);
    assert_eq!(Err(Constraints::ValueNegative), result);
}

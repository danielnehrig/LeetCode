struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        let test = n.count_ones();
        let zero = n.trailing_zeros();
        println!("{:b} {} {}", n, test, zero);
        test == 1 && zero % 2 == 0
    }

    pub fn is_power_of_two(n: i32) -> bool {
        println!("{:b}", n);
        n & (n - 1) == 0
    }

    pub fn is_power_of_three(n: i32) -> bool {
        n != 0 && 3486784401u64 % n as u64 == 0
    }
}

fn main() {
    let payload_two = [(1, true), (2, true), (4, true)];
    let payload_four = [
        (64, true),
        (16, true),
        (4, true),
        (1, true),
        (32, true),
        (8, false),
        (20, false),
        (5, false),
    ];
    for x in payload_two {
        let result = Solution::is_power_of_two(x.0);
        println!("{}", result);
        //assert_eq!(x.1, result);
    }

    // for x in payload_four {
    // let result = Solution::is_power_of_four(x.0);
    // println!("{}", result);
    // assert_eq!(x.1, result);
    // }
    // for x in 0..100 {
    // let result = Solution::is_power_of_four(x);
    // if result {
    // println!("{}, {}", x, result);
    // }
    // }
}

struct Solution {}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let forward = height.iter().cloned().scan(0, |a, b| {
            *a = (*a).max(b);
            Some(*a)
        });
        let backward = height
            .iter()
            .rev()
            .cloned()
            .scan(0, |a, b| {
                *a = (*a).max(b);
                Some(*a)
            })
            .collect::<Vec<_>>()
            .into_iter()
            .rev();

        println!("{:?}", forward);
        println!("{:?}", backward);

        forward
            .zip(backward)
            .map(|(a, b)| a.min(b))
            .zip(height.clone())
            .map(|(w, h)| w - h)
            .sum()
    }
}

fn main() {
    let payload = [
        (vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1], 6),
        (vec![4, 2, 0, 3, 2, 5], 9),
    ];

    for x in payload {
        let result = Solution::trap(x.0);
        println!("{}", result);
        assert_eq!(x.1, result);
    }
}

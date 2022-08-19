use std::collections::HashMap;

struct Leet {}

impl Leet {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut map = HashMap::<i32, i32>::new();
        for x in nums {
            if let Some(data) = map.get(&x) {
                map.insert(x, data + 1);
            } else {
                map.insert(x, 1);
            }
        }

        let result = map.into_iter().find(|a| a.1 >= 2);
        matches!(result, Some(_))
    }
}

fn main() {
    let input = vec![
        vec![1, 2, 3, 1],
        vec![1, 2, 3, 4],
        vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2],
    ];
    for nums in input {
        println!("{}", Leet::contains_duplicate(nums));
    }
}

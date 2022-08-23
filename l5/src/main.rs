struct Solution {}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        fn is_palindrom(s: &[u8]) -> bool {
            s.iter().zip(s.iter().rev()).all(|(l, r)| l == r)
        }

        for x in (1..=s.len()).rev() {
            println!("{}", x);
            match s.as_bytes().windows(x).find(|substr| is_palindrom(substr)) {
                Some(pdr) => return String::from_utf8(pdr.to_vec()).unwrap(),
                None => continue,
            }
        }

        "".to_string()
    }
}

fn main() {
    let payload = [
        (String::from("babad"), String::from("bab")),
        (String::from("ccc"), String::from("ccc")),
        // (String::from("cbbd"), String::from("bb")),
        // (String::from("kztakrekvefgchersuoiuatzlmwynzjhdqqftjcqmntoyckqfawikkdrnfgbwtdpbkymvwoumurjdzygyzsbmwzpcxcdmmpwzmeibligwiiqbecxwyxigikoewwrczkanwwqukszsbjukzumzladrvjefpegyicsgctdvldetuegxwihdtitqrdmygdrsweahfrepdcudvyvrggbkthztxwicyzazjyeztytwiyybqdsczozvtegodacdokczfmwqfmyuixbeeqluqcqwxpyrkpfcdosttzooykpvdykfxulttvvwnzftndvhsvpgrgdzsvfxdtzztdiswgwxzvbpsjlizlfrlgvlnwbjwbujafjaedivvgnbgwcdbzbdbprqrflfhahsvlcekeyqueyxjfetkxpapbeejoxwxlgepmxzowldsmqllpzeymakcshfzkvyykwljeltutdmrhxcbzizihzinywggzjctzasvefcxmhnusdvlderconvaisaetcdldeveeemhugipfzbhrwidcjpfrumshbdofchpgcsbkvaexfmenpsuodatxjavoszcitjewflejjmsuvyuyrkumednsfkbgvbqxfphfqeqozcnabmtedffvzwbgbzbfydiyaevoqtfmzxaujdydtjftapkpdhnbmrylcibzuqqynvnsihmyxdcrfftkuoymzoxpnashaderlosnkxbhamkkxfhwjsyehkmblhppbyspmcwuoguptliashefdklokjpggfiixozsrlwmeksmzdcvipgkwxwynzsvxnqtchgwwadqybkguscfyrbyxudzrxacoplmcqcsmkraimfwbauvytkxdnglwfuvehpxd"), String::from("dtzztd")),
    ];

    for x in payload {
        let result = Solution::longest_palindrome(x.0);
        println!("{}", result);
        assert_eq!(x.1, result);
    }
}

fn count_vowels_r(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    else {
        let _c = s.to_lowercase().chars().next().unwrap();
        let rest_s = &s[_c.len_utf8()..];
        let counter = count_vowels_r(rest_s);
        if _c == 'a' || _c == 'e' || _c == 'i' || _c == 'o' || _c == 'u' {
            return counter + 1;
        }
        else {
            return counter;
        }
    }
 }
 

 fn main() {
     let s = "abE cdIoU";
     println!("{}", count_vowels_r(s));
 }

 #[test]
 fn test_vowels_count1() {
    assert_eq!(count_vowels_r(""), 0);
    assert_eq!(count_vowels_r("abEcd"), 2);
    assert_eq!(count_vowels_r("ab12Exey5 7x8U3y5z"), 4);
}
 
 
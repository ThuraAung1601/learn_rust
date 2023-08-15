fn count_vowels(s: &str) -> usize {
    let mut counter = 0;
    for _c in s.to_lowercase().chars() {
         if _c == 'a' || _c == 'e' || _c == 'i' || _c == 'o' || _c == 'u' {
             counter += 1;
         }
         else {
             continue;
         }
    }
    return counter;
 }
 
 
 fn main() {
     let s = "abE cdIoU";
     println!("{}", count_vowels(s));
 }

 #[test]
 fn test_vowels_count1() {
    assert_eq!(count_vowels(""), 0);
    assert_eq!(count_vowels("abEcd"), 2);
    assert_eq!(count_vowels("ab12Exey5 7x8U3y5z"), 4);
}
 
 
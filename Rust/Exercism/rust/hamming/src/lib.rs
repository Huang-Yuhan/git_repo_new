/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {return None;}
    let mut sum=0;
    let mut i=0;
    while i<s1.len() {
        if s1.chars().nth(i).unwrap() != s2.chars().nth(i).unwrap() {sum+=1;}
        i+=1;
    }
    Some(sum)
}

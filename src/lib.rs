pub fn distance (a: &[u8], b: &[u8]) -> Vec<u8> {
    let a_len: usize = a.len();
    if a_len != b.len() { panic!("inputs must be of same length"); }
    let mut buf: Vec<u8> = Vec::with_capacity(a_len);
    for i in 0..a_len { buf.push(a[i] ^ b[i]); }
    buf
}

// pub fn compare (a: &[u8], b: &[u8]) -> i8 {}
//
// pub fn lt (a: &[u8], b: &[u8]) -> bool {}
//
// pub fn gt (a: &[u8], b: &[u8]) -> bool {}
//
// pub fn eq (a: &[u8], b: &[u8]) -> bool {}

#[cfg(test)]
mod tests {
    #[test]
    fn distance () {
        let a = vec![ 36, 44 ];
        let b = vec![ 77, 88 ];
        let d = super::distance(&a, &b);
        assert_eq!(d, vec![ 105, 116 ]);
    }
    #[test]
    #[should_panic(expected = "inputs must be of same length")]
    fn distance_panic () {
        let a = vec![ 36, 44, 99 ];
        let b = vec![ 77, 88 ];
        super::distance(&a, &b);
    }
}

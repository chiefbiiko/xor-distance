pub mod xor_distance {
    pub fn distance(a: &[u8], b: &[u8]) -> Vec<u8> {
        let a_len: usize = a.len();
        if a_len != b.len() {
            panic!("inputs must be of same length");
        }
        let mut buf: Vec<u8> = Vec::with_capacity(a_len);
        for i in 0..a_len {
            buf.push(a[i] ^ b[i]);
        }
        buf
    }

    pub fn compare (a: &[u8], b: &[u8]) -> i8 {
        let a_len: usize = a.len();
        if a_len != b.len() {
            panic!("inputs must be of same length");
        }
        for i in 0..a_len {
            if a[i] == b[i] {
                continue;
            } else if a[i] < b[i] {
                return -1;
            } else {
                return 1;
            }
        }
        0
    }

    pub fn lt (a: &[u8], b: &[u8]) -> bool {
        compare(a, b) == -1
    }

    pub fn gt (a: &[u8], b: &[u8]) -> bool {
        compare(a, b) == 1
    }

    pub fn eq (a: &[u8], b: &[u8]) -> bool {
        compare(a, b) == 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[should_panic(expected = "inputs must be of same length")]
    fn distance_panic () {
        let a = vec![ 36, 44, 99 ];
        let b = vec![ 77, 88 ];
        super::xor_distance::distance(&a, &b);
    }
    #[test]
    fn distance () {
        let a = vec![ 36, 44 ];
        let b = vec![ 77, 88 ];
        assert_eq!(super::xor_distance::distance(&a, &b), vec![ 105, 116 ]);
    }
    #[test]
    #[should_panic(expected = "inputs must be of same length")]
    fn compare_panic () {
        let a = vec![ 36, 44, 99 ];
        let b = vec![ 77, 88 ];
        super::xor_distance::compare(&a, &b);
    }
    #[test]
    fn compare_lt () {
        let a = vec![ 36, 44 ];
        let b = vec![ 77, 88 ];
        assert_eq!(super::xor_distance::compare(&a, &b), -1);
    }
    #[test]
    fn compare_gt () {
        let a = vec![ 99, 44 ];
        let b = vec![ 77, 88 ];
        assert_eq!(super::xor_distance::compare(&a, &b), 1);
    }
    #[test]
    fn compare_eq () {
        let a = vec![ 36, 44 ];
        let b = vec![ 36, 44 ];
        assert_eq!(super::xor_distance::compare(&a, &b), 0);
    }
    #[test]
    fn lt () {
        let a = vec![ 36, 44 ];
        let b = vec![ 77, 44 ];
        assert_eq!(super::xor_distance::lt(&a, &b), true);
    }
    #[test]
    fn gt () {
        let a = vec![ 99, 44 ];
        let b = vec![ 77, 44 ];
        assert_eq!(super::xor_distance::gt(&a, &b), true);
    }
    #[test]
    fn eq () {
        let a = vec![ 99, 44 ];
        let b = vec![ 99, 44 ];
        assert_eq!(super::xor_distance::eq(&a, &b), true);
    }
}

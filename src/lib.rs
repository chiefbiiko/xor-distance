#[cfg(test)]
mod tests;

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

pub fn str_str_1(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(index) => index as i32,
        None => -1
    }
}

pub fn str_str_2(haystack: String, needle: String) -> i32 {
    fn make_lps(s: &str) -> Vec<usize> {
        let bytes: &[u8] = s.as_bytes();
        let mut lps: Vec<usize> = vec![0; s.len()];
        let mut lo: usize = 0;

        for hi in 1..s.len() {
            while lo > 0 && bytes[lo] != bytes[hi] {
                lo = lps[lo - 1];
            }
            if bytes[lo] == bytes[hi] {
                lo += 1;
                lps[hi] = lo;
            }
        }

        lps
    }

    let lps: Vec<usize> = make_lps(&needle);
    let bytes: &[u8] = needle.as_bytes();
    let mut lo: usize = 0;

    for (i, &ch) in haystack.as_bytes().iter().enumerate() {
        while lo > 0 && bytes[lo] != ch {
            lo = lps[lo - 1];
        }
        if bytes[lo] == ch {
            lo += 1;
            if lo == needle.len() {
                return (i - lo + 1) as i32;
            }
        }
    }

    -1
}

    

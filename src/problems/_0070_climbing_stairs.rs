pub fn climb_stairs_0(n: i32) -> i32 {
    match n {
        0..=3 => n,
        _ => climb_stairs_0(n - 1) + climb_stairs_0(n - 2),
    }
}

pub fn climb_stairs_1(n: i32) -> i32 {
    let mut prev: i32 = 0;
    let mut curr: i32 = 1;

    for _ in 0..n {
        std::mem::swap(&mut curr, &mut prev);
        curr += prev;
    }

    curr
}

pub fn climb_stairs_2(n: i32) -> i32 {
    if n == 0 {
        return 1;
    }

    let sqrt_5: f64 = 2.23606797749979;
    let phi: f64 = 1.618033988749895;
    let phi_inv: f64 = 0.6180339887498948;

    let rn: i32 = n + 1;
    ((phi.powi(rn) - phi_inv.powi(rn)) / sqrt_5).round() as i32
}

pub fn climb_stairs_3(x: i32) -> i32 {
    fn fibonacci(upto: i32, m: i32, n: i32) -> i32 {
        if upto != 0 {
            return fibonacci(upto - 1, n, m + n);
        }

        m
    }

    fibonacci(x, 1, 1)
}

pub fn climb_stairs_4(n: i32) -> i32 {
    fn npossible(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }

        let mut prev_prev: i32 = 1;
        let mut prev: i32 = 2;
        let mut current: i32 = prev_prev + prev;

        for _ in 3..=n {
            current = prev_prev + prev;
            prev_prev = prev;
            prev = current;
        }

        prev
    }

    npossible(n)
}

pub fn climb_stairs_5(n: i32) -> i32 {
    fn npossible(n: i32, cache: &mut Vec<i32>) -> i32 {
        if cache[n as usize] == -1 {
            cache[n as usize] = npossible(n - 2, cache) + npossible(n - 1, cache);
        }
        cache[n as usize]
    }

    let mut cache: Vec<i32> = vec![-1; 46];
    cache[1] = 1;
    cache[2] = 2;
    cache[3] = 3;
    npossible(n, &mut cache)
}

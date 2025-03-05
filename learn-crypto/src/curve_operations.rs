use big_int::BigInt;
use crate::inverse::inverse;

// def double(point)
// # slope = (3x₁² + a) / 2y₁
// slope = ((3 * point[:x] ** 2 + $a) * inverse((2 * point[:y]), $p)) % $p # using inverse to help with division
//
// # x = slope² - 2x₁
// x = (slope ** 2 - (2 * point[:x])) % $p
//
// # y = slope * (x₁ - x) - y₁
// y = (slope * (point[:x] - x) - point[:y]) % $p
//
// # Return the new point
// return { x: x, y: y }
// end

const P: &isize = &47;
const a: &isize = &0;

pub fn double(point: (isize, isize)) -> (isize, isize) {
    let slope = ((3 * point.0.pow(2)  + a) * inverse(point.1 * 2, *P)) % *P;
    println!("slope: {}", slope);
    let x = (slope.pow(2) - (2 * point.0)) % *P;
    let y = (slope * (point.0 - x) - point.1) % *P;
    return (x, y);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
    }
}
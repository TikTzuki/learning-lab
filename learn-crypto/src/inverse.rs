/**
Extended Euclidean algorithm
* d=gcd(a,b) and ax+by=d; <br>
* 1. if b=0, d<-a, x<-1, y<-0, return (d,x,y) <br>
* 2. x2 = 1, x1 = 0, y2 = 0, y1 = 1 <br>
* 3. while b>0 do <br>
* 3.1. q<-floor(a/b), r<-a-q*b, x<-x2-q*x1, y<-y2-q*y1 <br>
* 3.2. a<-b, b<-r, x2<-x1. x1<-x, y2<-y1, y1<-y <br>
* 4. d<-a, x<-x2, y<-y2, return (d,x,y) <br>
* m    a   r   q   y0  y1  y <br>
* 88   17  3   5   0   1   -5 <br>
*
* @param a
* @param b
* @return Unit(a, b, x, y, d);
 */
pub fn inverse(origin_a: isize, mut m: isize) -> isize {
    let mut a = origin_a;
    let mut x ;
    let mut y;
    if m == 0 {
        x = 1;
        return x;
    }
    let mut x2 = 1;
    let mut x1 = 0;
    let mut y2 = 0;
    let mut y1 = 1;
    let mut q;
    let mut r;
    while m > 0 {
        q = a / m;
        r = a - q * m;
        x = x2 - q * x1;
        y = y2 - q * y1;

        a = m;
        m = r;
        x2 = x1;
        x1 = x;
        y2 = y1;
        y1 = y;
    }
    y = y2;
    return if y < 0 { origin_a + y } else { y };
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use crate::inverse::inverse;

    use super::*;

    #[parameterized(
        a = { 43, 43, 47 },
        b = { 1, 2, 2 },
        expected = { 1, 22, 24 }
    )]
    fn test_inverse(a: isize, b: isize, expected: isize) {
        assert_eq!(inverse(a, b), expected);
    }
}

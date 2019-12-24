use super::assert::*;

type Password = (i32, i32, i32, i32, i32, i32);

fn contains_repeated(x: Password) -> bool {
    x.0 == x.1 || x.1 == x.2 || x.2 == x.3 || x.3 == x.4 || x.4 == x.5
}

fn is_increasing(x: Password) -> bool {
    x.0 <= x.1 && x.1 <= x.2 && x.2 <= x.3 && x.3 <= x.4 && x.4 <= x.5
}

fn contains_pair(x: Password) -> bool {
    (x.0 == x.1 && x.1 != x.2)
        || (x.1 == x.2 && x.0 != x.1 && x.2 != x.3)
        || (x.2 == x.3 && x.1 != x.2 && x.3 != x.4)
        || (x.3 == x.4 && x.2 != x.3 && x.4 != x.5)
        || (x.4 == x.5 && x.3 != x.4)
}

fn increment(x: Password) -> Password {
    match x {
        (a, 9, 9, 9, 9, 9) => (a + 1, a + 1, a + 1, a + 1, a + 1, a + 1),
        (a, b, 9, 9, 9, 9) => (a, b + 1, b + 1, b + 1, b + 1, b + 1),
        (a, b, c, 9, 9, 9) => (a, b, c + 1, c + 1, c + 1, c + 1),
        (a, b, c, d, 9, 9) => (a, b, c, d + 1, d + 1, d + 1),
        (a, b, c, d, e, 9) => (a, b, c, d, e + 1, e + 1),
        (a, b, c, d, e, f) => (a, b, c, d, e, f + 1),
    }
}

fn less_than(x: Password, y: Password) -> bool {
    x.0 < y.0
        || (x.0 == y.0 && x.1 < y.1)
        || (x.0 == y.0 && x.1 == y.1 && x.2 < y.2)
        || (x.0 == y.0 && x.1 == y.1 && x.2 == y.2 && x.3 < y.3)
        || (x.0 == y.0 && x.1 == y.1 && x.2 == y.2 && x.3 == y.3 && x.4 < y.4)
        || (x.0 == y.0 && x.1 == y.1 && x.2 == y.2 && x.3 == y.3 && x.4 == y.4 && x.5 < y.5)
}

pub fn solve() {
    let low = (3, 7, 2, 0, 3, 7);
    let high = (9, 0, 5, 1, 5, 7);
    let mut lax_count = 0;
    let mut strict_count = 0;
    let mut current = low;

    while less_than(current, high) {
        if is_increasing(current) {
            if contains_repeated(current) {
                lax_count += 1;
                if contains_pair(current) {
                    strict_count += 1;
                }
            }
        }

        current = increment(current);
    }

    assert_eq(Day::new(4, Part::A), 481, lax_count);
    assert_eq(Day::new(4, Part::B), 299, strict_count);
}

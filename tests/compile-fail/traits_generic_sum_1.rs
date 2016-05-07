// Attempt 1: Insufficient bounds on N.
//
// error-pattern: mismatched types
// error-pattern: binary operation `+` cannot be applied to type `N`

fn sum<N>(values: &Vec<N>) -> N {
    let mut total: N = 0;
    for v in values {
        total = total + *v;
    }
    total
}

fn main() {}

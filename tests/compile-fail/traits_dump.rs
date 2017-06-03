// Printing a value requires a bound (for Display or Debug, depending on the format string)

// error-pattern: the trait bound `<I as std::iter::Iterator>::Item: std::fmt::Debug` is not satisfied

/// Print out all the values produced by an iterator
fn dump<I>(iter: I)
    where I: Iterator
{
    for (index, value) in iter.enumerate() {
        println!("{}: {:?}", index, value);   // error
    }
}

fn main() {}

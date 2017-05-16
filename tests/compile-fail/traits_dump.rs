// Printing a value requires a bound (for Display or Debug, depending on the format string)

// error-pattern: the trait bound `<I as std::iter::Iterator>::Item: std::fmt::Debug` is not satisfied

/// Print out all the values produced by an iterator
fn dump<I>(iter: I)
    where I: Iterator
{
    let mut index: usize = 0;
    for value in iter {
        println!("{}: {:?}", index, value);   // error
        index += 1;
    }
}

fn main() {}
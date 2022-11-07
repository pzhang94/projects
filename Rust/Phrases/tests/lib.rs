#[cfg(test)]
mod tests
{
extern crate phrases;

// Unit tests
#[test]
fn english_greeting_correct()
{
    assert_eq!("hello", phrases::greetings::english::hello());
}

#[test]
#[should_panic]
fn english_greeting_incorrect()
{
    assert_eq!("asdf", phrases::greetings::english::hello());
}

#[test]
#[ignore]
fn english_greeting_ignored()
{
    assert_eq!(1, 0);
}

}
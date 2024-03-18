use itertools::structs::PeekNth;

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(Number),
    String(QuarkString),
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Int(i64),
    Float(f64),
    ComplexInt(i64),
    ComplexFloat(f64),
}

#[derive(Debug, PartialEq)]
pub struct QuarkString(String);

impl Literal {
    pub fn new<T>(stream: &mut PeekNth<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        match stream.peek() {
            Some(&'"') => Self::String(QuarkString::new(stream)),
            Some(&symbol) if symbol.is_numeric() => Self::Number(Number::new(stream)),
            _ => unreachable!(),
        }
    }
}

impl QuarkString {
    fn new<T>(stream: &mut PeekNth<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut string = String::new();
        stream.next(); // consume the opening quote

        while let Some(&symbol) = stream.peek() {
            if symbol == '"' {
                stream.next();
                break;
            }
            string.push(stream.next().unwrap());
        }
        QuarkString(string)
    }
}

enum NumberIs {
    ComplexInt,
    ComplexFloat,
    Int,
    Float,
}

impl Number {
    pub fn new<T>(stream: &mut PeekNth<T>) -> Self
    where
        T: Iterator<Item = char>,
    {
        let mut number = String::new();
        let mut num_type: NumberIs = NumberIs::Int;
        let mut is_int = true;

        stream.consume_digits_into(&mut number);

        if stream.peek() == Some(&'.') && stream.peek_next().unwrap().is_ascii_digit() {
            is_int = false;
            number.push(stream.next().unwrap());
            stream.consume_digits_into(&mut number);

            if stream.peek() == Some(&'i') {
                stream.next();
                num_type = NumberIs::ComplexFloat;
            } else {
                num_type = NumberIs::Float;
            }
        }

        if is_int {
            if stream.peek() == Some(&'i') {
                stream.next();
                num_type = NumberIs::ComplexInt;
            } else {
                num_type = NumberIs::Int;
            }
        }

        parse_number(&number, num_type)
    }
}

fn parse_number(number: &str, num_type: NumberIs) -> Number {
    match num_type {
        NumberIs::ComplexInt => Number::ComplexInt(
            number
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Failed to parse complex integer")),
        ),
        NumberIs::ComplexFloat => Number::ComplexFloat(
            number
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Failed to parse complex float")),
        ),
        NumberIs::Int => Number::Int(
            number
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Failed to parse integer")),
        ),
        NumberIs::Float => Number::Float(
            number
                .parse::<f64>()
                .unwrap_or_else(|_| panic!("Failed to parse float")),
        ),
    }
}

// Utils: PeekNext and ConsumeDigits

trait PeekNext<I: Iterator> {
    fn peek_next(&mut self) -> Option<&I::Item>;
}

impl<I: Iterator> PeekNext<I> for PeekNth<I> {
    fn peek_next(&mut self) -> Option<&<I as Iterator>::Item> {
        self.peek_nth(1)
    }
}

trait ConsumeDigits {
    /// consume_digits_into is a method that consumes digits from the input stream and appends them to the given number string.
    fn consume_digits_into(&mut self, number: &mut String);
}

impl<T> ConsumeDigits for PeekNth<T>
where
    T: Iterator<Item = char>,
{
    fn consume_digits_into(&mut self, number: &mut String) {
        while let Some(&symbol) = self.peek() {
            if !symbol.is_ascii_digit() {
                break;
            }
            number.push(self.next().unwrap());
        }
    }
}

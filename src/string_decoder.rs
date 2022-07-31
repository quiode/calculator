/// Module for decoding a string into math operations
pub mod string_decoder {
    enum Operation {
        Addition,
        Subtraction,
        Division,
        Multiplication,
    }

    /// Takes a string with math operations as input and executes them
    ///
    /// # Arguments
    ///
    /// * `input` - The String to decode
    ///
    /// # ONLY ONE OPERATION AT THE TIME
    ///
    /// # Supported Opperations
    /// - Addition
    /// - Subtraction
    /// - Division
    /// - Multiplication
    /// - ~Brackets~
    ///
    ///
    ///
    /// # Examples
    /// ```rust
    /// assert!(decode("1+1") == 2);
    /// ```
    pub fn decode(input: &str) -> i32 {
        // check if first char is a numeric, if not panic because input is wrong
        if !input.chars().next().unwrap().is_numeric() {
            panic!("First character can't be an operator!")
        }

        let mut chars = input.chars().into_iter();

        let mut num1 = String::new();
        let mut num2 = String::new();

        let mut operation: Option<Operation> = None;

        let mut tmp_char: char;

        // get first number and operation
        while operation.is_none() {
            match chars.next() {
                None => break,
                Some(x) => tmp_char = x,
            }

            if tmp_char.is_numeric() {
                num1.push(tmp_char)
            } else {
                match tmp_char {
                    '+' => operation = Some(Operation::Addition),
                    '-' => operation = Some(Operation::Subtraction),
                    '*' => operation = Some(Operation::Multiplication),
                    '/' => operation = Some(Operation::Division),
                    _ => panic!("Operator not supported!"),
                }
            }
        }

        // get second number
        for tmp_char in chars {
            if tmp_char.is_numeric() {
                num2.push(tmp_char)
            }
        }

        let result: i32;

        let num1 = str_to_int(&num1);
        let num2 = str_to_int(&num2);

        // calculate result
        match operation {
            Some(op) => match op {
                Operation::Addition => result = num1 + num2,
                Operation::Subtraction => result = num1 - num2,
                Operation::Division => result = num1 / num2,
                Operation::Multiplication => result = num1 * num2,
            },
            None => panic!("No operation selected"),
        }

        result
    }

    fn str_to_int(string: &str) -> i32 {
        string.parse::<i32>().unwrap()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn add() {
            assert!(decode("1+1") == 2);
        }

        #[test]
        fn sub() {
            assert!(decode("2-1") == 1);
        }

        #[test]
        fn div() {
            assert!(decode("8/2") == 4);
        }

        #[test]
        fn mult() {
            assert!(decode("8*2") == 16);
        }
    }
}

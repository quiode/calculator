/// Module for decoding a string into math operations
pub mod string_decoder {
    /// Takes a string with math operations as input and executes them
    ///
    /// # Arguments
    ///
    /// * `input` - The String to decode
    ///
    /// # Supported Opperations
    /// - Addition
    /// - Subtraction
    /// - Division
    /// - Multiplication
    /// - ~Brackets~
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

        let mut result = 0;

        let mut temp_number = String::new();

        // cycle trough every character, if is a number, at to temporary storage, if it is an operation, execute it on the sum and the temporary stored number
        for char in input.chars() {
            // if char is numeric, add to temp_number
            if char.is_numeric() {
                temp_number.push(char);
            } else {
                match char {
                    '+' => result += str_to_int(&temp_number),
                    '-' => result -= str_to_int(&temp_number),
                    '/' => result /= str_to_int(&temp_number),
                    '*' => result *= str_to_int(&temp_number),
                    _ => panic!("Operation not supported!"),
                }

                temp_number.clear();
            }
        }

        return result;
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

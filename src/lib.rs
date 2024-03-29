mod collatz_seq {

    pub fn collatz_sequence(mut number: i32) -> i32 {
        //Collatz sequence
        //Even number: n / 2
        //Odd number: (n * 3) + 1

        let mut col_seq: Vec<i32> = [0].to_vec(); //Initialised

        while number != 1 {
            if number % 2 == 0 {
                number = number / 2;
                println!("{}", number);
                col_seq.push(number)
            } else {
                number = (number * 3) + 1;
                println!("{}", number);
                col_seq.push(number)
            }
        }

        //Print individual numbers without needing to implement the Display trait for Vec<i32> types.
        //Will change to be a separate function in future iterations.
        for n in col_seq.iter() {
            print!("{}, ", n)
        }
        return col_seq[0];
    }
}

//Unit tests currently don't work.
#[cfg(test)]
mod tests {
    use crate::collatz_seq::collatz_sequence;

    #[test]
    fn one() {
        assert_eq!(collatz_sequence(1), 1)
    }

    #[test]
    fn positive_small() {
        assert_eq!(collatz_sequence(10), 1)
    }

    #[test]
    fn positive_large() {
        assert_eq!(collatz_sequence(100), 1)
    }
}

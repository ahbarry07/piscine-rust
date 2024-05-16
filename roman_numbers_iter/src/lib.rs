use crate::RomanDigit::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
	Nulla,
	I,
	V,
	X,
	L,
	C,
	D,
	M,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>);

impl From<u32> for RomanDigit {
	fn from(value: u32) -> Self {
		match value {
            0 => Nulla,
            1 => I,
            5 => V,
            10 => X,
            50 => L,
            100 => C,
            500 => D,
            1000 => M,
            _ => todo!(),
        }
	}
}

impl From<u32> for RomanNumber {
	fn from(value: u32) -> Self {
        print!("value {:?}", value);
		let mut num = value;
		let mut roman_digits = Vec::new();
        let mut index = 0;
        let  powers_of_ten = vec![1000, 100, 10, 1];

		if num <= 0 {
            return RomanNumber(vec![RomanDigit::Nulla]);
        }

        while num > 0 {
            let digit = num / powers_of_ten[index];
            if digit > 0 {
                match digit {
                    1..=3 => {
                        for _ in 0..digit {
                            roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        }
                    }
                    4 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                    }
                    5 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                    }
                    6..=8 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index] * 5));
                        for _ in 0..(digit - 5) {
                            roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        }
                    }
                    9 => {
                        roman_digits.push(RomanDigit::from(powers_of_ten[index]));
                        roman_digits.push(RomanDigit::from(powers_of_ten[index - 1]));
                    }
                    _ => unreachable!(),
                }
            }
            num %= powers_of_ten[index];
            index += 1;
        }
        
        RomanNumber(roman_digits)
	}
	
}

impl Iterator for RomanNumber {
     type Item = RomanNumber;

    fn next(&mut self) -> Option<Self::Item> {
        let mut digits = self.0.clone();
        digits.push(I); 
        Some(RomanNumber(digits))
    }
}
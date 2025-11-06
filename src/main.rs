use std::io::{self, Write};

struct Calculator
{
    frequent_maclaurin_terms: u8,
    rare_maclaurin_terms: u8,
    newton_iterations: u8,
    ln2: f64,
    inverse_ln2: f64,
}

impl Calculator
{
    fn new() -> Self
    {
        let mut new_instance = Calculator
        {
            frequent_maclaurin_terms: 16,
            rare_maclaurin_terms: 32,
            newton_iterations: 16,
            ln2: 0.0,
            inverse_ln2: 0.0,
        };
        new_instance.ln2 = new_instance.ln_approximation(2.0);
        new_instance.inverse_ln2 = 1.0 / new_instance.ln2;
        return new_instance;
    }

    fn maclaurin_exp(&self, x: f64, terms: u8) -> f64
    {
        let mut result: f64 = 1.0;
        for n in (1..terms).rev()
        {
            result = 1.0 + ((result * x) / (n as f64));
        }
        return result;   
    }

    fn approximate_newton_lny_x(&self, x: f64, y: f64) -> f64
    {
        return x - (self.exp_approximation(x) - y)/self.exp_approximation(x);
    }

    fn exp_approximation(&self, x: f64) -> f64
    {
        return self.maclaurin_exp(x, self.rare_maclaurin_terms);
    }

    fn ln_approximation(&self, x: f64) -> f64
    {
        let mut guess: f64 = x/2.0;
        for _ in 0..self.newton_iterations
        {
            guess = self.approximate_newton_lny_x(guess, x);
        }
        return guess;
    }

    fn f64_extraction(&self, x: f64) -> (u8, f64, i16)
    {
        let bits: u64 = x.to_bits();
        let sign: u8 = (bits >> 63) as u8;
        let unadjusted_exponent: u64 = (bits >> 52) & 0x7FF;
        let significand_bits: u64 = bits & 0x000F_FFFF_FFFF_FFFF;
        if unadjusted_exponent == 0
        {
            // ZERO
            if significand_bits == 0
            {
                return (sign, 0.0, -1022);
            }
            // DENORMAL
            return (sign, (significand_bits as f64) * f64::from_bits(0x3CB0_0000_0000_0000), -1022);
        }
        else
        {
            let significand: f64 = f64::from_bits(significand_bits | 0x3FF0_0000_0000_0000);
            if unadjusted_exponent == 0x7FF
            {
                // INFINITY
                if significand_bits == 0
                {
                    return (sign, 0.0, 2047);
                }
                // NaN
                return (sign, significand, 2047);
            }
            // NORMAL
            return (sign, significand, unadjusted_exponent as i16 - 1023); 
        }
    }

    fn power_2(&self, power: i64) -> f64
    {
        // Larger than the largest IEEE754 normal number possible
        if power > 1023
        {
            return f64::INFINITY;
        }
        // Smaller the smallest IEEE754 denormal number possible
        else if power < -1074
        {
            return 0.0;
        }
        // Within normal number range
        else if power >= -1022
        {
            return f64::from_bits(((1023 + power) as u64) << 52);
        }
        // Within denormal number range
        return f64::from_bits(1u64 << ((power + 1074) as u64));
    }

    fn exp(&self, x: f64) -> f64
    {
        // e^-exp = 1 / e^exp
        if x < 0.0
        {
            return 1.0 / self.exp(-x);
        }
        // e^x = 2^whole * e^(x - whole ln2)
        let whole: i64 = (x * self.inverse_ln2 + 0.5) as i64;
        let decimals: f64 = x - (whole as f64) * self.ln2;
        let mult: f64 = self.maclaurin_exp(decimals, self.frequent_maclaurin_terms);
        return self.power_2(whole) * mult;
    }

    fn newton_new_a_lnx_a(&self, a: f64, x: f64) -> f64
    {
        // for finding a better x such that f(x) gets closer to 0 :
        // N(x) = x - f(x)/f'(x)
        return a - 1.0 + (x / self.exp(a));
    }

    fn ln(&self, x: f64) -> f64
    {
        let (sign, significand, exponent): (u8, f64, i16) = self.f64_extraction(x);
        if sign == 1
        {
            if significand == 0.0
            {
                return f64::NEG_INFINITY;
            }
            return f64::NAN;
        }
        // ln x = ln significand + exp ln 2
        let mut guess: f64 = significand - 1.0;
        for _ in 1..self.newton_iterations
        {
            guess = self.newton_new_a_lnx_a(guess, significand);
        }
        return guess + self.ln2 * (exponent as f64);
    }
}

fn main()
{
    let mut input: String = String::new();
    let calc: Calculator = Calculator::new();
    let t: u8 = 10;
    for i in 1..t+1
    {
        print!("Enter a number ({:02}/{:02}): ", i, t);
        io::stdout().flush().expect("Failed to flush STDOUT");
        io::stdin().read_line(&mut input).expect("Failed to read from STDIN");
        let user_number: Result<f64, std::num::ParseFloatError> = input.trim().parse::<f64>();
        input.clear();
        if user_number.is_err()
        {
            println!("Invalid input: You should enter a number.");
            continue;
        }
        let x: f64 = user_number.unwrap();
        println!("exp({:.2}) = {}", x, (x).exp());
        println!("exp({:.2}) ~ {}", x, calc.exp(x));
        println!("ln({:.2}) = {}", x, (x).ln());
        println!("ln({:.2}) ~ {}", x, calc.ln(x));
    }
}
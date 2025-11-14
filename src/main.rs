use std::{f64, io::{self, Write}};

struct Calculator
{
    maclaurin_terms: u8,
    newton_iterations: u8,
    ln2: f64,
    inverse_ln2: f64,
}

impl Calculator
{
    fn new() -> Self
    {
        let new_instance = Calculator
        {
            maclaurin_terms: 24,
            newton_iterations: 24,
            ln2: f64::consts::LN_2,
            inverse_ln2: f64::consts::LOG2_E,
        };
        return new_instance;
    }

    /*
    fn maclaurin_exp_backwards(&self, x: f64, terms: u8) -> f64
    {
        // slightly faster
        // less accurate
        let mut result: f64 = 1.0;
        for n in (1..terms).rev()
        {
            result = 1.0 + ((result * x) / (n as f64));
        }
        return result;   
    }
    */

    fn maclaurin_exp_forwards(&self, x: f64, terms: u8) -> f64
    {
        // slightly slower
        // more accurate
        let mut result = 1.0;
        let mut term = 1.0;
        for n in 1..terms
        {
            term *= x / (n as f64);
            result += term;
        }
        return result;
    }

    fn maclaurin_exp(&self, x: f64, terms: u8) -> f64
    {
        return self.maclaurin_exp_forwards(x, terms);
    }

    fn newton_new_a_lnx_a(&self, a: f64, x: f64, exp_function: impl Fn(f64) -> f64) -> f64
    {
        // for finding a better x such that f(x) gets closer to 0 :
        // N(x) = x - f(x)/f'(x)
        return a - 1.0 + (x / exp_function(a));
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
        // let whole: i64 = (x * self.inverse_ln2 + 0.5) as i64;
        let whole: i64 = (x * self.inverse_ln2).round() as i64;
        let decimals: f64 = x - (whole as f64) * self.ln2;
        let mult: f64 = self.maclaurin_exp(decimals, self.maclaurin_terms);
        return self.power_2(whole) * mult;
    }

    fn ln(&self, x: f64) -> f64
    {
        let (sign, significand, exponent): (u8, f64, i16) = self.f64_extraction(x);
        if sign == 1
        {
            if significand == 0.0
            {
                // The standard is to return -inf even when -0 should technically mathematically be NaN
                return f64::NEG_INFINITY;
            }
            return f64::NAN;
        }
        if x == 0.0
        {
            return f64::NEG_INFINITY;
        }
        // ln x = ln significand + exp ln 2
        let mut guess: f64 = significand - 1.0;
        for _ in 1..self.newton_iterations
        {
            guess = self.newton_new_a_lnx_a(guess, significand, |value| self.exp(value));
        }
        return guess + self.ln2 * (exponent as f64);
    }

    fn format_output(&self, x: f64) -> String
    {
        if x.abs() == 0.0
        {
            return "0".to_string();
        }
        else if x.abs() > 1e3
        {
            return format!("{:.5e}", x);
        }
        else if x.abs() < 1e-3
        {
            return format!("{:.5e}", x);
        }
        else if x.is_infinite()
        {
            if x.is_sign_positive()
            {
                return "inf".to_string();
            }
            return "-inf".to_string();
        }
        else if x.is_nan()
        {
            return "NaN".to_string();
        }
        else
        {
            return format!("{:.7}", x);
        }
    }
}

fn main()
{
    println!("[!] Project URL: https://GitHub.com/FYI-PSA/Rust-Calculator/");
    let mut input: String = String::new();
    let calc: Calculator = Calculator::new();
    // let t: i64 = 10_000_000;
    // for i in (-t-1)..(t+1)
    let t: i16 = 50;
    println!("[!] Enter nothing to quit.");
    for i in 1..t+1
    {
        print!("[?] Enter a number ({:02}/{:02}): ", i, t);
        io::stdout().flush().expect("[#] Failed to flush STDOUT");
        io::stdin().read_line(&mut input).expect("[#] Failed to read from STDIN");
        input = String::from(input.trim());
        let user_number: Result<f64, std::num::ParseFloatError> = input.parse::<f64>();
        let x: f64 = if user_number.is_err()
        {
            if input.is_empty()
            {
                println!("\n[!] Goodbye!\n");
                break;
            }
            else
            {
                println!("[#] Invalid input: You should enter a number.");
                continue;
            }
        }
        else
        {
            user_number.unwrap()
            // i as f64
        };
        input.clear();
        let exp_x: f64 = calc.exp(x);
        let ln_x: f64 = calc.ln(x);
        println!("exp({}) = {}", calc.format_output(x), calc.format_output(exp_x));

        if exp_x.is_finite()
        {
            let delta_exp: f64 = x.exp() - exp_x;
            let relative_delta: f64 = delta_exp / x.exp();
            if relative_delta.abs() > 1e-7
            {
                println!("[!] WARNING: Calculator.exp too inaccurate when x = {}", calc.format_output(x));
            }
        }
        
        println!("ln({}) = {}", calc.format_output(x), calc.format_output(ln_x));
        
        if x.is_sign_positive()
        {
            let delta_ln: f64 = x.ln() - ln_x;
            let relative_delta: f64 = delta_ln / x.ln();
            if relative_delta.abs() > 1e-7
            {
                println!("[!] WARNING: Calculator.ln too inaccurate when x = {}", calc.format_output(x));
            }
        }

        println!("");
    }
}
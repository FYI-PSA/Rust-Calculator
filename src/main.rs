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
        {frequent_maclaurin_terms: (16), rare_maclaurin_terms: (32), newton_iterations: (16), ln2: (0.0), inverse_ln2: (0.0)};
        new_instance.ln2 = new_instance.ln_approximation(2.0);
        new_instance.inverse_ln2 = 1.0 / new_instance.ln2;
        return new_instance;
    }

    fn maclaurin_exp(&self, x: f64, terms: u8) -> f64
    {
        let mut result: f64 = 1.0;
        for n in (1..terms).rev()
        {
            result *= x/(n as f64);
            result += 1.0;
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

    fn binary_exponentiation(&self, exponent: i64) -> f64
    {
        if exponent > 1023
        {   return f64::INFINITY; }
        else if exponent < -1022
        {   return 0.0; }
        let exponent_bits: u64 = ((1023 + exponent) as u64) << 52;
        return f64::from_bits(exponent_bits);
        // sign is 0, and the significand is 1.0000...00
        // literally the entire new number is just a bitshift of the exponent to its place
    }

    fn exp(&self, x: f64) -> f64
    {
        if x < 0.0
        {
            return 1.0 / self.exp(-x);
        }
        let whole: i64 = (x * self.inverse_ln2 + 0.5) as i64;
        let decimals: f64 = x - (whole as f64) * self.ln2;
        let mult: f64 = self.maclaurin_exp(decimals, self.frequent_maclaurin_terms);
        return self.binary_exponentiation(whole) * mult;
    }

    fn newton_new_a_lnx_a(&self, a: f64, x: f64) -> f64
    {
        return a - 1.0 + (x / self.exp(a));
    }

    fn ln(&self, x: f64) -> f64
    {
        if x < 0.0 || (x == 0.0 && x.is_sign_negative())
        {
            return f64::NAN;
        }
        else if x == 0.0 && x.is_sign_positive()
        {
            return f64::NEG_INFINITY;
        }
        // TODO: 
        //  BETTER FIRST GUESS
        //  RANGE REDUCTION
        let mut guess: f64 = if x > 1.0 { (x*x - 1.0)/(2.0 * x) } else { -2.0 } ;
        for _ in 1..self.newton_iterations
        {
            guess = self.newton_new_a_lnx_a(guess, x);
        }
        return guess;
    }
}

fn main()
{
    let mut input: String = String::new();
    let calc: Calculator = Calculator::new();
    let mut x: f64;
    let t: u8 = 5;
    for i in 1..t
    {
        print!("Enter a number ({:02}/{:02}): ", i, t);
        io::stdout().flush().expect("Failed to flush STDOUT");
        io::stdin().read_line(&mut input).expect("Failed to read from STDIN");
        x = input.trim().parse().expect("Input wasn't a number");
        input.clear();
        println!("exp({:.2}) = {}", x, (x).exp());
        println!("exp({:.2}) ~ {}", x, calc.exp(x));
        println!("ln({:.2}) = {}", x, (x).ln());
        println!("ln({:.2}) ~ {}", x, calc.ln(x));
    }
}
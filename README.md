# Rust Calculator

A fast and high-precision calculator in Rust, for mathematical functions using optimized numerical methods and proper handling of edge cases. Built as a learning project to explore Rust, floating-point arithmetic, and algorithmic optimization.

## Features

- **exp(x)**
- **ln(x)**

## Planned Features

- [ ] Square root
- [ ] Trigonometric functions (sin, cos, tan)
- [ ] Inverse trigonometric functions (arcsin, arccos, arctan)
- [ ] x exp(x)
- [ ] Lambert W function (branch 0, positive reals)

## Possible New Features (Not Planned)

- [ ] Hyperbolic functions
- [ ] Inverse hyperbolic functions
- [ ] Error function
- [ ] Complementary error function  
- [ ] Gamma function

## Usage

```bash
cargo run
```

Then enter numbers when prompted. As of right now, the calculator will compute both `exp(x)` and `ln(x)`, showing results with automatic formatting.

## Example

```
[!] Project URL: https://GitHub.com/FYI-PSA/Rust-Calculator/
[!] Enter nothing to quit.
[?] Enter a number (01/50): 0
exp(0) = 1.0000000
ln(0) = -inf

[?] Enter a number (02/50): 1
exp(1.0000000) = 2.7182818
ln(1.0000000) = 0

[?] Enter a number (03/50): 2
exp(2.0000000) = 7.3890561
ln(2.0000000) = 0.6931472

[?] Enter a number (04/50): 5
exp(5.0000000) = 148.4131591
ln(5.0000000) = 1.6094379

[?] Enter a number (05/50): 10
exp(10.0000000) = 2.20265e4
ln(10.0000000) = 2.3025851

[?] Enter a number (06/50): -10
exp(-10.0000000) = 4.53999e-5
ln(-10.0000000) = NaN

[?] Enter a number (07/50): -5
exp(-5.0000000) = 0.0067379
ln(-5.0000000) = NaN

[?] Enter a number (08/50):

[!] Goodbye!

```

## Note

This is an educational project focusing on precision, speed, and algorithms. For production use, prefer Rust's standard library functions!

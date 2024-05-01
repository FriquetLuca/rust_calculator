# Rust Calculator

>This project is just a simple calculator that parse a string such as `2 + sin(7 / 3) / 2` to compute it's result.

## Getters

1. PreviousResult (@)
 You can use the symbol `@` as a number being the previous answer given by the calculator. If there wasn't any previous answer, it's value will be equal to `0`.
 Example:
 `2 * 4`
 `= 8`
 `3 + @`
 `= 11`

## Operators

Since there's a lot of things that could be simplified visually, here's the full list of operators implemented.

1. Add (x+y)
  This operator handle the addition between two number.
  Example:
  `1 + 2`
  `= 3`
1. Subtract (x-y)
  This operator handle the subtraction between two number.
  Example:
  `2 - 1`
  `= 1`
1. Multiply (x*y)
  This operator handle the multiplication between two number.
  Example:
  `3 * 2`
  `= 6`
1. Divide (x/y)
  This operator handle the division between two number.
  Example:
  `4 / 2`
  `= 2`
1. Modulo (x%y)
  This operator handle the rest of the euclidian division between two number.
  Example:
  `4 % 2`
  `= 0`
1. PowerOf (x^y)
  This operator handle the power of `x` by `y`, `x` and `y` being both numbers.
  Example:
  `3^3`
  `= 27`
1. PowerOf2 (x²)
  This operator handle the power of `x` by 2, `x` being a number.
  Example:
  `5²`
  `= 25`
1. PowerOf3 (x³)
  This operator handle the power of `x` by 3, `x` being a number.
  Example:
  `4³`
  `= 64`
1. Factorial (x!)
  This operator handle the factorial of a real `x`.
  Example:
  `5!`
  `= 120`
1. DegToRad (x°)
  This operator handle the conversion from degree to radian. You should note that it's priority is the same as multiplication.
  Example:
  `3°`
  `= 0.05235987755982989`
1. RadToDeg (x rad)
  This operator handle the conversion from radian to degree. You should note that it's priority is the same as multiplication.
  Example:
  `3 rad`
  `= 171.8873385393`

## Function notation

Some function can be written purely using their original mathematical notation if wanted.

1. Abs (|x|)
  This function compute the absolute value of a number.
  Example:
  `|-5|`
  `= 5`
1. Floor (⌊x⌋)
  This function gives the greatest integer less than or equal to `x`.
  Example:
  `⌊2.4⌋`
  `= 2`
1. Ceiling (⌈x⌉)
  This function gives the smallest integer greater or equal to `x`.
  Example:
  `⌈2.4⌉`
  `= 3`

## Functions

1. Absolute value (abs(x))
1. Signum (sgn(x), sign(x), signum(x))
1. Power (pow(x,y))
1. Square root (sqrt(x))
1. Exponential (exp(x), exp2(x))
1. Logarithm (ln(x), log(x, b))
1. Truncate (trunc(x), truncate(x))
1. Floor (floor(x))
1. Ceil (ceil(x))
1. Round (round(x))
1. Sin (sin(θ))
1. Asin (asin(x))
1. cos (cos(θ))
1. Acos (acos(x))
1. Tan (tan(θ))
1. Atan (atan(x))
1. Sinh (sinh(θ))
1. Asinh (asinh(x), arsinh(x))
1. Cosh (cosh(θ))
1. Acosh (acosh(x), arcosh(x))
1. Tanh (tanh(θ))
1. Atanh (atanh(x), artanh(x))
1. Extremum (min(...X), max(...X))
1. Atan 2 (atan2(y, x))

## Closing the calculator

To close the calculator, just write `exit` or `close` instead of an expression.

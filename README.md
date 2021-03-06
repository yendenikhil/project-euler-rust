# Puzzle Solving - Project Euler

Learning the Rust by solving the puzzles in on project Euler.
I am hoping that I will learn more about the rust structuring, and 
learn about the algorithms. 

# Problems

## Problem 17 [#](https://projecteuler.net/problem=17)

To count the letters of numbers from 1 to 1000, the difficult part was 
writing the spellings properly. No fancy algorithm. Just count the letters
one by one for each number from 1. 

## Problem 27 [#](https://projecteuler.net/problem=27)

We need to find out the max number of consecutive primes we can calculate
using Euler's quadratic prime equation. Once we find the coefficients, 
we just need to find product of them and that is all. 
The challenge here is to find out whether the number generated by quadratic
equation is prime or not. I have used a dump implementation for it and it can
be better. But other than that it is straight forward to solve. 
For speed purposed I have used memoization here as there is a lot of repetition
for checking whether number is prime or not. 
I have seen some places where people have created the sieves of prime numbers
and then just looked at it.


## Problem 31 [#](https://projecteuler.net/problem=31)

This was interesting one. We have to find out number of different ways we can 
create 2 pounds from the coins [1, 2, 5, 10, 20, 50, 100, 200]pence. 

We use dynamic programming to solve this but in reversed order. Starting 
point is there is only one way you can create 1,2,3,...200 using only 1 pence 
coins. Now using this as base case, we calculate, how many different ways we can 
create this series using 1p and 2p coins. 
number of ways (1 and 2 p) for i = number of ways (1p) for i + number 
                                   of ways (1p) for (i - denomination)
Using this we calculate upwards how many ways we can generate the 
number of ways we can calculate 1..200 using 1p then 1 and 2p and further.
Once we have all the calculations generated, we just take the value used 
to generate 200 (2 pounds).

## Problem 32 [#](https://projecteuler.net/problem=32)

Here we need to get all the products which satisfy pandigital requirements
for multiplicant, multiplier, product. using simple way to iterate two 
numbers 1..99 and other one 123..9876. Whichever satisfies the criterion
we take the product and add them all. We have to make sure that the products
are unique and there is more than one way to get product number. 

## Problem 33 [#](https://projecteuler.net/problem=33)

We need to use the bad cancellation to come up with correct answer. The range is 
only two digit numbers so brute forcing it works. The final answer is the 
denominator of the product of fractions brought to its lowest denominator. 


## Problem 41 [#](https://projecteuler.net/problem=41)

We need to find all pandigital numbers which are also prime. This sounds 
that we need to check all numbers from 12 to 987,654,321. This is too big 
of dataset. What we can do instead is test which series will have primes 
in it. 

- 2, 3, 5, 8 and 9 digit pandigital numbers have the sum of their digits 
which are divisible by 3 and hence they will be divisible by three. so 
no pandigital number there. 
- only 4 and 7 digit numbers are prime pandigital. 


## Problem 41 [#](https://projecteuler.net/problem=41)

lets build number to match the requirements
2,3,4 digits from the permutations of all the numbers from 0 to 9
and they should be divisible by 2.
3,4,5 digits from the perm of all numbers from 0 to 9
and they should be divisible by 3 and so on
now each entry will have max 720 permutations. so total size of sample
is max 72 ** 9 which is huge number but we will hardly 720 for each position.
for first one (dig 2 3 and 4 div by 2), we will have 360 
for third one (dig 4, 5 and 6 div by 5) we will have 144 and so on.

# References

- [Project Euler](https://projecteuler.net/)
- [Rust](https://www.rust-lang.org/)
- [Vim](https://www.vim.org/)


# Chickens
![](diagram.jpg)

## Description
This program gives the answer to the following problem:
>There are $n$ 100% accuracy hunters and $n$ chickens. The hunters line up side-to-side in front of the chickens, which are also lined up. Each hunter picks a chicken to shoot without communication with the other hunters. Each hunter shoots their chosen chicken at the same time. What is the probability that exactly 1 chicken survives?

The program solves this problem by bruteforcing all possible outcomes. Instead of using `for` loops to go through all possiblities, it uses numbers in different bases to represent the scenarios, exploiting the fact that computers can increment numbers quickly. It then parallelizes this with the excellent `rayon` library in the Rust Programming Language.

## Solution
I made this project as I wanted to find the exact solution to this problem and a naive Monte-Carlo simulation only gave me an approximate answer. With this project, I was able to find the exact solution for any number of chickens, allowing me to find the analytical solution: $$P(\text{surviving chickens}=1)=\frac{n!\cdot \binom{n}{2}}{n^n}$$

## How to Run
By default, the program will calculate the probability for 2 chickens up to 10 chickens. To run the program, run
```
$ cargo run --release
```
The program will print out the relevant probabilities and counts in a neat fashion.

## Example
```
$ cargo run --release
```
![](example_output.png)

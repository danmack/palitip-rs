# Palitip-rs - Calculate tips that are palindromes

## Just a fun Sunday program in rust-lang

What if my bill + tip at a restaurant always summed to a number which
is a palindrome?  For fun, write a rust program to show some
palindromic tips around the target tip percentage.

If you always do this, it could be a sort of sort of checksum :-)

So, this program will do the following:

List some possible tips that are nearby palindromes.

## Example

```shell
% cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/palitip-rs`
Enter the current bill total:  32.15
Enter percentage tip to add: 20
You entered 32.15 and wish a tip of approx +20%
Some possible palindrome totals are:
add $5.58 as a tip to get $37.73 total
add $6.68 as a tip to get $38.83 total
add $7.78 as a tip to get $39.93 total
add $7.89 as a tip to get $40.04 total
```

## Bugs / Missing Features

 - needs some UI polish
 - should have an api for an alternative to the cli only

## Dependencies

Just the std::io library

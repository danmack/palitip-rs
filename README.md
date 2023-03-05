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
   Compiling palitip-rs v0.1.0 (/home/user/src/rust/palitip-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
     Running `target/debug/palitip-rs`
Enter the current bill total:  32.15
Enter percentage tip to add: 20
You entered 32.15 and wish a tip of approx +20%
Some possible tips are:
$37.73
$38.83
$39.93
$40.04
```

## Bugs / Missing Features

 - needs some UI polish
 - should have an api for an alternative to the cli only

## Dependencies

Just the std::io library

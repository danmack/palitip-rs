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
cargo run

```

## Bugs

The program generates some false positives because my method is flawed
in assuming that the trailing zero would be preserved when converting
an f32 to a string.

Next version will try to fix this.

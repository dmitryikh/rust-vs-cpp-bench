# Huffman encoding

## Problem description

Build optimal prefix code for given string consisted of a-z letters. One the
first line output `k n`, where `k` -- number of distinct letters, `n` -- length
of encoded string. Next `k` lines should containt `letter: code`. On the last
line output encoded string.

```
Sample Input:
abacabad

Sample Output:
4 14
a: 0
b: 10
c: 110
d: 111
01001100100111
```

The original problem was taken form: https://stepik.org/lesson/13239/step/5?unit=3425

## Other

Job `make gen` will generate tests inputs for *Huffman encoding* and *Huffman decoding* problems.

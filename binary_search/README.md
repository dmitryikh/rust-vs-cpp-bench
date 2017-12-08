# Binary search

## Problem description

For the given arrays `A` and `B` print index `j` that is `A[j] = B[i]` for
every `i`, or `-1` if `B[i]` is not found in the `A`.  The first line contains
`n`, which is number of elements in `A`. And `n` integer numbers, which is
`A`'s elements. The second line containes the same information for `B` array.

```
Sample Input:
5 1 5 8 12 13
5 8 1 23 1 11

Sample Output:
3 1 -1 1 -1
```

The original problem was taken form: https://stepik.org/lesson/13246/step/4

## Note

If A contains several elements with the same value, we should find any of them.

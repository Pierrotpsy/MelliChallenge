# Part 2

You may have noticed that our little virtual machine is not Turing complete yet. Let's change this! What we need is a jump instruction:

* `jnz x y`: Jump to the instruction `y` steps away (forward for positive values of y, backward for negative values of y), but only if `x` (a constant or register) is non-zero (Jump Not Zero).

**Example:**

For the following input

```
jnz 0 8
mov a 9992
mov b 3
mov c -1
add a c
add b c
jnz b -2
jnz 1 2
mov a 10060
print a
mov a 10
print a
```

your interpreter should output:

```
✅
```

Once you have implemented the new instruction, run the code in `input-2.asm`. It should display a message as well as the solution, which you submit together with your code.

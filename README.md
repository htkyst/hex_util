# HEX data utility

## Main options

### Output hex file data on command line
`--show <filename>`

### Convert hex file format
`--convert <filename> <format>`

|format|Description|
|:-----------|------------|
|mot| Motorola S-record format|
|hex| Intel hex format|
|bin| Raw binary format|

### Create hex file
`--create <filename> <size> <mode>`
|mode|Description|
|:-----------|------------|
|rand| Random data|
|inc| Increment data|
|fill | Fill data|

### Remove hex data
`--remove <filename>`

## Sub options

### Specify address range
`--range <start> <end>`

"""
This function calculates the sum of hexadecimal numbers within a specified range.

Parameters:
- start (str): A hexadecimal string in the format `0x...` representing the starting value of the range.
- end (str): A hexadecimal string in the format `0x...` representing the ending value of the range.

Returns:
- int: The sum of all integers within the specified range, inclusive.

Note:
- Both `start` and `end` must be valid hexadecimal strings prefixed with `0x`.
"""
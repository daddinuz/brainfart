# BrainFart

BrainFart is my implementation of the one and the only [BrainFuck](https://en.wikipedia.org/wiki/Brainfuck).

### Instructions

| Opcode | Description                                                                                                      |
|--------|------------------------------------------------------------------------------------------------------------------|
| <      | Move to the previous cell on the tape                                                                            |
| >      | Move to the next cell on the tape                                                                                |
| -      | Decrement the value of the current cell                                                                          |
| +      | Increment the value of the current cell                                                                          |
| ,      | Read one byte of input from stdin and store it inside the current cell                                           |
| .      | Write the value of the current cell to stdout                                                                    | 
| \[     | If the value of the current cell is 0 then jump to the instruction after the matching `]`                        |
| ]      | If the value of the current cell is not 0 then jump to the instruction after the matching `[`                    |
| {      | Define a function identified by the value of the current cell and jump to the instruction after the matching `}` |
| }      | End function definition                                                                                          |
| \\     | Early return                                                                                                     |
| @      | Call the function identified by the value of the current cell                                                    |

### Extensions

In order to support functions BrainFart implements three additional instructions that are not present in BrainFuck:
1. `{`: Def
2. `}`: End
3. `\`: Ret
4. `@`: Call

### Implementation details

- 8 bits cells
- wrapping on owerflows
- unlimited tape size (on both directions)
- Zero on EOF
- When a function gets called the value of the current cell will be set to 0
- Functions can be redefined: at the start of the program every function is defined as noop
- Since cells are used to identify functions the maximum number of functions allowed is 256: from 0 to 255
- early return from outside of any function terminates execution (if not in interactive mode)

# 🐸 quarks 

Its like porth and brainfu\*k in one language, but with the best tokens. Cool huh.

**Stack-oriented language with 3 characters**

Errors are still bloat

## How does it work?

You have 4 modes:

- Native - this mode is pretty much just mode to enabling other modes
    - `'` - enables stack mode
    - `"` - enables math mode
    - \` - enables misc mode  
- Stack - In this mode you are pushing and popping from stack
    - `'` - goes back to native mode
    - `"` - pushes to the stack 0
    - \` - pops from stack
- Math - In this mode you are adding and subtracting from top of stack 
    - `"` - goes back to native mode
    - `'` - adds 1 from top of stack
    - \` - substracts 1 from top of stack
- Misc - This mode contains printing and looping
    - \` - goes back to native mode
    - `'` - prints number as number on the top of stack
    - `"` - Starts/stop looping - When there is 0 on the top it stops 

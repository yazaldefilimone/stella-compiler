```sh
# install rust
# https://www.rust-lang.org/tools/install

# build the binary
cargo build --release

chmod +x ./lue_vm

# run a lua file
./lue_vm features/stdlib/print.lua

```

### TODO

- [ ] Arithmetic expressions
  - [ ] Addition (`+`)
  - [ ] Subtraction (`-`)
  - [ ] Multiplication (`*`)
  - [ ] Division (`/`)
- [ ] Boolean expressions
  - [ ] And (`and`)
  - [ ] Or (`or`)
  - [ ] Not (`not`)
- [ ] Comparisons
  - [ ] Equality (`==`)
  - [ ] Not equal (`~=`)
  - [ ] Less than (`<`)
  - [ ] Greater than (`>`)
  - [ ] Less than or equal (`<=`)
  - [ ] Greater than or equal (`>=`)
- [ ] Concatenation
  - [ ] Concatenate strings (`..`)
- [ ] `if`
  - [ ] `if ... then ... end`
  - [ ] `if ... then ... else ... end`
  - [ ] `if ... then ... elseif ... then ... else ... end`
- [ ] `while`
  - [ ] `while ... do ... end`
- [ ] `for`
  - [ ] Numeric `for` loop (`for i = ... , ... , ... do ... end`)
  - [ ] Generic `for` loop (`for k, v in pairs(t) do ... end`)
- [ ] `repeat ... until`
  - [ ] `repeat ... until ...`
- [ ] function definition
  - [ ] `function ... end`
- [ ] function calls
  - [ ] Function call with arguments
  - [ ] Function return values
- [ ] anonymous functions
  - [ ] `local f = function (...) ... end`
- [ ] local variables
  - [ ] Declaration (`local`)
  - [ ] Assignment
  - [ ] Access local variables
- [ ] global variables
  - [ ] Declaration
  - [ ] Assignment
  - [ ] Access global variables
- [ ] tables
  - [ ] Table declaration
  - [ ] Access table elements
  - [ ] Insert and remove table elements
  - [ ] Metatables and metamethods
- [ ] Standard Library (Stdlib)
  - [ ] Base functions
    - [x] `print`
    - [ ] `type`
    - [ ] `tostring`
    - [ ] `tonumber`
    - [ ] `error`
    - [ ] `pcall`
    - [ ] `xpcall`
  - [ ] Math functions
    - [ ] `math.abs`
    - [ ] `math.sin`
    - [ ] `math.cos`
    - [ ] `math.sqrt`
    - [ ] `math.random`
    - [ ] `math.floor`
    - [ ] `math.ceil`
  - [ ] String functions
    - [ ] `string.len`
    - [ ] `string.sub`
    - [ ] `string.upper`
    - [ ] `string.lower`
    - [ ] `string.gsub`
    - [ ] `string.find`
  - [ ] Table functions
    - [ ] `table.insert`
    - [ ] `table.remove`
    - [ ] `table.concat`
    - [ ] `table.sort`
    - [ ] `table.unpack`
- [ ] Coroutines
  - [ ] `coroutine.create`
  - [ ] `coroutine.resume`
  - [ ] `coroutine.yield`
  - [ ] `coroutine.status`
- [ ] Metatables
  - [ ] `setmetatable`
  - [ ] `getmetatable`
- [ ] Metamethods
  - [ ] `__index`
  - [ ] `__newindex`
  - [ ] `__add`
  - [ ] `__sub`
  - [ ] `__mul`
  - [ ] `__div`
  - [ ] `__concat`
  - [ ] `__eq`
  - [ ] `__lt`
  - [ ] `__le`
- [ ] Error handling
  - [ ] Exception handling
  - [ ] Error messages

```

```

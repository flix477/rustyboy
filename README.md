# rustyboy
🎮 Very WIP Gameboy emulator written in Rust.
Please don't use it yet, it's no good.

### Blargg's CPU Tests
Test ROMs are available [here](http://slack.net/~ant/old/gb-tests/)

| Test        | Status           |
| ------------- |:-------------:|
| `01-special`           | Passing ✅      |
| `02-interrupts`        | Passing ✅      |
| `03-op sp,hl`          | Failing 🚫      |
| `04-op r,imm`          | Passing ✅      |
| `05-op rp`             | Passing ✅      |
| `06-ld r,r`            | Passing ✅      |
| `07-jr,jp,call,ret,rst`| Passing ✅      |
| `08-misc instrs`       | Passing ✅      |
| `09-op r,r`            | Passing ✅      |
| `10-bit ops`           | Passing ✅      |
| `11-op a,(hl)`         | Failing 🚫      |

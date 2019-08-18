# rustyboy
🎮 WIP Gameboy emulator for iOS, Web and PC written in [Rust](https://www.rust-lang.org).

[![CircleCI](https://circleci.com/gh/flix477/rustyboy/tree/master.svg?style=svg)](https://circleci.com/gh/flix477/rustyboy/tree/master)

This is a monorepo. You can find individual ports in their respective subfolders under `packages/`

### Feature list
| Feature                  | State                       |
| ------------------------ | --------------------------- |
| CPU emulation            | Almost complete (see tests) |
| PPU emulation            | Complete ✅                 |
| Input emulation          | Complete ✅                 |
| Timer emulation          | Complete ✅                 |
| Sound emulation          | Not started 🚫              |
| Serial port emulation    | Not started 🚫              |
| Game Boy Color emulation | Not started 🚫              |
| Super Game Boy emulation | Not started 🚫              |
| MBC emulation            | In progress ⚠️              |

### Blargg's CPU Tests
Test ROMs are available [here](http://slack.net/~ant/old/gb-tests/)

| Test                   | Status     |
| ---------------------- | ---------- |
| `01-special`           | Passing ✅ |
| `02-interrupts`        | Passing ✅ |
| `03-op sp,hl`          | Failing 🚫 |
| `04-op r,imm`          | Passing ✅ |
| `05-op rp`             | Passing ✅ |
| `06-ld r,r`            | Passing ✅ |
| `07-jr,jp,call,ret,rst`| Passing ✅ |
| `08-misc instrs`       | Passing ✅ |
| `09-op r,r`            | Passing ✅ |
| `10-bit ops`           | Passing ✅ |
| `11-op a,(hl)`         | Passing ✅ |

### Want to make your own? Have some reads
- [Official Gameboy Programming Manual](https://ia801906.us.archive.org/19/items/GameBoyProgManVer1.1/GameBoyProgManVer1.1.pdf)
- [Pandocs](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf) (also available in HTML [here](http://gbdev.gg8.se/wiki/articles/Pan_Docs))
- [Gameboy cycle-accurate docs](https://github.com/AntonioND/giibiiadvance/blob/master/docs/TCAGBD.pdf)

# Rust OS

## Todo:
* Put `rustup target add aarch64-unknown-none-softfloat`in some config. Which one? Reason: Switched back to "Stable", then overrode current directory with "Nightly". After that, target was gone and had to add again.

## Building from source
`cargo build --target aarch64-unknown-none-softfloat`

### Target JSON
To see the current target JSON, use
`rustc +nightly -Z unstable-options --target=aarch64-unknown-none-softfloat --print target-spec-json`

which outputs
```JSON
{
  "arch": "aarch64",
  // https://llvm.org/docs/LangRef.html#data-layout
  "data-layout": "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128",
  // https://os.phil-opp.com/red-zone/
  "disable-redzone": true,
  "env": "",
  "executables": true,
  // "+" enables a feature, "-" disables a feature
  // https://os.phil-opp.com/disable-simd/
  "features": "+strict-align,-neon,-fp-armv8",
  "is-builtin": true,
  // https://lld.llvm.org/
  "linker": "rust-lld",
  "linker-flavor": "ld.lld",
  "linker-is-gnu": true,
  "llvm-target": "aarch64-unknown-none",
  "max-atomic-width": 128,
  "os": "none",
  "panic-strategy": "abort",
  "relocation-model": "static",
  "target-c-int-width": "32",
  "target-endian": "little",
  "target-pointer-width": "64",
  "unsupported-abis": [
    "stdcall",
    "fastcall",
    "vectorcall",
    "thiscall",
    "win64",
    "sysv64"
  ],
  "vendor": ""
}
```

## Inspecting:
`rust-$tool target/aarch64-unknown-none-softfloat/debug/rust_os`
### List of tools
* nm - List all symbols in an executable
* objcopy - Transform the output of Cargo (ELF) into binary format
* objdump - Disassemble a binary
* size - Print binary size in System V format
* strip - Strip all symbols from the build artifact
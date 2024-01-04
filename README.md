# SiCompiler v0.1

> [!NOTE]
> This stardars will be updates.

This is a simple compiler that prepare files for be using in SiCoMe simulator realized by the University of Cordoba (UCO).

## Installation

```bash
  mkdir build
  cd build
  cmake ..
  make
```

## How to use

The only thing you need to do is write the next command in the directory you have the executable

```bash
./sc.exe input_file_dir output_file_dir
```

The result will be an output file with the name you write in the same directory you are.

## Standars

This project is not complete, so is possible that it may be update in the future, but in this momment this compiler meets some standars that have been written resently.

### Comments

The comments in one line will be specified by a semicolon `;`:

```bash
CRA 23 ;One line comment
```

### Valid instructions

- CRA
- CTA
- ITA
- CRF
- CTF
- SFZ
- ROR_F_ACC
- ROL_F_ACC
- ADD dir
- ADDI dir
- STA dir
- JMP dir
- JMPI dir
- HALT

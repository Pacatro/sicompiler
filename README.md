# SiCompiler v0.1

This is a simple compiler that prepare files for be using in SiCoMe simulator realized by the University of Cordoba (UCO).

## How to use :pencil2:

The only thing you need to do is write the next command in the directory you have the executable.

### Without custom repertoire

```terminal
./sc <input_file_path> <output_file_path>
```

### With custom repertoire

```terminal
./sc <input_file_path> <output_file_path> -r <custom_repertoire_path>
```

The result will be an output file with the path you write or a default path in the same directory you are.

## Standars :books:

> [!IMPORTANT]
> These standards are subject to updates.

This project is not complete, so is possible that it may be update in the future, but in this momment this compiler meets some standars that have been written resently.

### One-line comments

The comments in one line will be specified by a semicolon `;`:

```terminal
CRA 23 ;One line comment
```

### Multi-line comments

The comments in one line will be specified by `***` at the beginig and another at the end:

```terminal
CRA 23 
***Multi 
line 
comment***
```

#### Program example

```terminal
0 0003
1 0003
3 0000
@
6
@
CRA 
CRF 
ADD 0001
ROR_F_ACC 
SFZ 
JMP 12
CRA 
CRF 
ADD 0000
ROL_F_ACC 
STA 0004
HALT 
CRA 
CRF 
ADD 0001
ROL_F_ACC 
STA 0004
HALT 
```

### Program structure

A program is divided into three clearly differentiated sections, each of them separated by the `@` character.

```terminal
<variables declaration>
@
<start dir>
@
<sentences>
```

### Repertoire structure

A `repertoire` of instructions is the set of instructions that can be in the SiCoMe program.

#### Repertoire example

```terminal
$
CB 4000100
CB 0201100
CB 3000300
$
HALT false 0
CRA false 8200
CTA false 10200
ITA false 18200
CRF false B0200
CTF false B8200
SFZ false 400 200200
SFZ_R false 200500
ROR_F_ACC false 38200
ROL_F_ACC false 30200
ADD true 8000100 1100 28200
ADDI true 8000100 1100 8000100 1100 28200
STA true 8000100 2100 1000200
JMP true 400200
JMPI true 8000100 1100 400200
CSR true 8000100 403100 1000100 200200
CSR_R true 8000100 403100 1200200
ISZ true 8000100 1100 4100 1000100 600 200200
ISZ_R true 8000100 1100 4100 1000100 200700
```

The structure of a repertoire is like this:

```terminal
$
<microprogram section> (Not implemented)
$
<valid instructions>
```

## Setup 💻

If you want to colaborate to add new features to the project you must do this simples steps.

### Prerequisites

- You must have Rust installed with cargo: <https://www.rust-lang.org/es/tools/install>

### Using cargo

```bash
git clone git@github.com:Pacatro/sicompiler.git
cd sicompiler
cargo build
```

## TODOs 🏁

- [ ] Write tests
- [ ] Check how the microprogram part of repertoires works
- [ ] Write some new docs

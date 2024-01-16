# SiCompiler v0.1

This is a compiler that process files for be using in the SiCoMe simulator realized by the University of Cordoba (UCO), also check for syntax errors in your program.

> [!WARNING]
> These project is not complete, don't hesitate to report any bug you find

## 🛠️ Instalation

### Using cargo

You must have [Rust](https://www.rust-lang.org/es/tools/install) installed with rustup:

```terminal
cargo install sicompiler
```

## :pencil2: How to use

The only thing you need to do is write the next command in the directory you have the executable.

### Execution

```terminal
sicompiler [OPTIONS] <INPUT_PATH> [OUTPUT_PATH]
```

### Options

```terminal
-r, --rep <REPERTOIRE_PATH>  The repertoire of instructions
-h, --help                   Print help
-V, --version                Print version
```

The result will be an output file with the path you write or a default path in the same directory you are.

### Input

```termial
0 0003  ;Number 1
1 0003  ;Number 2
3 0000  ;Resul

*** This is a 
multi-line comment ***

@
6
@

CRA
CRF
ADD 0001
ROR_F_ACC
SFZ
JMP 12  ;F!=0

;-----F==0-----
CRA
CRF
ADD 0000
ROL_F_ACC
STA 0004
HALT

;-----F!=0-----
CRA
CRF
ADD 0001
ROL_F_ACC
STA 0004
HALT
```

### Output

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

## 📚 Standars

> [!IMPORTANT]
> These standards are subject to updates.

This compiler meets some standars that have been written resently.

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

### Program structure

A program is divided into three clearly differentiated sections, each of them separated by the `@` character.

```terminal
<variables declaration>
@
<start dir>
@
<sentences>
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

### Repertoire structure

A `repertoire` of instructions is the set of instructions that can be in the SiCoMe program.

The structure of a repertoire is like this:

```terminal
$
<microprogram section> (Not implemented)
$
<valid instructions>
```

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

### Error cases

- The program does not follow the structure defined by the standard.
- The instructions in the program are not defined in the repertoire.
- The instructions does not have the correct number of parameters.
- The parameters of the instructions are not in hex base.

## 📑 Libraries used

- [Clap](https://crates.io/crates/clap)

## 💻 Development

If you want to colaborate and add new features to the project you must do this simples steps.

### Colaborate

```bash
git clone git@github.com:Pacatro/sicompiler.git
cd sicompiler
cargo build
```

## 🔑 License

[MIT](https://opensource.org/license/mit/) - Created by **P4k0**.

## TODOs 🏁

- [x] Refactoring
- [x] Custom Errors
- [ ] Write tests
- [ ] Figure out how many params can have each instruction
- [ ] Write some new docs

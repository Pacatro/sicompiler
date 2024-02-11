# SiCompiler

This is a simple project that check for syntax errors in your SiCoMe program.

You can see this project also in [Crates.io](https://crates.io/crates/sicompiler)

## 🛠️ Instalation

### Using cargo

You must have [Rust](https://www.rust-lang.org/es/tools/install) installed:

```terminal
cargo install sicompiler
```

## ✏️ How to use

To use sicompiler you need to have a `.txt` file where you write your program and a `.rep` file with the valid instructions.

### Execution

```terminal
sicompiler --rep <REPERTOIRE_PATH> <INPUT_PATH>
```

### Arguments

```terminal
<INPUT_PATH>   The input path to compile
<REPERTOIRE_PATH> The repertoire of instructions
[OUTPUT_PATH]  The output path to write to [default: out.txt]
```

### Options

```terminal
-o, --out <OUTPUT_PATH>      The output path to write to [default: out.txt]
-r, --rep <REPERTOIRE_PATH>  The repertoire of instructions
-h, --help                   Print help
-V, --version                Print version
```

The result will be an output file with the path you write or a default path in the same directory you are.

### Input

```termial
0 0003  #Number 1
1 0003  #Number 2
3 0000  #Result

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
JMP 12  #F!=0

#-----F==0-----
CRA
CRF
ADD 0000
ROL_F_ACC
STA 0004
HALT

#-----F!=0-----
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

The comments in one line will be specified by a `#`:

```terminal
CRA 23 #One line comment
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

### Repertoire structure

A `repertoire` of instructions is the set of instructions that can be in the SiCoMe program.

The structure of a repertoire is like this:

```terminal
$
<microprogram section> (Not implemented)
$
<valid instructions>
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

### Tests

There are unit tests for the principals structs `Tokenizer` and `Validator` and only two integration tests.

To execute tests you only need to run the following command:

```bash
cargo test
```

## 🔑 License

[MIT](https://opensource.org/license/mit/) - Created by **P4k0**.

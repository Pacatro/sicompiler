# SiCompiler v0.1

This is a simple compiler that prepare files for be using in SiCoMe simulator realized by the University of Cordoba (UCO).

## How to use :pencil2:

The only thing you need to do is write the next command in the directory you have the executable

```bash
./sc <input_file_dir> <output_file_dir>
```

The result will be an output file with the name you write in the same directory you are.

## Standars :books:

> [!NOTE]
> This stardars will be updates.

This project is not complete, so is possible that it may be update in the future, but in this momment this compiler meets some standars that have been written resently.

### Comments

The comments in one line will be specified by a semicolon `;`:

```bash
CRA 23 ;One line comment
```

### Structure

A program is divided into three clearly differentiated sections, each of them separated by the `@` character.

```bash
<variables declaration>
@
<start dir>
@
<sentences>
@
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

## Error cases :warning:

- If the sequence of tokens does not contain the 'HALT' instruction.
- If any instruction in the sequence is not a valid predefined token.
- If the number of parameters for any instruction does not match the expected number.
- If the parameter is not in hexadecimal base.
- If there are issues while writing the validated tokens to the output file.

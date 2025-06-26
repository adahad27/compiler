This project was started on June 7th 2025.

The goal of this project is to create a compiler in Rust that is capable of compiling a subset of C.
This subset is called <C (pronounced less-than C).
The compiler will be responsible for compiling <C into x86 assembly, more specifically to Intel syntax.
Intel syntax was chosen because this project uses NASM for assembling which employs Intel syntax. This project also uses GNU's own linker for linking.
More updates to the goal will be added as the scope of the project becomes clear.

Features of <C (either already implemented or planned):
1. Support for int, bool, and char primitive types
2. Support for arithmetic, boolean, and relational operators
3. Support for conditional and loop statements
4. Function declarations, definitions, and calls


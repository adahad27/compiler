This project was started on June 7th 2025.

The goal of this project is to create a compiler in Rust that is capable of compiling a subset of C.
This subset is called <C (pronounced less-than C).
The compiler will be responsible for compiling <C into x86 assembly, more specifically to Intel syntax.
Intel syntax was chosen because this project uses NASM for assembling which employs Intel syntax. This project also uses GNU's own linker for linking.
More updates to the goal will be added as the scope of the project becomes clear.

Implemented features of <C:
1. Support for int, bool
2. Support for arithmetic, boolean, and relational operators
3. Support for conditional and loop statements
4.Function declarations, definitions, and calls

Planned features of <C:
1. Support for char primitive type
2. Arrays (Stack allocated)

Compiler Specifics:
This compiler uses an LL(1) Recursive Descent Parser to create an Abstract Syntax Tree. For now, there is no conversion to an Intermediate Representation.
It uses a Post-Order Traversal of the AST to generate the necesary x86 ASM. For register allocation, Linear Scanning will be implemented first, Graph Coloring is being considered.
For name resolution and scoping similar to C, the compiler uses a doubly linked tree.
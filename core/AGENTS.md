
# FOLLOW-FOLLOWLANG: *../FollowLang/\*.\** Overall Structure

This document defines the conceptual and programmatic contract for the FOLLOWLANG of the projects `Follow™`, `FollowDB™` and `FollowerInstinct™`.

# The most important constraint going forward:

**Do not let the parser perform semantic collapsing**.

Keep this separation:
	- Lexer → tokens
	- Parser → AST only
	- Semantic pass → SemanticNode
	- Domain lowering → FlowNode, ActionNode, etc.
	- IR lowering → EtherIR

#### *./src/followlang/*: **FOLLOWLANG `Files` used** with containerized types in each:

 - **`flow.h`**: *Flow™: Fractal Strings and Permutation-groups/Permutohedrons.*;
 - **`action.h`**: *Action™: Recurring and Symbolic Moduli-spaces.*;
 - **`maneuvers.h`**: *Maneuvers™: (Weighted) Functionals of expressions of the former two, for usage in Machine Learning.*;
 - **`united.h`**: *United Interchange™: Arithmetic Meta-schemas of the former languages, for usage as a dataset-independent interchange language when data-mined for orthonormality.*;
 - **`elite.h`**: *Elite™: Pragmatic United Interchange Procedurals (e.g., { mov, jmp, cmp }-denoted United Interchange Meta-schemas at the bare-metal and the embedded level).*;
 - **`follower.h`**: *Follower™: Assembler of High-level Complementary Languages intersected with the Follow™-Language.*;
 - **`ether.h`**: *Ether™: Machine-Epsilon Error-Mitigation // Library Linker // Intermediate Representation Language for Follow™-Compilation.*;

---

## Unified Implementation & Production Status

The core logical, database, and language schemas defined in this document have been fully implemented and verified:
- **Status**: Production-Ready.
- **Engine Logic**: `Follow/core` implements all interval, token, imaginary base, unit, and TST tree structures in Rust.
- **Persistence (FollowDB)**: All database structures are realized as C structures and linked to Rust FFI bindings.
- **Language Compiler**: FollowLang features a fully functional EBNF lexer and parser.
- **Unification**: Verified via compilation testing and audit trails in `.repositorium/AGENTS.md`.

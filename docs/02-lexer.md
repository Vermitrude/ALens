# Lexer Iteration Notes (Post v1.0)

The current lexer (v1.0) successfully tokenizes a useful subset of Solidity and serves as a solid foundation for the parser. Before considering the lexer production-ready, the following improvements should be implemented.

---

## 1. Refactor Keyword Lookup

### Current State

Keywords are recognized using a large `match` statement inside `next_token()`.

Example:

```rust
match ident.as_str() {
    "contract" => Token::Contract,
    "function" => Token::Function,
    "pragma" => Token::Pragma,
    ...
}
```

As more Solidity keywords are added, this section will continue to grow, making `next_token()` increasingly difficult to read and maintain.

### Planned Improvement

Extract keyword recognition into a dedicated function, for example:

```rust
fn lookup_identifier(identifier: &str) -> Token
```

The lexer will simply read an identifier and delegate keyword recognition to this helper.

Benefits:

* Keeps `next_token()` clean and focused.
* Makes adding new keywords easier.
* Improves readability.
* Follows the architecture used in many production lexers.

---

## 2. Add Comment Support

### Current State

The lexer treats `/` only as the division operator.

As a result, comments are tokenized incorrectly.

Example:

```solidity
// This is a comment
```

is currently interpreted as:

* Divide
* Divide
* Identifier("This")
* Identifier("is")
* Identifier("a")
* Identifier("comment")

instead of being ignored.

### Planned Improvement

Support both Solidity comment styles:

Single-line comments:

```solidity
// comment
```

Multi-line comments:

```solidity
/*
    comment
*/
```

The lexer should skip comment contents entirely and continue lexing the next valid token.

Benefits:

* Correctly handles real Solidity source files.
* Prevents comments from generating invalid tokens.
* Brings the lexer closer to production quality.

---

## 3. Expand Solidity Language Coverage

### Current State

The lexer currently recognizes a limited subset of Solidity keywords and operators.

### Planned Improvement

Gradually extend support to include additional Solidity syntax, including (but not limited to):

Keywords

* address
* bool
* string
* bytes
* uint
* int
* mapping
* struct
* enum
* event
* modifier
* constructor
* emit
* memory
* storage
* calldata
* immutable
* constant
* override
* virtual
* abstract
* interface
* library
* receive
* fallback

Operators

* &&
* ||
* !
* ++

---

* +=
* -=
* *=
* /=
* %=

Additional punctuation

* [
* ]
* ?

Additional literal forms

* hexadecimal literals
* scientific notation (if needed)
* escaped string literals

Benefits:

* Supports a much larger subset of Solidity.
* Produces more accurate token streams.
* Prepares the lexer for parsing real-world smart contracts.

---

## Future Goal

The long-term objective is for the lexer to tokenize arbitrary Solidity source files with high accuracy while remaining modular, maintainable, and easy to extend. Each iteration should improve correctness, readability, and support for additional language features without significantly increasing the complexity of `next_token()`.

07-07-2026
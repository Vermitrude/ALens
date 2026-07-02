## 1 Why does ALens break? — Because it's parsing lines instead of the language. 
-- The current parser processes Solidity source code one line at a time and relies on simple string matching. This approach breaks when functions, variables, or declarations span multiple lines, contain comments, or use different formatting. Since it does not understand Solidity's syntax, it cannot reliably parse real-world contracts.

## 2 What assumptions is ALens making? — Because it's assuming a very specific formatting and contract style.
-- The parser assumes that important language constructs appear on a single line, that keywords are always formatted consistently, and that names can be extracted by splitting lines on whitespace. It also assumes that only specific function names (such as `transfer` and `withdraw`) and state variables (such as `owner` and `balances`) are relevant, making it unsuitable for general Solidity analysis.

## 3 What parser architecture would scale better?

Solidity Source Code
        │
        ▼
      Lexer
(Tokenizes the source code)
        │
        ▼
      Parser
(Builds an Abstract Syntax Tree)
        │
        ▼
 Semantic Analyzer
(Resolves symbols, scopes, inheritance, types)
        │
        ▼
 Audit Engine
(Runs independent security rules)
        │
        ▼
 Report Generator
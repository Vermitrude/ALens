Why does ALens break? — Because it's parsing lines instead of the language.
What assumptions is ALens making? — Because it's assuming a very specific formatting and contract style.
What parser architecture would scale better?

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
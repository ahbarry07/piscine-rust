# Piscine Rust

> An intensive one-month immersion into Rust — one of the most challenging and rewarding systems programming languages in modern software engineering.

![Status](https://img.shields.io/badge/status-succeeded-brightgreen)
![Language](https://img.shields.io/badge/language-Rust-000000?logo=rust&logoColor=white)
![Period](https://img.shields.io/badge/period-Apr%202024%20–%20May%202024-lightgrey)

---

## Overview

The Rust Piscine ran from **April 22 to May 20, 2024** and was successfully completed. This piscine introduced one of the most demanding programming paradigms in modern systems development — Rust's ownership model, borrow checker, and type system — through a rigorous series of exercises and formal exams.

Rust is known for its steep learning curve. Completing this piscine required not only learning new syntax, but fundamentally rethinking how memory, data ownership, and safety are managed at the language level.

---

## Curriculum Map

| Module | Focus |
|--------|-------|
| **DATA** | Primitive types, collections, data structures |
| **VARIABLES** | Mutability, shadowing, type inference |
| **MEMORY** | Ownership model, stack vs heap, `Clone` vs `Copy` |
| **ERRORS** | `Result`, `Option`, error propagation with `?` |
| **LIFETIMES** | Lifetime annotations, borrow checker rules |
| **POINTERS** | References, smart pointers (`Box`, `Rc`, `RefCell`) |
| **CREATES** | Modules, crates, and the Cargo ecosystem |
| **PATTERNS** | Pattern matching, destructuring, `match` expressions |
| **FEATURES** | Iterators, closures, functional programming in Rust |
| **TRAITS** | Trait definitions, implementations, generics |
| **DRAWING** | Bonus project — graphical rendering |
| **CHAIKIN** | Bonus project — Chaikin curve algorithm |
| **ROAD_INTERSECTION** | Bonus project — simulation of road intersection logic |

### Formal Examinations
- **Exam 01-RUST** — Fundamentals assessment
- **Exam 02-RUST** — Intermediate concepts
- **Exam 03-RUST** — Advanced topics
- **Final Exam-RUST** — Comprehensive evaluation

---

## What I Learned

### Ownership & Memory Safety
- Rust's ownership system: every value has a single owner
- Move semantics vs `Copy` types
- Borrowing rules: immutable (`&T`) and mutable (`&mut T`) references
- Preventing data races and dangling pointers at compile time

### Type System & Generics
- Strong static typing with type inference
- Generic functions and structs
- Trait bounds and where clauses
- Lifetime parameters for references

### Error Handling
- `Result<T, E>` and `Option<T>` as first-class types
- The `?` operator for clean error propagation
- Custom error types and trait implementations

### Patterns & Functional Style
- Exhaustive `match` expressions
- Destructuring of structs, enums, and tuples
- Iterator adapters (`map`, `filter`, `fold`, `chain`)
- Closures and their capture semantics

### Smart Pointers
- `Box<T>` for heap allocation
- `Rc<T>` for shared ownership
- `RefCell<T>` for interior mutability

---

## Skills Developed

| Area | Details |
|------|---------|
| Language | Rust (2021 Edition) |
| Build Tool | Cargo |
| Paradigms | Systems programming, functional, type-driven |
| Key Concepts | Ownership, lifetimes, traits, pattern matching |
| Safety | Memory safety without garbage collection |

---

## Intensity

Rust is consistently ranked among the most difficult languages to learn, and this piscine reflected that reality. The borrow checker rejects code that would compile in any other language, requiring a precise mental model of memory at all times. Four formal exams throughout the month validated progressive mastery of the language.

---

*Completed as part of the Zone01 Dakar curriculum.*

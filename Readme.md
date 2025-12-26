# Axiom

**My systems data structures for database internals**

---

## Mission Statement

Axiom is my ongoing effort to design, implement, and reason about the core data structures that underpin modern database
systems. This repository focuses on correctness, performance, and clear invariants, treating each data structure as a
fundamental system component rather than an isolated algorithm.

---

## Overview

This repository studies **data structures as they appear in real database systems**—inside storage engines, buffer
managers, logging subsystems, and indexes.  
The goal is not breadth, but depth: understanding *why* each structure exists, *what invariants it maintains*, and *how
it behaves under real systems constraints*.

---

## Scope

Axiom covers data structures commonly found in database internals, including but not limited to:

- Tree-based indexes (e.g. B+Tree variants)
- Log-structured data structures (LSM components)
- In-memory indexing and memtables
- Page-based storage layouts
- Probabilistic data structures (Bloom filters, sketches)
- Supporting structures used by buffer pools and WAL subsystems
- Skip lists, LinkedList and more

Each structure is implemented with attention to **memory layout**, **I/O behavior**, and **failure considerations**.

---

## Design Principles

This repository follows a systems-engineering mindset:

- **Correctness first**  
  Every data structure is defined by explicit invariants.

- **Performance awareness**  
  Cache behavior, memory access patterns, and I/O costs are considered.

- **Realistic constraints**  
  Designs reflect page sizes, write amplification, and durability boundaries.

- **Minimal abstraction**  
  Implementations avoid unnecessary indirection and hidden costs.

---

## What This Is Not

- Not an interview-preparation DSA repository
- Not a collection of toy implementations
- Not framework-driven or application-level code

Axiom is intentionally **low-level and systems-focused**.

---

## Repository Structure (Evolving)

The layout mirrors database subsystems rather than textbook categories:


---

## Language Choice

All implementations are written in **Rust**, chosen for:

- Explicit ownership and memory safety
- Predictable performance characteristics
- Suitability for systems-level programming

---

## Status

Axiom is a **living repository**.  
Designs evolve as understanding deepens and trade-offs are explored.

Correctness, clarity, and learning are prioritized over completeness.





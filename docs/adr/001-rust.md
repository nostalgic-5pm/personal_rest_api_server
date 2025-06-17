# 001 – Decision to Use Rust as the Main Programming Language for This Server

## Status
Accepted

## Context
I need to decide which programming language to use for this server.

## Considerations
I thought about this from a few perspectives:
・The language should be modern.
・It should be a good fit for server-side development.
・It should offer something unique that Python, Java, and JavaScript (Current Technology Stack) don't have.

I considered several modern server-side languages:
・Go
・TypeScript/Node.js
・Python
・Java
・Rust

## Decision
I'm going to use Rust as the main programming language for this server.

> The language should be modern.
→ Rust is a modern language, created in the 2010s, and its ecosystem is actively growing.

> The language should be suitable for server development.
→ Rust is fast, safe, and has a growing number of libraries (crates) for building web apps and servers.

> The language should offer something unique compared to my current technology stack.
→ Rust's memory safety without a garbage collector, its strict compiler, and zero-cost abstractions set it apart from my current tech stack.

This project is also meant to be a learning experience, and I believe using Rust will let me learn concepts and patterns not found in my current tech stack.

## Consequences
I'll need to learn Rust's ownership model and async patterns, but this should also help me write safer and more efficient code overall.

## Reference

## Notes
Owner: nostalgic.5pm
Proposed date: 2025-06-01
Last updated: 2025-06-01

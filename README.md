# Libellus - Performant Single File Order Books

A library for optimized in-memory general purpose order books. Generally, order books have no "silver bullet". therefore more multiple implementations are provided.

## StackBook
StackBook is a no-alloc stack backed order book, with the aim of extreme cache friendliness. all of the methods are explicitly inlined as well. Orders are inserted by shifting in a fixed sized array.

## TreeBook

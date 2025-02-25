# Libellus - Performant Single File Order Books

A library for optimized in-memory general purpose order books.
The book is a no-alloc stack backed order book, with the aim of extreme cache friendliness. all of the methods are explicitly inlined as well. Orders are inserted by shifting a fixed sized array, memory access is continious.


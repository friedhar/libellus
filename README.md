# Libellus - Performant Single File Order Books

A library for optimized in-memory general purpose order books.
The book is a no-alloc stack backed order book, with the aim of extreme cache friendliness. all of the methods are explicitly inlined as well. Orders are inserted by shifting a fixed sized array, memory access is continious.

## Benchmark
### Host Machine:
```
Model Name:	MacBook Pro
Chip:	Apple M4
Total Number of Cores:	10 (4 performance and 6 efficiency)
Memory:	16 GB
```

### Isolated Place(s)
```
iterations=10000, sample=1000, mean(place/s): 3_309_155.66
```

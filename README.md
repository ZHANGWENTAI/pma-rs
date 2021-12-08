# pma-rs
Packed Memory Array in Rust.

PMA is a data stuctrue that can maintain a batch of sorted data and supports query, insert and delete. Normal static arrays will double or truncate when they run out of space or are redundant, which can cause a sudden performance drop in a single operation. PMA, on the other hand, segments the normal array, always keeping some gap empty in each segment, so that when each segment is full, only a small data movement occurs.
For detail, take a look at **Chapter 5.1** in http://erikdemaine.org/papers/BRICS2002/paper.pdf please.

Playing around with Rust for the first time by implementing a toy regex matching engine. Referenced [Russ Cox's text](https://swtch.com/~rsc/regexp/regexp1.html).

I've added support for the following operations:
- Concat
- Alternation
- Star

For the future, I want to add:
- support for the
    - zero or one operation
    - one or more operation
- IO support for large files on disk
- parallelism for efficient processing of large sets of files
- unit tests
- a way to coalesce chains of epsilon transitions and clean up state

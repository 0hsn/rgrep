# rgrep
grep unitlity in Rust. Excercise of 
Rust Book Ch12 https://doc.rust-lang.org/book/ch12-00-an-io-project.html

This tool is tested with the [works of Shakespeare](https://ocw.mit.edu/ans7870/6/6.006/s08/lecturenotes/files/t8.shakespeare.txt).

Usage:
```bash
rgerp [pattern] [FILE]
```
where 
- `pattern` is a string, can be multiple words. Regular expression based 
pattern not supported.

- `FILE` is a valid file name in your system. 
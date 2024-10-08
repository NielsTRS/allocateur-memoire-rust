# allocateur-memoire-rust
## Build
```bash
$ cargo build
```
Make an executable located in : ./target/debug/allocateur

## Run the program
```bash
$ cargo run
```
This command rebuild and run the program

## Test
```bash
$ cargo test
```
Launch all the tests in the "tests" folder

To run a specific test : 
```bash
$ cargo test --test name_test
```

To print the output : 

```bash
$ cargo test --test name_test -- --nocapture
```

## Debug
Once the code is build using cargo, we can run gdb : 
```bash
$ gdb ./target/debug/allocateur
```
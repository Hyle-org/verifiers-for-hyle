This is a midenVM verifier implementation.

In order to verify midenVM proofs you'd need program_hash, proof_file path, stack_input path and stack_output path.

In order to get these files you'd need to install miden-vm and generate them based on your program.masm file.

Check https://0xpolygonmiden.github.io/miden-vm/intro/usage.html#running-miden-vm for instructions.

File helpers.rs is inspired/copied from core midenVM cli verifier implementation since those methods are not exposed by midenVM crate.

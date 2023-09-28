#[cfg(all(feature = "x8664", feature = "riscv"))]
compile_error!("feature \"x8664\" and feature \"riscv\" cannot be enabled at the same time");

#[cfg(feature = "x8664")]
pub mod constants {
    pub const ASSEMBLE_CMD: &str = "as submit.s -o tmp.o";
    pub const LINK_CMD: &str = "gcc -v -static -no-pie tmp.o -o test_target";
    pub const EXEC_CMD: &str = "./test_target";
}

#[cfg(feature = "riscv")]
pub mod constants {
    pub const ASSEMBLE_CMD: &str = "riscv64-unknown-elf-as submit.s -o tmp.o";
    pub const LINK_CMD: &str = "riscv64-unknown-elf-gcc -v -static -no-pie tmp.o -o test_target";
    pub const EXEC_CMD: &str = "spike --isa=rv64imac pk ./test_target";
}

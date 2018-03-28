# Bare minimal avr-rust build

The goal is to create an object file (just a ".o" embedded inside of rust's .rlib) that can be linked into a larger C project with avr-gcc

Unfortunately, with the following command:

	XARGO_RUST_SRC=/home/chris/other/avr/rust/src xargo build --target avr-unknown-unknown -p core

I get:

	[chris@manaslu avr-rust-test (master ✗)]$ XARGO_RUST_SRC=/home/chris/other/avr/rust/src xargo build --target avr-unknown-unknown -p core
	   Compiling core v0.0.0 (file:///home/chris/other/avr/rust/src/libcore)
	rustc: /home/chris/other/avr/rust/src/llvm/lib/CodeGen/MachineBasicBlock.cpp:1299: llvm::MachineBasicBlock::livein_iterator llvm::MachineBasicBlock::livein_begin() const: Assertion `getParent()->getProperties().hasProperty( MachineFunctionProperties::Property::TracksLiveness) && "Liveness information is accurate"' failed.
	error: Could not compile `core`.

	To learn more, run the command again with --verbose.
	error: `"cargo" "build" "--release" "--manifest-path" "/tmp/xargo.7yrOdO7599xh/Cargo.toml" "--target" "avr-unknown-unknown" "-p" "core"` failed with exit code: Some(101)
	stack backtrace:
	   0:     0x55a46f88d47c - backtrace::backtrace::trace::h72a3b2237eb3f313
	   1:     0x55a46f88d4b2 - backtrace::capture::Backtrace::new::h951ad984e98f7447
	   2:     0x55a46f86ceb4 - error_chain::make_backtrace::h68338ab6585fd6b9
	   3:     0x55a46f86cf84 - <error_chain::State as core::default::Default>::default::hdbddd7e773c213e5
	   4:     0x55a46f85c52d - xargo::sysroot::build::hb9a6a23e5743209d
	   5:     0x55a46f85f280 - xargo::sysroot::update::h5e2fafc45c90b342
	   6:     0x55a46f86b238 - xargo::run::hc479b5cf1af21d8c
	   7:     0x55a46f867b8d - xargo::main::he84301bae97ab298
	   8:     0x55a46f8c1c1c - panic_unwind::__rust_maybe_catch_panic
							at /home/chris/other/avr/rust/src/libpanic_unwind/lib.rs:99
	   9:     0x55a46f8bb03e - std::panicking::try<(),closure>
							at /home/chris/other/avr/rust/src/libstd/panicking.rs:459
							 - std::panic::catch_unwind<closure,()>
							at /home/chris/other/avr/rust/src/libstd/panic.rs:361
							 - std::rt::lang_start
							at /home/chris/other/avr/rust/src/libstd/rt.rs:59
	  10:     0x7f6181ade82f - __libc_start_main
	  11:     0x55a46f8487a8 - _start
	  12:                0x0 - <unknown>


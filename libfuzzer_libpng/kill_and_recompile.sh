#/bin/zsh

pkill -9 -f  ./fuzzer_libpng
rm libafl_unix_shmem_server 
cargo build
./target/debug/libafl_cxx ./harness.cc libpng-1.6.37/.libs/libpng16.a -I libpng-1.6.37/ -o fuzzer_libpng -lz -lm
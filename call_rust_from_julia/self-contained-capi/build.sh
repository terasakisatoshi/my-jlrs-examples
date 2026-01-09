export JULIA_DIR=`julia -e 'print(dirname(Sys.BINDIR))'`
export JLRS_JULIA_DIR=${JULIA_DIR}
export DYLD_LIBRARY_PATH=${JULIA_DIR}/lib:${DYLD_LIBRARY_PATH}
cargo run --release

export JULIA_DIR=`julia -e 'print(dirname(Sys.BINDIR))'`
export DYLD_LIBRARY_PATH=${JULIA_DIR}/lib:${DYLD_LIBRARY_PATH}
# julia version 1.11.4 -> julia-1-11
feature=$(echo `julia --version` | sed -E 's/^julia version ([0-9]+)\.([0-9]+)\.[0-9]+$/julia-\1-\2/')
cargo run --features ${feature} --release

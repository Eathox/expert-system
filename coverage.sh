#!/bin/sh
list_test_executables() {
    for file in $( \
        RUSTFLAGS="-C instrument-coverage" cargo test --no-run --message-format=json \
            | jq -r "select(.profile.test == true) | .filenames[]" \
            | grep -v dSYM - \
    ); do
        printf "%s %s " "-object" "$file";
    done
}

find . -name "expert_system*.prof*" -delete

RUSTFLAGS="-C instrument-coverage" LLVM_PROFILE_FILE="expert_system-%m.profraw" \
    cargo test --tests

DATA_FILE="expert_system.profdata"
rust-profdata merge -sparse expert_system-*.profraw -o $DATA_FILE

rust-cov report -ignore-filename-regex='/.cargo/registry' \
    -object target/debug/expert_system $(list_test_executables) \
    -instr-profile=$DATA_FILE

# rust-cov show -ignore-filename-regex='/.cargo/registry' \
#     -object target/debug/expert_system $(list_test_executables) \
#     -instr-profile=$DATA_FILE -show-branches=count \
#     -show-instantiations -show-line-counts-or-regions \
#     -format=html > coverage.html

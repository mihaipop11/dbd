#!/bin/sh

cd "$(dirname "$0")"/..

rm dbd-*.profraw

export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="dbd-%p-%m.profraw"

cargo test

mkdir target/debug/coverage

grcov . -s src --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing --ignore "*cargo*" -o ./target/debug/coverage/lcov.info 
genhtml -o ./target/debug/coverage/ --show-details --highlight --ignore-errors source --legend --rc lcov_branch_coverage=1 ./target/debug/coverage/lcov.info
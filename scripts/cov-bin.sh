#!/bin/sh

cd "$(dirname "$0")"/..

rm dbd-*.profraw

export RUSTC_BOOTSTRAP=1
export CARGO_INCREMENTAL=0
export RUSTFLAGS="-Cinstrument-coverage -Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort"
export RUSTDOCFLAGS="-Cpanic=abort"
export LLVM_PROFILE_FILE="dbd-%p-%m.profraw"

cargo test

mkdir target/debug/coverage

grcov . -s src --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing --ignore "*cargo*" --ignore ./target/debug/deps -o ./target/debug/coverage/lcov.info 
genhtml -o ./target/debug/coverage/ --show-details --highlight --ignore-errors source --legend --rc lcov_branch_coverage=1 ./target/debug/coverage/lcov.info
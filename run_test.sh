#!/bin/sh
RUSTFLAGS="-A warnings" cargo test -- --nocapture $1

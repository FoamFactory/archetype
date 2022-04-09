#!/bin/bash

if diesel migration pending > /dev/null 2>&1; then
    RUST_BACKTRACE=1 archetype
else
    diesel migration run && RUST_BACKTRACE=1 archetype
fi

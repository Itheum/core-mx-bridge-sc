#!/bin/bash

cargo llvm-cov --ignore-filename-regex '(storage.rs|events.rs|macros|errors.rs)'  --open
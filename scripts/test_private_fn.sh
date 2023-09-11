#!/bin/bash

# Only execute function with pattern `test_private*`
RUST_LOG=INFO cargo test --package crabket-server test_private
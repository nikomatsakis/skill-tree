#!/bin/bash
cargo install --vers ^0.3 mdbook --debug
cargo install -f --path mdbook-skill-tree --debug
cd book
mdbook-skill-tree install
mdbook build

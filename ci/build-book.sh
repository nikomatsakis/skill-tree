#!/bin/bash
cargo install -f --path mdbook-skill-tree
cd book
mdbook-skill-tree install
mdbook build

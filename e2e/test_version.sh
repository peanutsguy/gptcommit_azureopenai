#!/bin/sh
set -eu

gptcommit_azureopenai --version
# assert matches version in Cargo.toml

gptcommit_azureopenai --help


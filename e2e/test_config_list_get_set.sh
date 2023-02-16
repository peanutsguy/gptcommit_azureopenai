#!/bin/sh
set -eu

gptcommit_azureopenai config list
# assert is valid TOML

gptcommit_azureopenai config get openai.model
# assert default = text-davinci-003
gptcommit_azureopenai config set openai.model foo
gptcommit_azureopenai config get openai.model
# assert is foo

gptcommit_azureopenai config delete openai.model
gptcommit_azureopenai config get openai.model
# back to default

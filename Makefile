# 🐻‍❄️📦 charted_sdk: Rust SDK library for Noelware's Charts Platform
# Copyright (c) 2022-2023 Noelware Team <team@noelware.org>
#
# Permission is hereby granted, free of charge, to any person obtaining a copy
# of this software and associated documentation files (the "Software"), to deal
# in the Software without restriction, including without limitation the rights
# to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
# copies of the Software, and to permit persons to whom the Software is
# furnished to do so, subject to the following conditions:
#
# The above copyright notice and this permission notice shall be included in all
# copies or substantial portions of the Software.
#
# THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
# IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
# FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
# AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
# LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
# OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
# SOFTWARE.

# Usage: `make help`
.PHONY: help
help: ## Prints the help usage on the charted-rust-sdk
	@printf "\033[34m───────────────────────────────────────────────────────────────────────────────────────────────────\033[0m\n"
	@printf "Welcome to charted-rust-sdk. This Makefile is present to help you build and run the library tests."
	@printf "\n"
	@printf "\n:: Usage ::\n"
	@printf "make <target> [VARIABLE=value]\n"
	@printf "\n:: Targets ::\n"
	@awk 'BEGIN {FS = ":.*##"; } /^[a-zA-Z_-]+:.*?##/ { printf "  make \033[36m%-15s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 25) } ' $(MAKEFILE_LIST)

.PHONY: clippy
clippy: ## Runs the `cargo clippy` command
	@cargo clippy

.PHONY: clippy-fix
clippy-fix: ## Runs `cargo clippy` with the --fix, --allow-dirty, and --allow-staged flags.
	@cargo clippy --fix --allow-dirty --allow-staged

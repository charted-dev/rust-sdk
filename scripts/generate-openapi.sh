#!/bin/bash

# 🐻‍❄️📦 charted_sdk: Rust SDK library for Noelware's Charts Platform
# Copyright (c) 2022-2023 Noelware, LLC. <team@noelware.org>
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

print!() {
    echo $@
}

fatal!() {
    echo "[~+~ fatal ~+~]" $@
    exit 1
}

# check if we have Git, Java, and Docker. Since we are building this from source.
if ! command -v git &> /dev/null; then
    fatal! "Missing \`git\` on system."
fi

if ! command -v java &> /dev/null; then
    fatal! "Missing \`java\` on system."
fi

if ! command -v docker &> /dev/null; then
    fatal! "Missing \`docker\` on system."
fi

if ! command -v make &> /dev/null; then
    fatal! "Missing \`make\` on system."
fi

print! "Now pulling OpenAPI tools image..."
docker pull "openapitools/openapi-generator-cli:v6.3.0"

mkdir ./_work
print! "Cloning charted-server on main branch!"
git clone https://github.com/charted-dev/charted -b main ./_work &> /dev/null

print! "Building project..."
pushd ./_work
    make build
popd

print! "Generating OpenAPI from CLI..."
OPENAPI_FILE=$(cd ./_work/cli/build/install/charted && ./bin/charted openapi --format=json --openapi-version=3.0)

print! "Generating Rust client from OpenAPI..."
echo $OPENAPI_FILE > ./_work/openapi.json

docker run --rm --name charted-sdk-rust-openapi \
    -v "$(pwd):/local" -v "$(pwd)/_work:/_work" openapitools/openapi-generator-cli:v6.3.0 generate \
    -i /_work/openapi.json \
    -g rust \
    -o /local/out/rust \
    --skip-validate-spec

rm -rf ./_work
print! "Done~!"

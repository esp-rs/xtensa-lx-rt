#!/usr/bin/env bash

set -euxo pipefail

# remove existing blobs because otherwise this will append object files to the old blobs
rm -f bin/*.a


xtensa-esp32-elf-ar crs bin/xtensa_vectors.a bin/xtensa_vectors.o
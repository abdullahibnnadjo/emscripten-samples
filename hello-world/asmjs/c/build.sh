#!/bin/sh
set -ex
emcc main.c -o main.js -s ONLY_MY_CODE=1 -s NO_FILESYSTEM=1 -s NO_EXIT_RUNTIME=1 -s INVOKE_RUN=0 -s ABORTING_MALLOC=0 -s DISABLE_EXCEPTION_CATCHING=1 --memory-init-file 1 -s EXPORTED_RUNTIME_METHODS="[]" -s "EXPORT_NAME='My_Module'" -s MODULARIZE=1
{ set +x; } 2>/dev/null

#!/bin/sh
set -ex
emcc main.c -o main_release.js -s ONLY_MY_CODE=1 -s ASSERTIONS=0 -s NO_FILESYSTEM=1 -s NO_EXIT_RUNTIME=1 -s INVOKE_RUN=0 -s ABORTING_MALLOC=0 -s DISABLE_EXCEPTION_CATCHING=1 -s AGGRESSIVE_VARIABLE_ELIMINATION=1 --memory-init-file 1 -s EXPORTED_RUNTIME_METHODS="[]" -O3 --closure 1 -s "EXPORT_NAME='My_Module'" -s MODULARIZE=1
{ set +x; } 2>/dev/null

uglify_me(){
    for file in "$@"
    do
        set -x
        uglifyjs "$file" > "$file.min"
        gzip -9 -k "$file.min"
        { set +x; } 2>/dev/null
    done
}
uglify_me main_release.js main_release.asm.js

echo "main_release.asm.js.min is so small that the gzipped version is bigger! Not a big deal, let things how they are for this time"
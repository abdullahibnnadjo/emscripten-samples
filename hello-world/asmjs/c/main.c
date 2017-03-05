#include <stdio.h>
#include <emscripten.h>

/*
** Note : Here, we're not using any function like puts, printf, write...
** Those, used in a Emscripten context, imply that we would need to link to
** libc and that we can't use the "-s NO_FILESYSTEM=1" option of emcc
*/
int main() {
    EM_ASM({
        document.getElementById('log-console').textContent += 'Hello World!\n';
    });
    return 0;
}

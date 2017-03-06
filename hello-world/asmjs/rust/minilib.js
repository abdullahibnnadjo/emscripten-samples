mergeInto(LibraryManager.library, {
    // Helper functions
    log: function (str) {
        window.document.getElementById("log-console").textContent += str + "\n";
    },

    // Implement extern functions
    // Don't forget to add _ on helper functions
    my_log: function (ptr, len) {
        _log(Pointer_stringify(ptr, len));
    },

    // Make dep to method_support
    my_log__deps: ['log'],
});

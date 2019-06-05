const rust = import("./pkg/hello_world");

rust
    .then(m => m.greet("World!"))
    .catch(console.error);

A minimal demonstration how modules interact in Rust and how to call function from different modules.

```
src/
├── a
│   ├── fa1.rs -- called by fa2
│   ├── fa2.rs
│   └── mod.rs
├── b
│   ├── fb1.rs -- calls fa1
│   └── mod.rs
├── lib.rs
└── main.rs -- calls fb1
```
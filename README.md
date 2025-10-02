# Rust JNI Base🚀

A starter template for building Rust projects that interact with Java via JNI (Java Native Interface).
> **Note:** This template is dynamically loaded into the working process and designed for use only on the Windows platform.
## ✨ Features

- 🦀 Basic Rust setup for JNI integration
- 🔗 Example JNI functions callable from working JVM
- 🛠️ Build instructions for generating native libraries

## 🏁 Getting Started

### 📋 Prerequisites

- 🦀 [Rust](https://rust-lang.org)
- ☕ Java Development Kit (JDK)
- ⚙️ `cargo` build tool

### ⚡ Build

```sh
cargo build --release
```



For more details, see the [LICENSE](./LICENSE) file.
This will produce a native library in the `target/release` directory.

### 🚦 Usage

1. Use JNIEnv for finding class, calling methods etc.
2. Build your code.
3. Inject your DLL file using a DLL injector (like System Informer).

## 💡 Example

```rust
let example_class = env.find_class("java/lang/String");

if let Ok(_) = example_class {
    logger::info("Class found.");
} else {
    logger::error("Class could not found.");
}
```

## 📄 License

This project is licensed under the MIT License. This means you are free to use, modify, and distribute the code, provided that you include the original copyright and license notice in any copies or substantial portions of the software.
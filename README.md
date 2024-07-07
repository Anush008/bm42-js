<div align="center">
  <h1><a href="https://crates.io/crates/bm42">BM42-js ⚡️</a></h1>
 <h3>Typescript/NodeJS implementation of <a href="https://qdrant.tech/articles/bm42/" target="_blank">Qdrant BM42</a></h3>
  <a href="https://www.npmjs.com/package/bm42"><img src="https://img.shields.io/npm/v/bm42.svg" alt="npmjs.org"></a>
  <a href="https://github.com/Anush008/bm42-js/blob/main/LICENSE"><img src="https://img.shields.io/badge/license-apache-blue.svg" alt="Apache Licensed"></a>
</div>

## 🔍 Not looking for JS?

- Python 🐍: [BM42 with FastEmbed](https://github.com/qdrant/fastembed)
- Rust 🦀: [BM42-rs](https://github.com/Anush008/bm42-rs)

## 📥 Installation

Run the following command in your project directory:

```bash
npm install @anush008/bm42
```

## Platform Support

This library uses Rust bindings of https://github.com/Anush008/bm42-rs. Works for the following architectures.

- darwin-arm64
- darwin-x64
- linux-x64-gnu
- win32-x64-msvc

## 📖 Usage

### Generating Sparse Embeddings

```rust
import { BM42 } from "@anush008/bm42";

// With default options
let bm42 = new BM42();

// With custom options
let bm42 = new BM42({
  alpha: 0.5,
  showDownloadProgress: true,
  // others
});

let texts = [
    "It's a truth universally acknowledged that a zombie in possession of brains must be in want of more brains.",
    "We're not in Infinity; we're in the suburbs.",
    "I was a thousand times more evil than thou!",
    "History is merely a list of surprises... It can only prepare us to be surprised yet again.",
];

// Generate embeddings for indexing
let doc_embeddings = bm42.embed(texts);

// Generate embeddings for querying
let query_embeddings = bm42.queryEmbed(texts);
```

## 📄 LICENSE

Apache 2.0 © [2024](https://github.com/Anush008/bm42-js/blob/main/LICENSE)
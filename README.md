# 🧩 Geometric Shapes Drawing Project

<div style="display: flex; gap: 3em;">
<img src="instructions.png" alt="Example Output" width="400" height="300">
<p align="left">
    Welcome to the <em>Geometric Shapes Drawing</em> tool — a Rust-powered application that renders 2D and basic 3D shapes to PNG images. Whether you're exploring computer graphics or testing your Rust skills, this project offers a modular and extensible foundation.
</p>
</div>

---

## 📚 Table of Contents

- [🧩 Geometric Shapes Drawing Project](#-geometric-shapes-drawing-project)
  - [📚 Table of Contents](#-table-of-contents)
  - [🚀 Overview](#-overview)
  - [✨ Features](#-features)
  - [⚙️ Quick Start](#️-quick-start)
    - [1. Clone and Build](#1-clone-and-build)
    - [2. Run the Program.](#2-run-the-program)
  - [🧪 Usage](#-usage)
    - [💡 Default Run](#-default-run)
    - [🧪 Run with Debug Info](#-run-with-debug-info)
    - [🧹 Clean Build Files](#-clean-build-files)
    - [📦 Rebuild Everything](#-rebuild-everything)
  - [📁 Project Structure](#-project-structure)
  - [👥 Authors](#-authors)
  - [🤝 Contributing](#-contributing)
  - [📜 License](#-license)

---

## 🚀 Overview

This project allows you to render geometric shapes and export them as PNG images. It's written in **Rust** with a focus on:

- Clean modular code
- Random shape generation
- Trait-based extensibility
- Simple 3D projections for bonus content

---

## ✨ Features

- ✅ **Core Shapes**: Point, Line, Triangle, Rectangle, Circle  
- 🔷 **Bonus**: Pentagon, 3D Cube projection  
- 🎨 **Customizable**: Shape colors and dimensions  
- 🔄 **Randomizer**: Generate random shapes with ease  
- 📦 **Portable**: Run with simple terminal commands

---

## ⚙️ Quick Start

### 1. Clone and Build

```bash
git clone https://learn.zone01kisumu.ke/git/adaniel/drawing
cd drawing
make setup
```

### 2. Run the Program.

```sh
make run
```

- No need to remember long Cargo commands anymore — we've got a Makefile for that!

## 🧪 Usage

### 💡 Default Run

```sh
make run
```

### 🧪 Run with Debug Info

```sh
make debug
```

### 🧹 Clean Build Files

```sh
make clean
```

### 📦 Rebuild Everything

```sh
make rebuild
```

- The Makefile simplifies building, running, and cleaning up. Modify it to extend functionality as needed.

---

## 📁 Project Structure

```sh
.
├── src/
│   ├── main.rs
│   └── geometrical_shapes/
│       ├── mod.rs
│       ├── point.rs
│       ├── line.rs
│       ├── triangle.rs
│       ├── circle.rs
│       ├── polygon.rs
│       ├── pentagon.rs
│       └── cube.rs
├── Makefile
├── Cargo.toml
├── .gitignore
├── Instructions.md
├── instructions.png
└── README.md
```

## 👥 Authors

- Core Team:

    [rcaleb](https://github.com/Raymond9734) — Architecture & structure

    [Anxiel Ray](https://github.com/anxielray) — Shape logic

    [adaniel](https://github.com/adiozdaniel) — Geometry & rendering

## 🤝 Contributing

- *We welcome PRs!* 🚀

  1. Fork the repo

  2. Create a feature branch:
  git checkout -b feature/your-feature

  3. Commit changes:
  git commit -m "Add new feature"

  4. Push to your branch:
  git push origin feature/your-feature

  5. Submit a Pull Request 📝

- *Read the Contribution Guidelines for more.*

## 📜 License

- This project is licensed under the MIT License.
- See [LICENSE](LICENSE.md) for full details.

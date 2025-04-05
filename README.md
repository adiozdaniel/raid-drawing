# Geometric Shapes Drawing Project

![Example Output](instructions.png)

## Table of Contents

- [Geometric Shapes Drawing Project](#geometric-shapes-drawing-project)
  - [Table of Contents](#table-of-contents)
  - [Project Overview](#project-overview)
  - [Features](#features)
  - [Installation](#installation)
  - [Usage](#usage)
  - [Project Structure](#project-structure)
  - [Authors](#authors)
  - [Contributing](#contributing)
  - [License](#license)

## Project Overview

A Rust implementation for rendering geometric shapes to PNG images, featuring:

- Modular shape implementations
- Random shape generation
- Trait-based drawing system
- Bonus 3D shape support

## Features

✅ Core Shapes: Point, Line, Triangle, Rectangle, Circle  
⭐ Bonus: Pentagon, Cube projections  
🎨 Customizable colors and dimensions  
🔄 Random shape generation utilities  

## Installation

```bash
git https://learn.zone01kisumu.ke/git/adaniel/drawing
cd drawing
cargo build
```

## Usage

```rust
cargo run
```

## Project Structure

```sh
.
├── src/
│   ├── main.rs
│   └── geometrical_shapes/ # Shape implementations
│       ├── mod.rs         # Module declarations and trait definitions
│       ├── point.rs       # 2D point implementation
│       ├── line.rs        # Line segment between points
│       ├── triangle.rs    # Three-sided polygon
│       ├── circle.rs      # Circular shape with radius
│       ├── polygon.rs     # Base polygon functionality
│       ├── pentagon.rs    # Five-sided polygon (bonus)
│       └── cube.rs        # 3D cube projection (bonus)
├── .gitignore
├── Cargo.toml
├── Instructions.md
├── instructions.png
└── Readme.md
```

## Authors

**Core Team**:

- [rcaleb](https://github.com/) - Project architecture
- [Anxiel Ray](https://github.com/anxielray) - Shape algorithms
- [adaniel](https://github.com/) - Shape algorithms

## Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

See our [Contribution Guidelines](CONTRIBUTING.md) for details.

## License

- This project is licensed under the **MIT License** - see the [LICENSE.md](LICENSE.md) file for details.

---

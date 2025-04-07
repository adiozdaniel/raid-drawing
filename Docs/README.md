# 🧩 Geometric Shapes Drawing Project

<div style="display: flex; gap: 3em;">
<img src="media/instructions.png" alt="Example Output" width="400" height="300">
<p align="left">
    Welcome to the <em>Geometric Shapes Drawing</em> tool — a Rust-powered application that renders 2D and basic 3D shapes to PNG images. Whether you're exploring computer graphics or testing your Rust skills, this project offers a modular and extensible foundation. View the project instructions <a href="https://learn.zone01kisumu.ke/git/adaniel/drawing/src/branch/main/Docs/INSTRUCTIONS.md">Here</a> or <a href="INSTRUCTIONS.md" >here</a>. With the comprehensive details outlined in the instructions.md, you can seamlessly integrate the code into the `main.rs` file. Simply paste it as is, and your program will be ready to run flawlessly—no additional modifications required. Enjoy coding with ease and sophistication!
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
  - [🎨 Geometrical Shapes Renderer in Rust](#-geometrical-shapes-renderer-in-rust)
    - [📁 Project Structure](#-project-structure)
    - [📦 Dependencies Used](#-dependencies-used)
      - [1. raster](#1-raster)
      - [2. rand](#2-rand)
    - [🔧 How It Works](#-how-it-works)
    - [🧩 Traits](#-traits)
      - [Drawable](#drawable)
      - [Displayable](#displayable)
  - [Code](#code)
    - [📄 File: point.rs (in geometrical\_shapes module)](#-file-pointrs-in-geometrical_shapes-module)
      - [📘 `Point` Struct \& Trait Implementation Summary](#-point-struct--trait-implementation-summary)
    - [📄 File line.rs (in geometrical\_shapes module)](#-file-liners-in-geometrical_shapes-module)
      - [📦 What this file contributes to the project](#-what-this-file-contributes-to-the-project)
    - [📄 File triangle.rs (in geometrical\_shapes module)](#-file-trianglers-in-geometrical_shapes-module)
      - [Parameters](#parameters)
      - [Returns](#returns)
      - [random](#random)
      - [Parameters](#parameters-1)
      - [Returns](#returns-1)
    - [📄 File circle.rs (in geometrical\_shapes module)](#-file-circlers-in-geometrical_shapes-module)
      - [Functions](#functions)
      - [Parameters for circle::new()](#parameters-for-circlenew)
      - [Returns circle::new()](#returns-circlenew)
      - [Parameters for circle::random()](#parameters-for-circlerandom)
      - [Returns circle::random()](#returns-circlerandom)
    - [📄 File rectangle.rs (in geometric\_shapes module)](#-file-rectanglers-in-geometric_shapes-module)
      - [Functions for rectangle](#functions-for-rectangle)
      - [Parameters for rectangle::new()](#parameters-for-rectanglenew)
      - [Returns for rectangle::new()](#returns-for-rectanglenew)
      - [Parameters](#parameters-2)
      - [Returns](#returns-2)
    - [📄 File cubes.rs (in geometric\_shapes module)](#-file-cubesrs-in-geometric_shapes-module)
    - [📄 File pentagons.rs (in geometric\_shapes module)](#-file-pentagonsrs-in-geometric_shapes-module)
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

<div style="display: flex; gap: 11px;">
<div style="flex: 1">

### 💡 Default Run

```sh
make run
```

</div>  <div style="border-left: 1px solid #ccc; height: auto;"></div> <div style="flex: 1">

### 🧪 Run with Debug Info

```sh
make debug
```

</div>  <div style="border-left: 1px solid #ccc; height: auto;"></div> <div style="flex: 1">

### 🧹 Clean Build Files

```sh
make clean
```

</div>  <div style="border-left: 1px solid #ccc; height: auto;"></div> <div style="flex: 1">

### 📦 Rebuild Everything

```sh
make rebuild
```

</div>  <div style="border-left: 1px solid #ccc; height: auto;"></div> <div style="flex: 1">

### 🧪 Run tests

```sh
make test
```

</div> </div>

- The Makefile simplifies building, running, and cleaning up. Modify it to extend functionality as needed.

---

## 🎨 Geometrical Shapes Renderer in Rust

- _This project generates and draws various geometric shapes — including lines, circles, triangles, rectangles, pentagons, and cubes — onto a 1000x1000 pixel canvas using the raster image library._

- _When run, the project outputs a file called image.png containing your rendered shapes._

### 📁 Project Structure

```sh
.
├── Docs/
│   ├──  media/
│   |     └── instructions.png
│   ├── CONTRIBUTIONS.md
│   ├── INSTRUCTIONS.md
│   ├── LICENSE.md
│   └── README.md
├── src/
│   ├── geometrical_shapes/
│   |    ├── circle.rs
│   |    ├── cube.rs
│   |    ├── line.rs
│   |    ├── mod.rs
│   |    ├── pentagon.rs
│   |    ├── point.rs
│   |    ├── rectangle.rs
│   |    └── triangle.rs
│   └── main.rs
│
├── .gitignore
├── Cargo.toml
├── Cargo.lock
└── Makefile
```

### 📦 Dependencies Used

#### 1. raster

🔹 Purpose:
Handles image creation, manipulation, and saving.

🔹 Used For:

➜ Creating a blank canvas:

```rs
Image::blank(1000, 1000)
```

➜ Drawing pixels safely using a custom Displayable trait:

```rs
image.display(x, y, color);
```

➜ Setting individual pixels:

```rs
raster::editor::draw_pixel(image, x, y, color);
```

➜ Exporting the final image:
➜ Implementing .random() constructors for shapes:

```rs
raster::save(&image, "image.png")
```

🔹 Role: Core to all rendering and output functionality.

---

#### 2. rand

🔹 Purpose:

- Generates random numbers for random shape positioning and attributes.

🔹 Used For:

➜ Creating random points and dimensions:

```rs
rand::random::<u32>() % 1000
```

➜ Implementing .random() constructors for shapes:

```rs
Point::random()
Line::random()
Rectangle::random()
```

🔹 Role: Provides randomness to shape generation, adding variety to the rendered image.

---

### 🔧 How It Works

`main.rs:`

- Initializes a blank canvas (Image::blank(1000, 1000)).

- Randomly generates and draws multiple shapes.

`Uses custom traits:`

- **Drawable:** every shape implements draw() and color().

- **Displayable:** used by the Image to render pixels safely.

`geometrical_shapes/:`

- Contains the logic for creating, drawing, and coloring shapes.

  **Defines:**

  Point, Line, Rectangle, Triangle, Circle, Pentagon and Cube

- Each shape implements trait methods and associated constructors:

  **new()** ➜ creates a shape from parameters.

  **random()** ➜ generates shapes with random positions.

---

### 🧩 Traits

#### Drawable

Implemented by all shapes:

```rs
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}
```

#### Displayable

Implemented for the raster Image to plot individual pixels safely:

```rs
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}
```

🎁 Example Output

After running the project, you should generate something like this:

<img src="media/instructions.png" alt="Example Output" width="200" height="200">

---

## Code

- Based on the file structure, the functional project is divided into the main sourcefile and the geometrical-shape modules.

- Let us have a look at each file:

### 📄 File: point.rs (in geometrical_shapes module)

**🔍 Purpose:**

- Defines the logic and structure of a Point on the image. A Point has coordinates and a unique color, and can be manually placed or randomly generated.
  **📌 Key Functions:**

#### 📘 `Point` Struct & Trait Implementation Summary

| **Element**        | **Type** / **Signature**                   | **Description**                                                                              |
| ------------------ | ------------------------------------------ | -------------------------------------------------------------------------------------------- |
| **Struct**         | `Point`                                    | Represents a drawable point with coordinates and color.                                      |
| **Fields**         | `x: i32`<br>`y: i32`<br>`color: Color`     | The point's position and its color.                                                          |
| **Method**         | `new(x: i32, y: i32) -> Point`             | Creates a new `Point` at given coordinates with a random RGB color.                          |
| **Method**         | `random(width: i32, height: i32) -> Point` | Creates a `Point` at a random `(x, y)` within a given width and height, with a random color. |
| **Trait Impl**     | `impl Drawable for Point`                  | Allows a `Point` to be drawn on an image.                                                    |
| **Method (trait)** | `draw(&self, image: &mut Image)`           | Draws the point as a 3x3 square on the image using its color.                                |
| **Method (trait)** | `color(&self) -> Color`                    | Returns a clone of the point’s color.                                                        |

<b style="font-weight: bold">🧩Traits Implemented:</b>

Drawable: So that the point knows how to draw itself.

Uses Displayable indirectly for image pixel setting.

<b style="font-weight: bold">🧠 Role in Program:</b>

Used in the main program to place individual points or draw complex shapes like triangles or rectangles.

Enhances the visual output and randomness of the image generation process.

### 📄 File line.rs (in geometrical_shapes module)

This file defines a Line struct that represents a colored line between two points. It implements a method to draw that line (with thickness and color) on an image using Bresenham’s algorithm—a common line drawing technique in computer graphics.

It also implements the Drawable trait, meaning this Line can be rendered visually in the same way Point was.

#### 📦 What this file contributes to the project

Graphics Engine Module: This file adds line drawing functionality to your project.

Visual Rendering: By implementing Drawable, Line objects can now be rendered on an image.

Custom/Random Behavior: Supports both randomly generated lines and manually constructed ones.

| **Code Section**                             | **Explanation**                                                                                                                                                                        |
| -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `use super::{Drawable, Point, Displayable};` | Imports traits `Drawable` and `Displayable`, and the `Point` struct from the parent module.                                                                                            |
| `use rand::Rng;`                             | Used to generate random numbers for line thickness, coordinates, and color.                                                                                                            |
| `use raster::{Color, Image};`                | From the `raster` crate, `Color` represents RGB color values, and `Image` is the drawable canvas.                                                                                      |
| `fn draw_line_with_color(...)`               | **Private helper function** that draws a line from `start` to `end` using a variant of Bresenham's algorithm. It supports adjustable `thickness` and draws with the specified `color`. |
| `struct Line`                                | Represents a line with a start point, end point, color, and thickness.                                                                                                                 |
| `impl Line` – `new`                          | Creates a new line between two given points with random thickness and color.                                                                                                           |
| `impl Line` – `random`                       | Generates a line with random start and end points, random color and thickness, within specified `width` and `height`.                                                                  |
| `impl Line` – `from_points`                  | Custom constructor that accepts two points, a specific thickness, and color to create a line.                                                                                          |
| `impl Drawable for Line` – `draw`            | Implements the `draw` method from the `Drawable` trait to render the line on an image.                                                                                                 |
| `impl Drawable for Line` – `color`           | Returns the color of the line.                                                                                                                                                         |

### 📄 File triangle.rs (in geometrical_shapes module)

The `Triangle` module defines a structure for creating triangles, allowing for both specific triangle generation and random triangle generation. It implements the `Drawable` trait, enabling the drawing of triangles onto an image.

The `Triangle` structure holds a vector of tuples, where each tuple contains three points and a color defining the triangle's vertices and its color.

**Functions**
`new`

This function creates a triangle from three specified points.

#### Parameters

a: A reference to the first Point (vertex of the triangle).
b: A reference to the second Point (vertex of the triangle).
c: A reference to the third Point (vertex of the triangle).

#### Returns

A new instance of Triangle.

Example Usage in `main.rs`

```rs
let point_a = gs::Point::new(100, 100);
let point_b = gs::Point::new(150, 50);
let point_c = gs::Point::new(200, 100);
let triangle = gs::Triangle::new(&point_a, &point_b, &point_c);
triangle.draw(&mut image);
```

#### random

This function generates a random collection of triangles, allowing for variability in triangle dimensions and positions.

#### Parameters

- \_a, \_b, \_c: These parameters are not used in the random generation but are maintained for function signature consistency.

#### Returns

A new instance of Triangle containing between 2 and 3 randomly generated triangles.

Example Usage in main.rs

To generate and draw 2 to 3 random triangles, use the following code snippet:

```rs
let random_triangle = gs::Triangle::random(&gs::Point::new(0, 0), &gs::Point::new(0, 0), &gs::Point::new(0, 0));
random_triangle.draw(&mut image);
```

### 📄 File circle.rs (in geometrical_shapes module)

The `Circle` module defines a structure for creating circles, allowing for both specific circle generation and random circle generation. It implements the `Drawable` trait, enabling the drawing of circles onto an image.

The `Circle` structure holds a vector of tuples, where each tuple contains a center point and a radius that define the circle's position and size.

#### Functions

`new`
This function creates a circle from a specified center point and radius.

#### Parameters for circle::new()

`center:` A reference to the Point representing the center of the circle.
radius: An integer that specifies the radius of the circle.

#### Returns circle::new()

A new instance of Circle.

Example Usage in main.rs
To create a single, specific circle using this function, use the following code snippet in your main.rs:

```rs
let center = gs::Point::new(500, 500);
let radius = 100;
let circle = gs::Circle::new(&center, radius);
circle.draw(&mut image);
```

**random**
This function generates a random circle, allowing for variability in position and radius.

#### Parameters for circle::random()

width: The width of the image area to constrain random placement of the circle.
height: The height of the image area to constrain random placement of the circle.

#### Returns circle::random()

A new instance of Circle containing a randomly generated circle.

Example Usage in main.rs
To generate and draw a random circle, use the following code snippet:

```rs
let random_circle = gs::Circle::random(image.width, image.height);
random_circle.draw(&mut image);
```

### 📄 File rectangle.rs (in geometric_shapes module)

The `Rectangle` module defines a structure for creating rectangles, allowing for both specific rectangle generation and random rectangle generation. It implements the `Drawable` trait, enabling the drawing of rectangles onto an image.

The `Rectangle` structure holds a vector of tuples, where each tuple contains two points (defining the corners of the rectangle) and a color.

#### Functions for rectangle

`new`
This function creates a rectangle defined by two specified points.

#### Parameters for rectangle::new()

p1: A reference to the first Point representing one corner of the rectangle.
p2: A reference to the second Point representing the opposite corner of the rectangle.

#### Returns for rectangle::new()

A new instance of Rectangle.

Example Usage in main.rs
To create a single, specific rectangle using this function, use the following code snippet in your main.rs:

```rs
let point1 = gs::Point::new(100, 100);
let point2 = gs::Point::new(300, 200);
let rectangle = gs::Rectangle::new(&point1, &point2);
rectangle.draw(&mut image);
```

`random`
This function generates between 2 and 3 random rectangles, each with random dimensions and colors.

#### Parameters

\_p1, \_p2: These parameters are not directly used in the random generation but are included for consistency in the function signature.

#### Returns

A new instance of Rectangle containing randomly generated rectangles.

Example Usage in main.rs
To generate and draw random rectangles, use the following code snippet:

```rs
let random_rectangles = gs::Rectangle::random(&gs::Point::new(0, 0), &gs::Point::new(0, 0));
random_rectangles.draw(&mut image);
```

### 📄 File cubes.rs (in geometric_shapes module)

The Cubes module is designed to provide functionality for drawing three-dimensional cube shapes on a two-dimensional canvas. This allows users to visualize cubes in a flat image format. Each cube is defined by a center point and a size, enabling easy customization of its position and dimensions. The center point determines where the cube is located on the canvas, while the size parameter defines the length of its sides.

When drawing a cube, the module calculates the appropriate vertices based on the given center and draws lines connecting these vertices to create the illusion of a three-dimensional object. By leveraging this functionality, users can generate multiple cubes at varying positions and sizes, enhancing the complexity and visual appeal of the drawings. This capability is particularly useful in graphic applications where three-dimensional shapes are needed in a primarily 2D context, allowing developers to simulate depth and perspective simply by manipulating the size and position of each cube

To create and draw a cube in the main.rs file, you would use the following code snippet:

```rs
let cube = gs::Cubes::new(&gs::Point::new(650, 250), 150);
cube.draw(&mut image);
```

**Methods:**

new(center: &Point, size: i32): Creates a new cube centered at a given point with a specified size.
random(width: i32, height: i32): Generates a random number of cubes (between 3 to 6), each with a random center, size, and color.

**Drawing Multiple Cubes**
To draw more than one cube, you can leverage a loop that creates multiple instances of the Cubes. You can vary the center position and size based on some logic or randomization to generate a more diverse set of cubes. Here’s how you can implement it. Add the following code snippet to your main.rs file:

```rs
let cubes = gs::Cubes::random(image.width, image.height);
cubes.draw(&mut image);
```

### 📄 File pentagons.rs (in geometric_shapes module)

The Pentagon module allows for the efficient creation and rendering of pentagons on the canvas. With a simple interface, users can define a pentagon by specifying a center point and a radius. The radius determines the distance from the center to each vertex of the pentagon, facilitating uniformity in shape regardless of size. This module takes advantage of trigonometric calculations to evenly distribute the vertices around the center point, ensuring the pentagon maintains its regular shape.

When the pentagon is drawn, the module calculates the five vertices based on the provided center and radius, producing a symmetrical and aesthetically pleasing shape. The ability to generate random colors further enhances its visual diversity, allowing for vibrant and engaging drawings. This capability makes the Pentagon module a valuable addition for creating intricate designs or patterns, as well as introducing geometric elements into more complex visual compositions.

To create and draw a pentagon in the main.rs file, the following code snippet can be used:

```rs
let pentagon = gs::Pentagon::new(&gs::Point::new(820, 800), 120);
pentagon.draw(&mut image);
```

Similarly we can generate multiple number of pentagons by using the random method. We willl generate a rnandom number between 2 and 6 and draw them on the image. To do this we would add te following code snippet to your main.rs file:

```rs
let pentagons = Pentagon::random(image.width, image.height);
pentagons.draw(&mut image);
```

---

## Testing

Got it! Here's a README section specifically for documenting the tests of the file, including an overview of what each test checks and how to run them:


---

## Testing

This project includes unit tests to ensure the correct behavior of the Rectangle and Point structs, as well as their associated methods. The tests are written using Rust's built-in test framework and verify the functionality of methods such as creation, random generation, drawing, and color handling.

### Running the Tests

To run the tests for this project, use the following command in your terminal:

```sh
cargo test
```

This will run all the unit tests defined in the project.

### Tests Overview

The following tests are included:

`test_rectangle_new`

**Purpose:** Verifies that the Rectangle::new() method correctly creates a rectangle based on two points (p1 and p2), ensuring that the top-left and bottom-right points are set correctly.

_What it checks:_

Correct x and y coordinates for the top-left and bottom-right points.

Color values (RGB) are within the expected range (100 to 255).

`test_rectangle_random`

**Purpose:** Tests the Rectangle::random() method, which generates random rectangles with random positions and sizes.

_What it checks:_

The random rectangles' positions fall within the valid bounds of the canvas (0 to 800 for both x and y).

The RGB values of the generated colors are within the expected range (100 to 255).

`test_rectangle_draw`

**Purpose:** Verifies the functionality of the draw() method by rendering a rectangle onto an image and checking if the pixels are updated correctly.

_What it checks:_

Ensures that the rectangle's edges are properly drawn and that the pixels within the rectangle match the expected color.

`test_rectangle_color`

**Purpose:** Confirms that the color() method of the Rectangle struct returns the correct color. In this case, it is expected to return Color::black().

_What it checks:_

Ensures that the color of the rectangle is set to black.

---

## 👥 Authors

- Core Team:

  [Ray Madara](https://github.com/Raymond9734) ➜ Architecture & structure

  [Adioz](https://github.com/adiozdaniel) ➜ Geometry & rendering

  [Anxiel Ray](https://github.com/anxielray) ➜ Shape logic

## 🤝 Contributing

- _We welcome PRs!_ 🚀

  1. Fork the repo

  2. Create a feature branch:
     git checkout -b feature/your-feature

  3. Commit changes:
     git commit -m "Add new feature"

  4. Push to your branch:
     git push origin feature/your-feature

  5. Submit a Pull Request 📝

- _Read the [Contribution Guidelines](CONTRIBUTIONS.md) for more._

## 📜 License

- This project is licensed under the MIT License.
- See [LICENSE](LICENSE.md) for full details.

---

❓ Questions or Issues?

Feel free to [open an issue](https://learn.zone01kisumu.ke/git/adaniel/drawing/issues/new) or submit a [pull request!](https://learn.zone01kisumu.ke/git/adaniel/drawing/pulls)

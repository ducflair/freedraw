<p align="center">
  <br/>
  <a target="_blank"><img width="256px" src="https://github.com/ducflair/freedraw/blob/main/public/card.png?raw=true" /></a>
  <p align="center">Rust Hand Drawn Path Generator</p>
  <p align="center" style="align: center;">
    <a href="https://crates.io/crates/freedraw"><img src="https://shields.io/badge/Crates-FFC933?logo=Rust&logoColor=646464&style=round-square" alt="Crates" /></a>
    <a href="https://github.com/ducflair/freedraw/releases"><img src="https://img.shields.io/crates/v/freedraw?style=round-square&label=latest%20stable" alt="Crates.io freedraw@latest release" /></a>
    <a href="https://crates.io/crates/freedraw"><img src="https://img.shields.io/crates/d/freedraw?style=round-square&color=salmon" alt="Downloads" /></a>
    <img src="https://shields.io/badge/Rust-CE412B?logo=Rust&logoColor=fff&style=round-square" alt="Rust" />
  </p>
</p>

# freedraw

Rust port of the [perfect-freehand](https://github.com/steveruizok/perfect-freehand) library for creating smooth freehand lines in SVG.

## SVG Examples

This repository includes a Yew demo application that showcases various SVG examples generated with the `freedraw` library.

### Generating SVG Examples

The SVG examples are generated using the `svg_conversion` tool:

```bash
cargo run --example svg_conversion
```

This will read data from the `tests` directory and generate SVG files in the `demo/dist` directory.

### Running the Demo

To run the demo application:

```bash
cd demo
trunk serve
```

Then visit http://localhost:8080/freedraw/ in your browser.

## Project Structure

- `src/` - Core library code
- `tests/` - Test data and unit tests
- `tools/` - Utility scripts
- `demo/` - Yew application for showcasing SVG examples

## Overview

freedraw helps you create high-quality freehand strokes in Rust applications. It generates smooth, beautiful outlines from a series of input points, supporting pressure sensitivity and customizable styling.

## Features

- Create beautiful, smooth freehand strokes
- Pressure-sensitive drawing with natural tapering
- Customizable stroke properties (size, smoothing, thinning)
- Simulate pressure based on velocity
- Generate SVG paths from strokes
- Built as a lightweight, pure Rust implementation

## Installation

Add the dependency to your Cargo.toml:

```toml
[dependencies]
freedraw = "x.x.x"
```

## Basic Usage

```rust
use freedraw::{get_stroke, InputPoint, StrokeOptions};

fn main() {
    // Create some example points
    let points = vec![
        InputPoint::Array([100.0, 100.0], Some(0.5)),
        InputPoint::Array([200.0, 150.0], Some(0.7)),
        InputPoint::Array([300.0, 100.0], Some(0.5)),
    ];

    // Create default options
    let options = StrokeOptions::default();

    // Generate the stroke outline
    let outline = get_stroke(&points, &options);

    // Use the outline points to draw a path
    // (e.g., convert to SVG or render with a graphics library)
}
```

## Converting to SVG Paths

The library includes a utility to convert stroke outlines to SVG path data:

```rust
use freedraw::{get_stroke, get_svg_path_from_stroke, InputPoint, StrokeOptions};

// Generate the stroke outline
let outline = get_stroke(&points, &options);

// Convert to SVG path data
let path_data = get_svg_path_from_stroke(&outline, true); // true = closed path

// Use in an SVG element
println!("<path d=\"{}\" fill=\"black\" />", path_data);
```

## Options

You can customize the appearance of strokes using the `StrokeOptions` struct:

```rust
let options = StrokeOptions {
    size: Some(16.0),               // Base size (diameter) of the stroke
    thinning: Some(0.5),            // Effect of pressure on stroke width
    smoothing: Some(0.5),           // How much to soften the stroke's edges
    streamline: Some(0.5),          // How much to streamline the stroke
    simulate_pressure: Some(true),  // Whether to simulate pressure based on velocity
    start: Some(TaperOptions {      // Tapering options for the start of the line
        taper: Some(TaperType::Bool(true)),
        ..Default::default()
    }),
    end: Some(TaperOptions {        // Tapering options for the end of the line
        taper: Some(TaperType::Bool(true)),
        ..Default::default()
    }),
    ..Default::default()
};
```

### Option Details

| Property           | Type     | Default | Description                                           |
| ------------------ | -------- | ------- | ----------------------------------------------------- |
| `size`             | number   | 8       | The base size (diameter) of the stroke.               |
| `thinning`         | number   | .5      | The effect of pressure on the stroke's size.          |
| `smoothing`        | number   | .5      | How much to soften the stroke's edges.                |
| `streamline`       | number   | .5      | How much to streamline the stroke.                    |
| `simulatePressure` | boolean  | true    | Whether to simulate pressure based on velocity.       |
| `last`             | boolean  | true    | Whether the stroke is complete.                       |

The `start` and `end` options accept a `TaperOptions` struct:

| Property | Type              | Default | Description                                                                              |
| -------- | ----------------- | ------- | ---------------------------------------------------------------------------------------- |
| `cap`    | boolean           | true    | Whether to draw a cap.                                                                   |
| `taper`  | TaperType         | None    | The distance to taper. Can be a numerical value or boolean.                             |
| `easing` | EasingType        | linear  | An easing function for the tapering effect.                                              |

## Input Points

The library supports two formats for input points:

1. Array format: `InputPoint::Array([x, y], pressure)` where pressure is optional
2. Struct format: `InputPoint::Struct { x, y, pressure }` where pressure is optional

If pressure is not provided, it defaults to 0.5.

## Advanced Usage

For advanced usage, the library exports smaller functions that `get_stroke` uses internally:

```rust
// Get detailed stroke points with pressure, vector, distance, etc.
let stroke_points = get_stroke_points(&input_points, &options);

// Generate outline points from the detailed stroke points
let outline_points = get_stroke_outline_points(&stroke_points, &options);
```

## Examples

Check out the examples directory for more usage examples:

- SVG path generation
- Different stroke styles and parameters
- Edge cases (one point, two points)
- HTML viewer for comparing different options

## License

[MIT License](LICENSE)

Original JavaScript library by [Steve Ruiz](https://twitter.com/steveruizok) 
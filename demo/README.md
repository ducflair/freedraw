# Freedraw SVG Demo

A Yew application that showcases SVG examples generated with the Freedraw library.

## Running the Demo

Prerequisites:
- Rust and Cargo
- Trunk: `cargo install trunk`

To run the demo application:

```bash
# From the demo directory
trunk serve

# OR from the project root
cd demo && trunk serve
```

Then visit http://localhost:8080/freedraw/ in your browser.

## Generating SVG Examples

The SVG examples are generated from test data using the `svg_conversion` tool:

```bash
# From the project root
cargo run --example svg_conversion
```

This will generate SVG files in the `demo/dist` directory, which are then served by the Yew application.

## Directory Structure

- `src/` - Yew application source code
- `dist/` - Generated SVG files
- `styles.css` - Application styles
- `index.html` - HTML template
- `Trunk.toml` - Trunk configuration

## Features

The demo showcases various SVG rendering options from the Freedraw library:
- Different stroke styles (fill vs. stroke)
- Tapered vs. non-tapered strokes
- Various thickness settings
- Different input point formats
- Edge cases (one point, two points, etc.)

## Purpose

This application provides a visual gallery of SVG examples generated with different parameters and settings in Perfect Freehand. The examples showcase:

- Different stroke styles (fill vs. stroke)
- Tapered vs. non-tapered strokes
- Various thickness settings
- Different input point formats
- Edge cases (one point, two points, etc.)
- Real-world examples (sample drawing, flash, corners)

## Credits

Based on the Rust port of [Perfect Freehand](https://github.com/steveruizok/perfect-freehand) by Steve Ruiz. 
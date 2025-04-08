mod svg_conversion;

fn main() {
    // Run the SVG generation code
    svg_conversion::main();
    println!("SVG generation complete. Run the web app using 'trunk serve' to view the results.");
} 
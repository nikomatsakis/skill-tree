use rust_embed::RustEmbed;

mod preprocessor;

// Embed the contents of the viz-js folder at build time
#[derive(RustEmbed)]
#[folder = "../viz-js/"]
struct VizJs;

fn main() {}

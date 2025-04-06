//  Follow up on this https://github.com/rust-lang/cargo/issues/9398

module.exports = {
  branches: ["main", { name: "next", prerelease: true }],
  plugins: [
    "@semantic-release/commit-analyzer",
    "@semantic-release/release-notes-generator",
    [
      "@semantic-release/exec",
      {
        // Prepare step: Set the crate version and build the project
        prepareCmd:
          "cargo set-version ${nextRelease.version} && cargo build --release",
        
        // Publish step: Publish the crate to crates.io
        publishCmd:
          "cargo publish --allow-dirty --token ${process.env.CARGO_REGISTRY_TOKEN}",
      }
    ],
    ["@semantic-release/github", {
      "assets": [
        {"path": "freedraw/target/release/libfreedraw.rlib", "label": "Freedraw Library"},
      ]
    }],
    [
      "@semantic-release/git",
      {
        "assets": ["freedraw/Cargo.toml", "freedraw/Cargo.lock"],
        "message": "chore(release): ${nextRelease.version} [skip ci]\n\n${nextRelease.notes}"
      }
    ]
  ],
};
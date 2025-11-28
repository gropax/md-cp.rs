{ pkgs, src }:

let
  version = "0.1.0";
in
pkgs.rustPlatform.buildRustPackage {
  pname = "md-cp";
  version = version;

  meta = {
    description = "Rust CLI app to overwrite Markdown file content without erasing frontmatter";
  };

  src = src;

  cargoLock = {
    lockFile = src + "/Cargo.lock";
  };
}

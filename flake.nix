{
  description = "Rust dev env for md-cp";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        appName = "md-cp";
        version = "0.1.0";
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = appName;
          version = version;

          meta = {
            description = "Rust CLI app to overwrite Markdown file content without erasing frontmatter";
          };

          src = pkgs.lib.cleanSource ./.;  # Prevent rebuilding after modifying non source file

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          # Inject version into Cargo at build time
          #buildInputs = [ pkgs.rustPlatform.cargoSetupHook ];
          #CARGO_PKG_VERSION = version;
        };

        devShells.default = pkgs.mkShell {
          name = appName;

          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            rust-analyzer
          ];

          shellHook = ''
            rustup component add rust-src >/dev/null 2>&1 || true
          '';
        };

        apps.default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/${appName}";
        };
      }
    );
}

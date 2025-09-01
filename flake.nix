{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    {
      self,
      nixpkgs,
      ...
    }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system} = {
        default = self.packages.${system}.todo-cli-rs;
        todo-cli-rs = pkgs.rustPlatform.buildRustPackage {
          pname = "todo-cli-rs";
          version = "1.0";
          src = pkgs.lib.sourceFilesBySuffices ./. [
            ".rs"
            ".toml"
            ".lock"
          ];
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      };

      devShells.${system}.default = pkgs.mkShell {
        packages = with pkgs; [
          nixfmt-rfc-style

          # rust
          rustc
          cargo
          rust-analyzer
          rustfmt
          clippy
        ];

        env = {
          RUST_BACKTRACE = "full";
        };
      };
    };
}

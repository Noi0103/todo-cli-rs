{
  description = "An example project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/25.05";
    systems.url = "github:nix-systems/default";
    git-hooks.url = "github:cachix/git-hooks.nix";
  };

  outputs =
    {
      self,
      nixpkgs,
      systems,
      git-hooks,
      ...
    }@inputs:
    let
      forEachSystem = nixpkgs.lib.genAttrs (import systems);
    in
    {
      packages = forEachSystem (system: {
        default =
          let
            pkgs = nixpkgs.legacyPackages.${system};
            inherit (self.checks.${system}.pre-commit-check)
              shellHook
              enabledPackages
              ;
          in
          pkgs.rustPlatform.buildRustPackage {
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
      });

      # Enter a development shell with `nix develop`.
      # The hooks will be installed automatically.
      # Or run pre-commit manually with `nix develop -c pre-commit run --all-files`
      devShells = forEachSystem (system: {
        default =
          let
            pkgs = nixpkgs.legacyPackages.${system};
            inherit (self.checks.${system}.pre-commit-check)
              shellHook
              enabledPackages
              ;
          in
          pkgs.mkShell {
            inherit shellHook;
            buildInputs = enabledPackages;
            packages = with pkgs; [
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
      });

      # Run the hooks with `nix fmt`.
      formatter = forEachSystem (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          config = self.checks.${system}.pre-commit-check.config;
          inherit (config) package configFile;
          script = ''
            ${package}/bin/pre-commit run --all-files --config ${configFile}
          '';
        in
        pkgs.writeShellScriptBin "pre-commit-run" script
      );

      # Run the hooks in a sandbox with `nix flake check`.
      # Read-only filesystem and no internet access.
      checks = forEachSystem (system: {
        pre-commit-check = inputs.git-hooks.lib.${system}.run {
          src = ./.;
          hooks = {
            nixfmt-rfc-style.enable = true;
            #rustfmt.enable = true;
            #clippy = {
            #enable = true;
            #packageOverrides.cargo = pkgs.cargo;
            #packageOverrides.clippy = pkgs.clippy;
            #settings.allFeatures = true;
            #};
          };
        };
      });
    };
}

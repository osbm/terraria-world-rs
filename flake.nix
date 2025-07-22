{
  description = "A Terraria world parser in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    terraria-worlds.url = "github:osbm/terraria-worlds";
    terraria-worlds.flake = false;
  };

  outputs =
    {
      self,
      nixpkgs,
      naersk,
      terraria-worlds,
    }:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
      ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      overlay = final: prev: {
        "${cargoToml.package.name}" = final.callPackage ./. {
          inherit naersk terraria-worlds;
          lihzahrd = self.outputs.packages.${final.system}.lihzahrd;
        };
      };

      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlay
            ];
          };
        in
        {
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
          lihzahrd = pkgs.python312Packages.buildPythonPackage rec {
            pname = "lihzahrd";
            version = "3.1.1";
            src = pkgs.fetchFromGitHub {
              owner = "Steffo99";
              repo = "lihzahrd";
              rev = "v${version}";
              sha256 = "sha256-95xu4FJ+cqGYxqrGkf4k8M2zz2fcoErM2P1PqH2hQ28=";
            };
            format = "pyproject";
            build-system = with pkgs.python312Packages; [
              hatchling
            ];
            pythonImportsCheck = [ "lihzahrd" ];

            doCheck = false; # no tests available

            meta = with pkgs.lib; {
              description = "Terraria game world parser for Python";
              homepage = "https://github.com/Steffo99/lihzahrd";
              license = licenses.eupl12;
            };
          };
        }
      );

      defaultPackage = forAllSystems (
        system:
        (import nixpkgs {
          inherit system;
          overlays = [ self.overlay ];
        })."${cargoToml.package.name}"
      );

      checks = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlay
            ];
          };
        in
        {
          format =
            pkgs.runCommand "check-format"
              {
                buildInputs = with pkgs; [
                  rustfmt
                  cargo
                ];
              }
              ''
                ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
                ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
                touch $out # it worked!
              '';
          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";
        }
      );
      devShell = forAllSystems (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlay ];
          };
        in
        pkgs.mkShell {
          inputsFrom = [
            pkgs."${cargoToml.package.name}"
          ];
          buildInputs = with pkgs; [
            rustfmt
            nixpkgs-fmt
            rustPackages.clippy
            yamlfix
            black
            mdbook
            (pkgs.python312.withPackages (python-pkgs: [
              python-pkgs.pip
              self.outputs.packages.${system}.lihzahrd
            ]))
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          TEST_WORLDS_DIR = "${terraria-worlds}";

        }
      );
    };
}

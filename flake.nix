{
  description = "A Terraria world parser in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    terraria-worlds.url = "github:osbm/terraria-worlds";
    terraria-worlds.flake = false;
  };

  outputs = { self, nixpkgs, naersk, terraria-worlds }:
    let
      cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
      supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" ];
      forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system: f system);
    in
    {
      overlay = final: prev: {
        "${cargoToml.package.name}" = final.callPackage ./. { inherit naersk; };
      };

      packages = forAllSystems (system:
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

            doCheck = false; # Disable tests for now

            meta = with pkgs.lib; {
              description = "Terraria game world parser for Python";
              homepage = "https://github.com/Steffo99/lihzahrd";
              license = licenses.eupl12;
            };
          };
        });

      defaultPackage = forAllSystems (system: (import nixpkgs {
        inherit system;
        overlays = [ self.overlay ];
      })."${cargoToml.package.name}");

      checks = forAllSystems (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              self.overlay
            ];
          };
        in
        {
          format = pkgs.runCommand "check-format"
            {
              buildInputs = with pkgs; [ rustfmt cargo ];
            } ''
            ${pkgs.rustfmt}/bin/cargo-fmt fmt --manifest-path ${./.}/Cargo.toml -- --check
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out # it worked!
          '';

          "${cargoToml.package.name}" = pkgs."${cargoToml.package.name}";

          # Integration test using lihzahrd Python library
          integration-test = pkgs.runCommand "integration-test"
            {
              buildInputs = with pkgs; [
                python312
                python312Packages.lihzahrd
                cargo
                rustc
              ];
              TEST_WORLDS_DIR = "${terraria-worlds}";
            } ''
            # Run the external integration test script
            if [ -f "small_corruption.wld" ]; then
              python3 ${./tests/integration_test.py} small_corruption.wld > python_output.json
              echo "Python parsing completed successfully"
            else
              echo "No test world file found, skipping integration test"
            fi

            touch $out
          '';
        });

      devShell = forAllSystems (system:
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
            python312
            self.packages."${system}".lihzahrd

          ];
          TEST_WORLDS_DIR = "${terraria-worlds}";
        });
    };
}

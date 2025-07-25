# default.nix
{
  lib,
  naersk,
  # , stdenv
  # , clangStdenv
  # , hostPlatform
  targetPlatform,
  pkg-config,
  libiconv,
  rustfmt,
  cargo,
  rustc,
  python312,
  terraria-worlds,
  lihzahrd,
  rustPackages,
# , llvmPackages # Optional
# , protobuf     # Optional
}:

let
  cargoToml = (builtins.fromTOML (builtins.readFile ./Cargo.toml));
in

naersk.lib."${targetPlatform.system}".buildPackage {
  src = ./.;

  buildInputs = [
    rustfmt
    pkg-config
    cargo
    rustc
    rustPackages.clippy
    libiconv
    (python312.withPackages (python-pkgs: [
      python-pkgs.pip
      lihzahrd
    ]))
  ];
  checkInputs = [
    cargo
    rustc
  ];

  doCheck = true;
  CARGO_BUILD_INCREMENTAL = "false";
  RUST_BACKTRACE = "full";
  copyLibs = true;

  # Optional things you might need:
  #
  # If you depend on `libclang`:
  # LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
  #
  # If you depend on protobuf:
  # PROTOC = "${protobuf}/bin/protoc";
  # PROTOC_INCLUDE = "${protobuf}/include";
  TEST_WORLDS_DIR = "${terraria-worlds}";

  name = cargoToml.package.name;
  version = cargoToml.package.version;

  meta = with lib; {
    description = cargoToml.package.description;
    homepage = cargoToml.package.homepage;
    license = with licenses; [ mit ];
    maintainers = with maintainers; [ osbm ];
  };
}

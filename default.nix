{
  pkgs ? import <nixpkgs> { },
}:
let
  src = pkgs.lib.cleanSource ./.;
  cargoTOML = pkgs.lib.importTOML "${src}/Cargo.toml";
in
pkgs.rustPlatform.buildRustPackage {
  pname = cargoTOML.package.name;
  version = cargoTOML.package.version;
  nativeBuildInputs = [ pkgs.cmake ];
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };

  inherit src;
}

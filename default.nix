{
  pkgs ? import <nixpkgs> { },
  lib,
  ...
}:
let
  src = pkgs.lib.cleanSource ./.;
  cargoTOML = pkgs.lib.importTOML "${src}/Cargo.toml";
in
pkgs.rustPlatform.buildRustPackage {
  pname = cargoTOML.package.name;
  version = cargoTOML.package.version;
  meta = {
    mainProgram = "whether";
  };
  nativeBuildInputs = [ pkgs.cmake pkgs.makeWrapper ];
  cargoLock = {
    lockFile = "${src}/Cargo.lock";
  };

  runtimeInputs = [
    pkgs.imagemagick
    pkgs.inkscape
    pkgs.rsync
    pkgs.openssh
  ];
  postFixup = "wrapProgram $out/bin/whether --prefix PATH : ${
    lib.makeBinPath [
      pkgs.imagemagick
      pkgs.inkscape
      pkgs.rsync
      pkgs.openssh
    ]
  }";

  inherit src;
}

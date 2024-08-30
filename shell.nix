{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  inputsFrom = [ (pkgs.callPackage ./package.nix { }) ];

  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
  ];

  RUST_BACKTRACE = 1;
}

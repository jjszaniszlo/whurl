{
  lib,
  rustPlatform,
  openssl,
  pkg-config,
}: let
  manifest = (lib.importTOML ./Cargo.toml).package;
  src = lib.cleanSource ./.; 
in
rustPlatform.buildRustPackage {
  pname = manifest.name;
  version = manifest.version;
  cargoLock.lockFile = ./Cargo.lock;
  src = src; 

  buildInputs = [
    openssl
  ];

  nativeBuildInputs = [
    pkg-config
  ];

  RUST_BACKTRACE = 1;
}

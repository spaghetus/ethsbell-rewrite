# This file defines a nix development environment.
# It allows for very fast development startup in
# new or ephemoral environments.
with (import <nixpkgs> {
  overlays = [
    (import (builtins.fetchTarball
      "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
  ];
}).pkgs;

pkgs.mkShell {
  nativeBuildInputs = [ rust-bin.nightly.latest.default openssl pkg-config ];
}

{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.clippy
    pkgs.cargo-release
  ];
  shellHook = ''
    export PATH="$PWD/scripts:$PATH"
    bash scripts/fetch-rebase
    echo "cargo: $(cargo -V) | rustc: $(rustc -V)"
    echo "rustfmt: $(rustfmt --version) | clippy: $(cargo clippy --version)"
  '';
}


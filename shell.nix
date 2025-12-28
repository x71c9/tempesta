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
    echo "cargo: $(cargo -V) | rustc: $(rustc -V)"
    echo "rustfmt: $(rustfmt --version) | clippy: $(cargo clippy --version)"
  '';
}


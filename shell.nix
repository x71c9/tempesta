{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.cargo-release
  ];
  shellHook = ''
    echo "cargo: $(cargo -V) | rustc: $(rustc -V)"
  '';
}


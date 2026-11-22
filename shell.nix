{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
  packages = [
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.clippy
  ];
  shellHook = ''
    export PATH="$PWD/scripts:$PATH"
    echo "cargo: $(cargo -V) | rustc: $(rustc -V)"
    echo "rustfmt: $(rustfmt --version) | clippy: $(cargo clippy --version)"
  '';
}


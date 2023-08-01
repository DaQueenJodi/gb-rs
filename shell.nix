{ pkgs ? import <nixpkgs> {} }: with pkgs;
mkShell {
  packages = [
    rustc
    cargo
  ];
}

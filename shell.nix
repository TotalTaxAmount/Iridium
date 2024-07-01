{ pkgs ? (import <nixpkgs> {
    config.allowUnfree = true;
}) }:

pkgs.stdenv.mkDerivation {
  name = "chess-bot";

  buildInputs = with pkgs; [
    rustup
    cargo
    arena
  ];
}
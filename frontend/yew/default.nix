{ nixpkgs ? import <nixpkgs> {  } }:

let
  pkgs = [
    nixpkgs.rustup
  ];

in
  nixpkgs.stdenv.mkDerivation {
    name = "fdd-yew";
    buildInputs = pkgs;
  }

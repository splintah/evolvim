{ pkgs ? import <nixpkgs> { } }:

with pkgs;

stdenv.mkDerivation rec {
  name = "rust-env";

  src = ./.;

  nativeBuildInputs = [
    rustc
    cargo

    # Example Build-time Additional Dependencies
  ];

  # Run-time Additional Dependencies
  buildInputs = with pkgs.xorg; [
    libX11
    libX11.dev
    libXcursor
    libXrandr
    libXi
    pkgs.libglvnd
  ];

  # Set Environment Variables
  RUST_BACKTRACE = 1;
  LD_LIBRARY_PATH = "${pkgs.stdenv.lib.makeLibraryPath buildInputs}";
}

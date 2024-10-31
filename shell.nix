{ pkgs ? import <nixpkgs> { } }:

pkgs.mkShell
{
  buildInputs = [
    pkgs.clippy
    pkgs.rustfmt
    pkgs.wayland
    pkgs.xorg.libX11
    pkgs.xorg.libXcursor
    pkgs.pkg-config
    pkgs.rustc
    pkgs.cargo
  ];
  shellHook = ''
    PATH=$PATH:/home/parzival/.cargo/bin
    export LIB_PATH="${pkgs.wayland}/lib:${pkgs.xorg.libX11}/lib:${pkgs.xorg.libXcursor}/lib";
    export RUSTC_WRAPPER=sccache
    export RUSTFLAGS="-Clink-args=-Wl,-rpath=$LIB_PATH"
  '';
}

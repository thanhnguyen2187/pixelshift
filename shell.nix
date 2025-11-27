{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  packages = [
    pkgs.openssl
    pkgs.pkg-config
    # pkgs.sqlite
  ];

  shellHook = ''
  '';
}

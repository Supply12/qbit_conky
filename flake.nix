{
  description = "Minimal Rust with pkg-config";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      devShell.${system} = pkgs.mkShell {
        packages = [
          pkgs.pkg-config
          pkgs.rustc
          pkgs.cargo
          pkgs.openssl
          pkgs.openssl.dev
          pkgs.zlib
          pkgs.zlib.dev
        ];
        
        env = {
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig:${pkgs.zlib.dev}/lib/pkgconfig";
        };
        
        shellHook = ''
          echo "Minimal Rust+pkg-config environment"
          echo "PKG_CONFIG_PATH: $PKG_CONFIG_PATH"
        '';
      };
    };
}
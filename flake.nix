{
  outputs = { self, naersk, nixpkgs, flake-utils }:
    {
      lib = import ./lib.nix nixpkgs.lib;
    } //
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        naersk' = pkgs.callPackage naersk { };
      in
      {
        packages = {
          default = naersk'.buildPackage {
            src = ./.;
          };
          vim = pkgs.vimUtils.buildVimPlugin {
            name = "fretwire";
            src = ./vim;
          };
        };
        devShells.default = pkgs.mkShell { };
      }
    );

  inputs.naersk = {
    url = "github:nix-community/naersk";
    inputs.nixpkgs.follows = "nixpkgs";
  };
}

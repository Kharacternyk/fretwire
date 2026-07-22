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
            name = "fretwire";
            version = "1.0.0-rc";
            src = ./.;
          };
          vim = pkgs.vimUtils.buildVimPlugin {
            pname = "fretwire";
            version = "1.0.0-rc";
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

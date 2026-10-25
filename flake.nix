{
  outputs =
    {
      flake-utils,
      naersk,
      nixpkgs,
      self,
    }:
    {
      lib = import ./lib.nix nixpkgs.lib;
    }
    // flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        naersk' = pkgs.callPackage naersk { };
      in
      {
        devShells.default = pkgs.mkShell { };
        packages = {
          default = naersk'.buildPackage {
            name = "fretwire";
            src = ./.;
            version = "1.0.0-rc";
          };
          vim = pkgs.vimUtils.buildVimPlugin {
            pname = "fretwire";
            src = ./vim;
            version = "1.0.0-rc";
          };
        };
      }
    );
  inputs.naersk = {
    inputs.nixpkgs.follows = "nixpkgs";
    url = "github:nix-community/naersk";
  };
}

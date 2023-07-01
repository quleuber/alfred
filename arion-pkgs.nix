let 
  flake = import ./nix/compat.nix;
in
  flake.inputs.nixpkgs.legacyPackages.x86_64-linux 

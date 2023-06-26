{
  description = "Alfred";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

  outputs = { self, nixpkgs }:

  let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in
  {
    devShell.${system} = pkgs.mkShell {
      buildInputs = with pkgs; [
        # openssl
        # prisma-engines
        nodePackages.prisma
      ];
    };
  };
}

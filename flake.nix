{
  description = "Alfred";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flake-utils.url = "github:numtide/flake-utils";

    flake-compat.url = "github:edolstra/flake-compat";
    flake-compat.flake = false;
  };

  outputs = { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        devShell = pkgs.mkShell {
          buildInputs = [
            pkgs.arion
            pkgs.terraform
            pkgs.terraform-providers.oci
            # pkgs.prisma-engines
            # pkgs.nodePackages.prisma
          ];
          # shellHook = let prisma-engines = pkgs.prisma-engines; in ''
          #   export PRISMA_MIGRATION_ENGINE_BINARY="${prisma-engines}/bin/migration-engine"
          #   export PRISMA_QUERY_ENGINE_BINARY="${prisma-engines}/bin/query-engine"
          #   export PRISMA_QUERY_ENGINE_LIBRARY="${prisma-engines}/lib/libquery_engine.node"
          #   export PRISMA_INTROSPECTION_ENGINE_BINARY="${prisma-engines}/bin/introspection-engine"
          #   export PRISMA_FMT_BINARY="${prisma-engines}/bin/prisma-fmt"
          # '';
        };
      });
}

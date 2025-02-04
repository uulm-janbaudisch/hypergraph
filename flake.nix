{
  description = "Packages for evaluating hypergraph partitioners.";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    crane.url = "github:ipetkov/crane/v0.20.0";
    d4 = {
      url = "github:SoftVarE-Group/d4v2/dump-preproc";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      d4,
      ...
    }:
    let
      lib = nixpkgs.lib;

      systems = [
        "aarch64-darwin"
        "aarch64-linux"
        "x86_64-darwin"
        "x86_64-linux"
      ];
    in
    {
      formatter = lib.genAttrs systems (system: nixpkgs.legacyPackages.${system}.nixfmt-rfc-style);
      packages = lib.genAttrs systems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          craneLib = crane.mkLib pkgs;
        in
        {
          default = self.packages.${system}.all;
          cnf_partitioner = pkgs.callPackage ./nix/cnf_partitioner.nix { inherit craneLib; };
          hypergraph_partitioner = pkgs.callPackage ./nix/hypergraph_partitioner.nix { inherit craneLib; };
          d4 = d4.packages.${system}.d4;
          mt-kahypar = d4.packages.${system}.mt-kahypar-bin;
          kahypar = pkgs.callPackage ./nix/kahypar.nix { };
          patoh = pkgs.callPackage ./nix/patoh.nix { };
          all = pkgs.buildEnv {
            name = "all";
            paths = [
              self.packages.${system}.cnf_partitioner
              self.packages.${system}.hypergraph_partitioner
              self.packages.${system}.d4
              self.packages.${system}.mt-kahypar
              self.packages.${system}.kahypar
              self.packages.${system}.patoh
            ];
          };
          container = pkgs.dockerTools.buildLayeredImage {
            name = "cnf_partitioner";
            contents = [
              self.packages.${system}.cnf_partitioner
              self.packages.${system}.hypergraph_partitioner
              self.packages.${system}.d4
              self.packages.${system}.kahypar
              self.packages.${system}.patoh
              (pkgs.runCommand "create-tmp" { } "install -dm 1777 $out/tmp")
            ];
            config = {
              Entrypoint = [ "/bin/cnf_partitioner" ];
              Env = [
                "D4_PATH=/bin/d4"
                "KAHYPAR_PATH=/bin/KaHyPar"
                "PATOH_PATH=/bin/patoh"
                "RUST_PATH=/bin/hypergraph_partitioner"
              ];
              Labels = {
                "org.opencontainers.image.source" = "https://github.com/uulm-janbaudisch/hypergraph";
                "org.opencontainers.image.description" = "Hypergraph Evaluation on d-DNNF Compilation";
              };
            };
          };
        }
      );
    };
}

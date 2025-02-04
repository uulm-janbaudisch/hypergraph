{
  lib,
  craneLib,
}:
let
  src =
    let
      mtkahyparFilter = path: _type: builtins.match ".*kahypar.*\.ini$" path != null;
      filter = path: type: (mtkahyparFilter path type) || (craneLib.filterCargoSources path type);
    in
    lib.cleanSourceWith {
      src = ./..;
      inherit filter;
      name = "source";
    };

  workspace = {
    pname = "hypergraph_partitioner";
    version = "0.0.0";
    inherit src;
    strictDeps = true;
  };

  cargoArtifacts = craneLib.buildDepsOnly workspace;
  metadata = craneLib.crateNameFromCargoToml { src = ../hypergraph_partitioner; };
in
craneLib.buildPackage (
  workspace
  // {
    inherit cargoArtifacts;
    pname = metadata.pname;
    version = metadata.version;
    cargoExtraArgs = "-p hypergraph_partitioner";
  }
)

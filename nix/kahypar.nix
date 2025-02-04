{
  lib,
  stdenv,
  fetchFromGitHub,
  cmake,
  boost,
}:
stdenv.mkDerivation ({
  pname = "kahypar";
  version = "1.4";

  outputs = [
    "out"
    "lib"
    "dev"
  ];

  src = fetchFromGitHub {
    owner = "kahypar";
    repo = "kahypar";
    fetchSubmodules = true;
    rev = "0823ea2c16d69eff4b5d14b044af1d69377396c7";
    hash = "sha256-oNMQz9/vH7Q0i0nNmpgaKtTZEbo1B98dEi67Hp3g+oU=";
  };

  patches = [ ./kahypar.patch ];

  nativeBuildInputs = [
    cmake
  ];

  buildInputs = [
    boost.dev
  ];

  meta = with lib; {
    mainProgram = "KaHyPar";
    description = "Multilevel hypergraph partitioning framework";
    longDescription = "KaHyPar (Karlsruhe Hypergraph Partitioning) is a multilevel hypergraph partitioning framework providing direct k-way and recursive bisection based partitioning algorithms that compute solutions of very high quality.";
    homepage = "https://github.com/kahypar/kahypar";
    license = licenses.gpl3;
    platforms = platforms.unix ++ platforms.windows;
  };
})

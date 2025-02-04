{
  lib,
  stdenv,
  fetchzip,
}:
let
  systemMap = {
    aarch64-darwin = {
      system = "Darwin-arm64";
      hash = "sha256-JpjBx+QNN/h0Ajqjk44BItT5mSQ+1Vn5/6sIIzLpYGc=";
    };
    aarch64-linux = {
      system = "Linux-aarch64";
      hash = "sha256-8WSs5OLYONB2jbbOJJUnO92B1Ocy6MC6rlUFgtCX9gY=";
    };
    x86_64-darwin = {
      system = "Darwin-x86_64";
      hash = "sha256-hmisGNSXsIzqlJUxr2/DtuZdpVxxRiSLzw5kcR6voLA=";
    };
    x86_64-linux = {
      system = "Linux-x86_64";
      hash = "sha256-vnO3PKNdKF9Bom4jJogV+VKacyaSc5JtMQHgda62D9g=";
    };
  };
  systemVersion = systemMap.${stdenv.system};
in

stdenv.mkDerivation {
  name = "PaToH";
  version = "3.3";

  outputs = [
    "out"
    "dev"
    "doc"
  ];

  src = fetchzip {
    url = "https://web.archive.org/web/20240607085226/https://faculty.cc.gatech.edu/~umit/PaToH/patoh-${systemVersion.system}.tar.gz";
    hash = systemVersion.hash;
  };

  installPhase = ''
    cd ${systemVersion.system}
    install -D patoh $out/bin/patoh
    install -D patoh.h $out/include/patoh.h
    install -D libpatoh.a $out/lib/libpatoh.a
    install -D manual.pdf $out/share/doc/patoh/manual.pdf
  '';

  meta = with lib; {
    mainProgram = "patoh";
    description = "A multilevel hypergraph partitioning tool.";
    homepage = "https://faculty.cc.gatech.edu/~umit/PaToH/manual.pdf";
    platforms = [
      "aarch64-darwin"
      "aarch64-linux"
      "x86_64-darwin"
      "x86_64-linux"
    ];
  };
}

{ stdenv, fetchurl, unzip }:
let
  version = "3.0.0.2";
in
stdenv.mkDerivation {
    name = "hydraHarpLib-${version}";

    buildInputs = [ unzip ];

    src = fetchurl {
        url = "https://www.picoquant.com/dl_software/HydraHarp400/HydraHarp400_SW_and_DLL_v3_0_0_2.zip";
        sha256 = "ca812d39ec7d0d906c0c3ebc5af6875d41412e140adecbddcdc953c0b5025bef";
    };

    unpackPhase = ''
        unzip $src
        tar -xvf HydraHarp\ HHLib\ v${version}/Linux/64bit/hhlibv${version}-linux-64bit.tar.gz
    '';

    installPhase = ''
        mkdir -p $out/lib
        mkdir -p $out/include
        cp hhlibv${version}-linux-64bit/library/*.h $out/include
        cp hhlibv${version}-linux-64bit/library/hhlib.so $out/lib
    '';
}

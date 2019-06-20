{ stdenv, fetchurl, unzip, libusb1, patchelf }:
stdenv.mkDerivation rec {
    name = "hydraHarpLib-${version}";
    version = "3.0.0.2";
    buildInputs = [ unzip patchelf libusb1 ];

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
        mv hhlibv${version}-linux-64bit/library/hhlib.so $out/lib/libhh.so
        patchelf --replace-needed libusb-0.1.so.4 ${libusb1}/lib/libusb.so $out/lib/libhh.so
    '';
}

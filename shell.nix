let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz); 
in
	with import <nixpkgs> { overlays = [ moz_overlay ]; };

	stdenv.mkDerivation {
	  name = "rust-env";
	  buildInputs = [
	    rustChannels.stable.rust
	    rustChannels.stable.cargo
      llvmPackages.libclang
	  ];
    LIBCLANG_PATH = "${llvmPackages.libclang}/lib";
	}


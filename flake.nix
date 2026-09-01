{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
		flake-utils.url = "github:numtide/flake-utils";
	};

	outputs = { self, nixpkgs, flake-utils, ... }:
	flake-utils.lib.eachDefaultSystem (system:
		let
			pkgs = import nixpkgs { inherit system; };
		in {
			devShells.default = pkgs.mkShell {
				buildInputs = with pkgs; [
					rustc
					cargo
				];

				shellHook = ''
					OLD_HOME=$HOME
					export HOME="$PWD/.nix-cache"


					export PS1="\[\e[0;34m\]\w\n\[\e[0;32m\]\$\[\e[00m\] "
				'';
			};
		}
	);
}

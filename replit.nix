{ pkgs }: {
	deps = [
        pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
        pkgs.openssl
        pkgs.pkg-config
	];
}
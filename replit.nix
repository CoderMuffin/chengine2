{ pkgs }: {
	deps = [
		pkgs.pkg-config
        pkgs.openssl
        pkgs.cargo-flamegraph
        pkgs.linuxPackages_hardened.perf
        pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}
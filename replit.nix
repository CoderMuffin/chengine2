{ pkgs }: {
	deps = [
		pkgs.cargo-flamegraph
  pkgs.linuxPackages_hardened.perf
  pkgs.rustc
		pkgs.rustfmt
		pkgs.cargo
		pkgs.cargo-edit
        pkgs.rust-analyzer
	];
}
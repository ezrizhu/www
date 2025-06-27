---
Title: Embedded rust development on esp32 with flakes
Description: My first time doing rust embedded development on nix flakes.
Tags: 
  - embedded
  - esp32
  - nix
  - flakes
  - rust

---

So I started doing embedded rust dev on riscv esp32. However it was a hell
getting the development environment working with nix flakes. After hours of
suffering, here it is. I consulted a *lot* of online resources, they are also
listed below.

```nix
{
  description = "EMLSS dev env";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-esp-dev.url = "github:mirrexagon/nixpkgs-esp-dev";
  };
  outputs = { self, nixpkgs, rust-overlay, flake-utils, nixpkgs-esp-dev, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [
          rust-overlay.overlays.default
          nixpkgs-esp-dev.overlays.default
        ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rustToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain: toolchain.default.override {
          extensions = [ "rust-src" "miri" "rustfmt" ];
          targets = ["riscv32imc-unknown-none-elf"];
        });
        espIdf = pkgs.esp-idf-full;
      in
      {
        devShells.default = pkgs.mkShell {
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          buildInputs = with pkgs; [
            openssl
            pkg-config
            fd
            rustToolchain
            espflash
            esp-generate
            ldproxy
            python3
            cmake
            ninja
            python311
            python3Packages.pip
            python3Packages.virtualenv
            espIdf
            probe-rs-tools
          ];
          shellHook = ''
          export ESP_IDF_TOOLS_INSTALL_DIR=fromenv
          export PATH="$IDF_PYTHON_ENV_PATH/bin:$PATH"
          '';
        };
      }
    );
}
```

Further reading
* [Nix Wiki: ESP-IDF](https://wiki.nixos.org/wiki/ESP-IDF)
* [nixpkgs-esp-dev](https://github.com/mirrexagon/nixpkgs-esp-dev)
* [fenix issue comment on custom
  toolchain](https://github.com/nix-community/fenix/issues/58#issuecomment-2156056797)
* [rust-overlay](https://github.com/oxalica/rust-overlay)
* [eclss flakes](https://github.com/hawkw/eclss/blob/main/flake.nix)

This is a short one, I will write more posts about my embedded projects in the
future :)

{ pkgs, config, ... }: {
  languages.rust.enable = true;

  packages = with pkgs; [
    bacon
    cargo-edit
    rust-analyzer
    cargo-deny
    cargo-machete
    cargo-llvm-cov
    release-plz
  ];

  enterShell = ''
    export LLVM_COV="${pkgs.rustc.llvmPackages.llvm}/bin/llvm-cov"
    export LLVM_PROFDATA="${pkgs.rustc.llvmPackages.llvm}/bin/llvm-profdata"
  '';
}

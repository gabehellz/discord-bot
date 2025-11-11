{ pkgs, ... }:

{
  packages = [
    pkgs.openssl
    pkgs.sqlx-cli
  ];

  dotenv.enable = true;
  
  languages.rust = {
    enable = true;
    channel = "stable";

    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "rust-docs"
      "rust-src"
    ];
  };

  git-hooks.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
  
  enterTest = ''
    cargo test
  '';
}

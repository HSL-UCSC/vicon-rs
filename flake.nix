{
  description = "Rust development environment with Cargo dependencies";

  # Specify the inputs, such as nixpkgs
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  # Define the outputs
  outputs = { self, nixpkgs, flake-utils }: flake-utils.lib.eachDefaultSystem (system: let
    pkgs = import nixpkgs { inherit system; };
  in {
    # Define a devShell for development
    devShell = pkgs.mkShell {
      # Add Rust and Cargo to the environment
      buildInputs = [
        pkgs.clang
        pkgs.cmake
        pkgs.rust
        pkgs.zsh
      ];

      # Optionally, set environment variables
      CARGO_HOME = "./.cargo";
      RUST_BACKTRACE = "1"; # Enable backtrace for debugging

      # Optional shellHook to fetch dependencies when entering the shell
      shellHook = ''
        export GIT_CONFIG=$PWD/.gitconfig
        export CARGO_NET_GIT_FETCH_WITH_CLI=true
        export GIT_SSH_COMMAND="ssh -F ~/.ssh/config"  # Ensure it uses your SSH config
        export LD_LIBRARY_PATH=${PWD}/vendor/libvicon:$LD_LIBRARY_PATH
        echo "LD_LIBRARY_PATH is set to: $LD_LIBRARY_PATH"
        # Start Zsh if not already the active shell
        if [ "$SHELL" != "$(command -v zsh)" ]; then
          export SHELL="$(command -v zsh)"
          exec zsh
        fi
        echo "Entering Rust development environment..."
        cargo fetch # Pre-fetch dependencies defined in Cargo.toml
      '';
    };
  });
}

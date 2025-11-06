fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Only compile protos if gRPC feature is enabled
    #[cfg(feature = "grpc")]
    {
        // Check if we should skip proto compilation
        if std::env::var("SKIP_PROTO").is_ok() {
            println!("cargo:warning=Skipping protobuf compilation (SKIP_PROTO set)");
            return Ok(());
        }

        // Try to compile gRPC proto files
        tonic_build::configure()
            .build_server(true)
            .build_client(false)
            .compile_protos(&["proto/patterns.proto"], &["proto"])
            .map_err(|e| {
                eprintln!("\n⚠️  WARNING: Failed to compile protobuf files");
                eprintln!("   Error: {}", e);
                eprintln!("   To install protobuf compiler:");
                eprintln!("     macOS:  brew install protobuf");
                eprintln!("     Linux:  sudo apt-get install protobuf-compiler");
                eprintln!("   Or build without gRPC: cargo build --no-default-features\n");
                e
            })?;

        println!("cargo:rerun-if-changed=proto/patterns.proto");
        Ok(())
    }

    #[cfg(not(feature = "grpc"))]
    {
        println!("cargo:warning=Building without gRPC support (feature disabled)");
        Ok(())
    }
}

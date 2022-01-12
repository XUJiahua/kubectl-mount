#!/bin/bash

echo "Building linux amd64 release"
cross build --release --target=x86_64-unknown-linux-gnu
echo "Copying linux amd64 release"
mkdir -p dist/kubectl-mount_linux_amd64 && cp target/x86_64-unknown-linux-gnu/release/kubectl-mount dist/kubectl-mount_linux_amd64

echo "Building macos amd64 release"
cross build --release --target=x86_64-apple-darwin
echo "Copying macos amd64 release"
mkdir -p dist/kubectl-mount_darwin_amd64 && cp target/x86_64-apple-darwin/release/kubectl-mount dist/kubectl-mount_darwin_amd64

echo "Building windows amd64 release"
cross build --release --target=x86_64-pc-windows-gnu
echo "Copying windows amd64 release"
mkdir -p dist/kubectl-mount_windows_amd64 && cp target/x86_64-pc-windows-gnu/release/kubectl-mount.exe dist/kubectl-mount_windows_amd64

# Linux Packaging Guide

This document describes how to package `bump2version` for major Linux distributions like Debian/Ubuntu and RedHat/Fedora.

## Debian/Ubuntu (.deb)

The `debian/` directory handles building `.deb` packages using standard `debhelper` and `cargo` infrastructure.

### Build Instructions

1. Install build dependencies:
   ```sh
   sudo apt-get update
   sudo apt-get install build-essential debhelper dh-cargo devscripts pkg-config libssl-dev cargo rustc
   ```
1. Build the package in the root directory:
   ```sh
   debuild --preserve-envvar PATH -us -uc -b
   ```
1. Install the generated package:
   ```sh
   sudo dpkg -i ../bump2version_0.2.2-1_amd64.deb
   ```
1. Verify the installation:
   ```sh
   bump --version
   cargo-bump --version
   ```

## RedHat/Fedora (.rpm)

The `rpm/` directory manages `.rpm` package configurations through the `bump2version.spec` file.

### Build Instructions

1. Install build dependencies:
   ```sh
   sudo dnf install rpm-build rpmdevtools cargo rust pkgconfig openssl-devel
   ```
1. Set up the `rpmbuild` workspace tree:
   ```sh
   rpmdev-setuptree
   cp rpm/bump2version.spec ~/rpmbuild/SPECS/
   ```
1. Archive the module and place it into the `SOURCES` directory:
   ```sh
   tar -czvf ~/rpmbuild/SOURCES/bump2version-0.2.2.tar.gz \
     --transform "s,^\.,bump2version-0.2.2," \
     --exclude=.git .
   ```
1. Build the RPM:
   ```sh
   rpmbuild -bb --nodeps ~/rpmbuild/SPECS/bump2version.spec
   ```
1. Install the built RPM:
   ```sh
   sudo dnf install ~/rpmbuild/RPMS/x86_64/bump2version-0.2.2-1.x86_64.rpm
   ```
1. Verify:
   ```sh
   bump --version
   ```

## Automated Releases

Both packages are built and published automatically via [`.github/workflows/linux-publish.yml`](.github/workflows/linux-publish.yml) on every `v*` tag push. The workflow uploads the built `.deb` and `.rpm` files directly to the GitHub Release.

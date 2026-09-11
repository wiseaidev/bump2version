%global debug_package %{nil}

Name:    bump2version
Version: 0.2.1
Release: 1%{?dist}
Summary: High-performance version bumper CLI written in Rust
License: MIT
URL:     https://github.com/wiseaidev/bump2version
Source0: bump2version-%{version}.tar.gz

BuildRequires: cargo
BuildRequires: rust
BuildRequires: openssl-devel
BuildRequires: pkgconfig

Requires: openssl

%description
bump2version (bump) is a fully compatible, Rust-native reimplementation of
bumpversion/bump-my-version. It manages version numbers across any file type,
creates git commits and tags, and supports Cargo workspaces, pre-release
cycling, watch mode, and multi-language manifest auto-detection.

%prep
%autosetup -n bump2version-%{version}

%build
cargo build --release --features=rust-binary

%install
install -Dpm 0755 target/release/bump -t %{buildroot}%{_bindir}/
install -Dpm 0644 README.md -t %{buildroot}%{_docdir}/%{name}/
ln -sf %{_bindir}/bump %{buildroot}%{_bindir}/cargo-bump

%files
%license LICENSE
%doc %{_docdir}/%{name}/README.md
%{_bindir}/bump
%{_bindir}/cargo-bump

%changelog
* Fri Sep 11 2026 Mahmoud Harmouch <oss@wiseai.dev> - 0.2.1-1
- Initial release with workspace, watch, and detect features

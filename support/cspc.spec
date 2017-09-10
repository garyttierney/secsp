%global crate cspc
%global repo rust-csp
%global author garyttierney

Name:           %{crate}
Version:        0.1.0
Release:        1%{?dist}
Summary:        Compiler for the C-style SELinux policy langauge. 

License:        MIT
URL:            https://github.com/%{author}/%{repo}
Source0:        rust-csp.tar.gz

ExclusiveArch: x86_64
BuildRequires: cargo rust

%description
High-level SELinux policy language.

%prep
%setup -q

%build
cargo build --release

%install
mkdir -p %{buildroot}/%{_bindir}
cp target/release/cspc %{buildroot}/%{_bindir}

%files
%{_bindir}/cspc

%changelog
* Sun Sep 03 2017 Gary Tierney <gary.tierney@gmx.com> - 0.1.0-1
- Initial package


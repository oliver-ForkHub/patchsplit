Name:           patchsplit
Version:        1.0.4
Release:        1%{?dist}
Summary:        A tool for splitting patch files

License:        MIT
URL:            https://github.com/zitzhen/patchsplit

Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo

%description
A command-line tool for splitting patch files.

%prep
%autosetup

%build
cargo build --release --locked

%install
install -Dm755 target/release/patchsplit \
    %{buildroot}%{_bindir}/patchsplit

%files
%license LICENSE
%{_bindir}/patchsplit

%changelog
* Sun Sep 13 2026 Oliver Lin <oliver@liuxiaozhen.dev> - 1.0.4-1
- Initial RPM package
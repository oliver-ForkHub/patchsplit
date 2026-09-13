Name:           patchsplit
Version:        1.0.4
Release:        1%{?dist}

Summary:        A tool for splitting patch files
Group:          Development/Tools

License:        MIT
Packager:       Oliver Lin <oliver@liuxiaozhen.dev>
URL:            https://github.com/zitzhen/patchsplit
Source0:        %{name}-%{version}.tar.gz

BuildRequires:  rust
BuildRequires:  cargo

%description
Patchsplit is a command-line tool for splitting patch files into
separate files.

%prep
%autosetup

%build
cargo build --release --locked

%check
cargo test --release --locked

%install
install -Dm755 target/release/patchsplit \
    %{buildroot}%{_bindir}/patchsplit

install -Dm644 packaging/patchsplit.1 \
    %{buildroot}%{_mandir}/man1/patchsplit.1

%files
%license LICENSE
%doc README.md
%{_bindir}/patchsplit
%{_mandir}/man1/patchsplit.1*

%changelog
* Sun Sep 13 2026 Oliver Lin <oliver@liuxiaozhen.dev> - 1.0.4-1
- Initial RPM package
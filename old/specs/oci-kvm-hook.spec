Name: oci-kvm-hook
Summary: %{name}
Version: 0
Release: 1%{?dist}
Source0: %{name}-%{version}.tar.xz
License: ASL 2.0
BuildRequires: git
BuildRequires: golang
BuildRequires: /usr/bin/go-md2man

%description
%{summary}

%prep
%autosetup -Sgit

%build
make

%install
make install DESTDIR=%{buildroot}

%files
%license LICENSE
%{_mandir}/man1/*
%{_prefix}/libexec/oci/hooks.d/*

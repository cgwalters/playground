Name: oci-kvm-hook
Summary: %{name}
Version: 0
Release: 1%{?dist}
Source0: %{name}-%{version}.tar.xz
License: ASLv2
BuildRequires: git

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

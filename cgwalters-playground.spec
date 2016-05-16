Name: cgwalters-playground
Summary: %{name}
Source0: %{name}-%{version}.tar.xz
License: LGPLv2+
BuildArch: noarch
BuildRequires: git

%description
%{summary}

%prep
%autosetup -Sgit

%build

%install
mkdir -p $RPM_BUILD_ROOT/%{_bindir}
cp nova-list-ips.py $RPM_BUILD_ROOT/%{_bindir}

%files
%license COPYING
%doc testing.md
%{_bindir}/*.py

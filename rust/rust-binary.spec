# A spec file for creating an RPM from the official Rust
# binaries.  Based on the spec file extracted from the
# SRPM in https://copr.fedoraproject.org/coprs/fabiand/rust-binary/builds/

%global rust_version 1.1.0
%global rust_version_upstream 1.1.0
%global archsuffix x86_64-unknown-linux-gnu

%global debug_package %{nil}
# Do not check any files in docdir for requires
%global __requires_exclude_from ^%{_bindir}/.*$

Name:           rust-binary
Version:        %{rust_version}
Release:        2%{?dist}
Summary:        The Rust Programming Language (official static build)

License:        ASL 2.0, MIT
URL:            http://www.rust-lang.org
Source0:        https://static.rust-lang.org/dist/rust-%{rust_version_upstream}-%{archsuffix}.tar.gz

ExclusiveArch:  x86_64


%description
This is a compiler for Rust, including standard libraries, tools and
documentation.
This package is wrapping the official binary builds.


%prep
%setup -q -n rust-%{rust_version_upstream}-%{archsuffix}


%build
# Nothing

%install
./install.sh \
    --prefix=%{buildroot}/%{_prefix} --libdir=%{buildroot}/%{_libdir} \
    --disable-verify

# Create ld.so.conf file
mkdir -p %{buildroot}/%{_sysconfdir}/ld.so.conf.d
cat <<EOF | tee /%{buildroot}/%{_sysconfdir}/ld.so.conf.d/rust-%{_target_cpu}.conf
%{_libdir}/rustlib/
%{_libdir}/rustlib/%{_target_cpu}-unknown-linux-gnu/lib/
EOF

# Remove buildroot from manifest
sed -i "s#^\(file\|dir\):%{buildroot}##" %{buildroot}/%{_libdir}/rustlib/manifest*
# Blow away the bash/zsh completions for now
rm %{buildroot}/%{_prefix}/etc/ -rf
rm %{buildroot}/%{_datadir}/zsh/ -rf
# And we don't need this
rm %{buildroot}/%{_libdir}/rustlib/install.log


%post -p /sbin/ldconfig


%files
%doc COPYRIGHT LICENSE-APACHE LICENSE-MIT README.md
%{_sysconfdir}/ld.so.conf.d/rust-*.conf
%{_bindir}/rustc
%{_bindir}/cargo
%{_bindir}/rust-*
%{_bindir}/rustdoc
%{_libdir}/lib*
%{_libdir}/rustlib/*
%{_datadir}/man/*
%{_datadir}/doc/*


%changelog
* Sat Jun 27 2015 Colin Walters <walters@verbum.org> - 1.1.0-2
- Update to 1.1.0

* Sun Dec 28 2014 Fabian Deutsch <fabiand@fedoraproject.org> - 0.12.0-1
- Update to 0.12.0

* Sat Jul 05 2014 Fabian Deutsch <fabiand@fedoraproject.org> - 0.11.0-1
- Initial package


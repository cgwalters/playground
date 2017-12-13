Server side (just informational)
------

You don't have to do any of this; I'm just showing what I did. Skim this and
skip to the Client section below.

`$ git clone fedora-atomic`

```
{
    "ref": "walters/fedora/26-gold/${basearch}/atomic-host",
    "include": "fedora-atomic-host-base.json",
    "repos": ["fedora-26"]
}
```

```
$ rpm-ostree compose tree -r repo-build --ex-unified-core --cache-only --cachedir cache ~user/src/pagure/fedora-atomic/fedora-atomic-host.json                              
No previous commit for walters/fedora/26-gold/x86_64/atomic-host                                                                                                                                                   
Enabled rpm-md repositories: fedora-26       
...
walters/fedora/26-gold/x86_64/atomic-host => f8fb70ba9d6b23b79cc80823c0d16e97bca78da35e3ee81dc05c6139635d16a4
```

```
$ rpm-ostree ex commit2jigdo --repo repo-build --pkgcache-repo cache/pkgcache-repo  f8fb70ba9d6b23b79cc80823c0d16e97bca78da35e3ee81dc05c6139635d16a4 ~user/src/pagure/fedora-atomic/fedora-atomic-host.spec $(pwd)/demo-jigdo-out
```

```
$ rpm -qp --provides demo-jigdo-out/x86_64/fedora-atomic-host-26-1.fc27.x86_64.rpm
fedora-atomic-host = 26-1.fc27                      
fedora-atomic-host(x86-64) = 26-1.fc27              
rpmostree-jigdo(v3)                                 
rpmostree-jigdo-commit(f8fb70ba9d6b23b79cc80823c0d16e97bca78da35e3ee81dc05c6139635d16a4)  
$ 
```


Client
------

We will demonstrate downloading the F26 "gold" RPMs, plus a special "jigdoRPM"
[from my fedorapeople](https://fedorapeople.org/~walters/f26-jigdo-demo/).  Using
`rpm-ostree ex jigdo2commit` we will download *bit for bit* the "image" or OSTree
commit that I built server side.

Preparation (inside a F27 container or host):

 - Enable https://copr.fedorainfracloud.org/coprs/walters/rpm-ostree-dev (or build git master)
 - `yum -y install rpm-ostree`

As non-root (or root if you really want to):

```
ostree --repo=repo init --mode=bare-user
# Setting releasever to 26 works around a bug; specifying --releasever should work but does not
sed -i -e 's,$releasever,26,' < /etc/yum.repos.d/fedora.repo > fedora.repo
cat > walters-jigdo-demo.repo << EOF
[walters-jigdo-demo]
baseurl=https://fedorapeople.org/~walters/f26-jigdo-demo/
gpgcheck=0
EOF
rpm-ostree ex jigdo2commit --repo repo -d $(pwd) -e fedora -e walters-jigdo-demo walters-jigdo-demo:fedora-atomic-host
ostree --repo=repo cat f8fb70ba9d6b23b79cc80823c0d16e97bca78da35e3ee81dc05c6139635d16a4 /usr/lib/os-release
cowsay We download the gold Fedora 26 RPMs, plus one extra special "jigdoRPM"
cowsay But the *exact same checksum* results
cowsay If I update the jigdoRPM, you only download changed RPMs
```

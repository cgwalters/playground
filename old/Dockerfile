FROM registry.fedoraproject.org/fedora:28
WORKDIR /src
COPY ./* /src/
COPY .git/ /src/.git/
RUN for x in .git ostree-releng-scripts; do ls -al $x; done && date > foo

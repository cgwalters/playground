FROM fedora
RUN yum -y install npm && yum clean all
RUN useradd npm
USER npm
RUN cd && npm install doctoc




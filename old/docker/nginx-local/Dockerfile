FROM fedora:25
RUN yum -y install nginx && yum clean all
RUN echo "daemon off;" >> /etc/nginx/nginx.conf && \
    echo "nginx on Fedora" > /usr/share/nginx/html/index.html && \
    mkdir -m 0755 -p /etc/pki/nginx
COPY ssl.conf /etc/nginx/conf.d/ssl.conf
COPY local-wildcard.* /etc/pki/nginx/

EXPOSE 80
EXPOSE 443

CMD [ "/usr/sbin/nginx" ]

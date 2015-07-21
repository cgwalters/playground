FROM centos:7
RUN yum -y install httpd git
RUN rm -rf /var/www/html && cd /var/www/ && git clone --depth=1 https://github.com/jbt/markdown-editor html
RUN sed -i -e 's,Listen 80,Listen 8080,' /etc/httpd/conf/httpd.conf
RUN chown -R -h apache:apache /var/log/httpd
EXPOSE 8080
USER apache
CMD ["/usr/sbin/httpd", "-D", "FOREGROUND"]


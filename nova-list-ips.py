#!/usr/bin/env python

from __future__ import print_function

import os
import novaclient.v2.client as nvclient
import argparse
from keystoneclient.auth.identity import v2
from keystoneclient import session
from novaclient import client

parser = argparse.ArgumentParser(description='Generate /etc/hosts data from OpenStack nova')
parser.add_argument('--net-prefix', action='store', help='Only emit IP addresses with this prefix')
parser.add_argument('--prefix', action='store', help='Only emit hosts with this prefix')
args = parser.parse_args()


auth = v2.Password(auth_url=os.environ['OS_AUTH_URL'],
                   username=os.environ['OS_USERNAME'],
                   password=os.environ['OS_PASSWORD'],
                   tenant_name=os.environ['OS_TENANT_NAME'])
sess = session.Session(auth=auth)
nova = client.Client('2', session=sess)

address_hosts={}
for server in nova.servers.list():
    if args.prefix and not server.name.startswith(args.prefix):
        continue
    for (network,ips) in server.networks.iteritems():
        for ip in ips:
            if (args.net_prefix is None or 
                ip.startswith(args.net_prefix)):
                address_hosts[ip] = server
for address,server in address_hosts.iteritems():
    print('{0} {1}  # id={2}'.format(address, server.name, server.id))

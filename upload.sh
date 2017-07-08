#!/bin/sh

cd _out

#new digitalocean thing
tar czf - . | ssh earendil.kaashif.co.uk 'cat - > ~/site.tgz; doas rm -rf /var/www/htdocs/*; doas tar -C /var/www/htdocs -xvzf site.tgz'

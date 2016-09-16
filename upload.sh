#!/bin/sh

cd _out

#old openshift thing
#git init
#git --git-dir=./.git add .
#git --git-dir=./.git commit -m 'rebuilt'
#git --git-dir=./.git push -f ssh://55193ee3fcf9334054000012@blog-kaashif.rhcloud.com/~/git/blog.git/ master

#new ec2 thing
tar czf - . | ssh -i "~/.important/new-blog-aws.pem" ubuntu@ec2-52-211-56-49.eu-west-1.compute.amazonaws.com 'cat - > ~/site.tgz; sudo rm -rf /var/www/html/*; sudo tar -C /var/www/html -xvzf site.tgz'

#!/bin/sh

cd _out

#old openshift thing
#git init
#git --git-dir=./.git add .
#git --git-dir=./.git commit -m 'rebuilt'
#git --git-dir=./.git push -f ssh://55193ee3fcf9334054000012@blog-kaashif.rhcloud.com/~/git/blog.git/ master

#new ec2 thing
tar czf - . | ssh -i "~/misc/important/blog-aws.pem" ubuntu@ec2-52-16-74-211.eu-west-1.compute.amazonaws.com 'cat - > ~/site.tgz; sudo rm -rf /usr/share/nginx/html/*; sudo tar -C /usr/share/nginx/html -xvzf site.tgz'

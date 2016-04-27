#!/bin/sh

cd _out
git init
git --git-dir=./.git add .
git --git-dir=./.git commit -m 'rebuilt'
git --git-dir=./.git push -f ssh://55193ee3fcf9334054000012@blog-kaashif.rhcloud.com/~/git/blog.git/ master


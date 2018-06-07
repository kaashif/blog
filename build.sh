#!/bin/sh -xe

rm -rf _out/

#set archive to the index
cp _src/archive.html _src/index-template.html

#set the posts per page to infinity for the archive
sed -i 's/posts-per-page = 10/posts-per-page = 9999/g' .frogrc

#build the site
raco frog -b

#move index page to archive
mv _out/index.html _out/archive.html

#change the posts-per-page back
sed -i 's/posts-per-page = 9999/posts-per-page = 10/g' .frogrc

#set index to the index
rm _src/index-template.html
cp _src/index.html _src/index-template.html

#build the site again to get the normal index
raco frog -b

#delete the copied index
rm _src/index-template.html

#copy the other dirs
cp -r css _out/
cp -r static _out/

#copy keybase
cp keybase.txt _out/
cp favicon.ico _out/

#preview the site
raco frog -p

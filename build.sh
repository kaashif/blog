#!/bin/sh
rm -rf _out/

#set archive to the index
cp _src/archive.html _src/index-template.html

#build the site
raco frog -b

#move index page to archive
mv _out/index.html _out/archive.html

#set index to the index
rm _src/index-template.html
cp _src/index.html _src/index-template.html

#build the site again to get the normal index
raco frog -b

#delete the copied index
rm _out/index-template.html

#copy the other dirs
cp -r css _out/
cp -r static _out/

#preview the site
raco frog -p


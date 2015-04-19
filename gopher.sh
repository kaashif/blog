#!/usr/bin/env zsh

if [ "x$1" = "xupload" ]; then
	if ! [ -f "gopher/gophermap" ]; then
		echo Please run without any arguments first
		exit 1
	fi
	rsync -avz gopher/ elwe:/var/gopher/
	exit 0
fi

rm -rf gopher
cp -r posts gopher
figlet -w 71 Kaashif\'s Gopher Hole > gopher/gophermap

cat >> gopher/gophermap <<EOF

There isn't actually all that much stuff here, and what
is here is actually just the source from my blog
crudely converted from Markdown into HTML then dumped
into text using `lynx -dump` so I wouldn't lose all
the formatting. Mostly, it hasn't worked too well.

See below for some blog posts:

EOF

for post in gopher/*.markdown; do
	tmpp=${post/markdown/tmp}
	htmlp=${post/markdown/html}
	txtp=${post/markdown/txt}
	awk 'NR >= 6 { print }' <${post} >${tmpp}
	markdown "${tmpp}" > ${post/markdown/html}
	lynx -dump "${htmlp}" > ${txtp}
	rm ${htmlp} ${tmpp}
done
rm gopher/*.markdown

cd gopher
for post in *.txt; do
cat >> gophermap << EOF
0${post}	/${post}
EOF
done


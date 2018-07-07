ShareLaTeX on OpenBSD
2014-12-06

The other day, I was trying to access <http://sharelatex.com> at
school, and it didn't really work, probably due to a combination of
Internet Explorer and possibly an overzealous filter that could have
been blocking something. That's what I thought, anyway, until I tried
it on Chrome and it still didn't work. Odd. The best solution was
obviously to set up my own ShareLaTeX instance on my server.
<!-- more -->

##Dependencies
On the
[ShareLaTeX wiki page](https://github.com/sharelatex/sharelatex/wiki/Dependencies)
about dependencies, I saw that I needed Node.js, Grunt, Redis,
MongoDB, Aspell, and TeXLive. All of these are packaged in OpenBSD and
they can all be easily installed:

<pre><code># pkg_add node redis mongodb texlive_texmf-full aspell latexmk
</code></pre>

After that, I installed Grunt using npm:

<pre><code># npm install -g grunt-cli
</code></pre>
	
That's that. I read through the instructions and they said I needed to
configure MongoDB, but that's not actually necessary.

##The hard part
I chose to install into /var/www, as per the recommendations on the
wiki, so I cloned the repo into /var/www/sharelatex and ran the
commands. It's important to note that Grunt expects the make binary in
the PATH to be GNU make, and I couldn't be bothered to find some way
to change this expectation, so I moved BSD make and symlinked GNU make
in its place:

<pre><code># mv /usr/bin/make /usr/bin/bmake
# ln -s /usr/local/bin/gmake /usr/bin/make
</code></pre>

Now I was ready to install billions of npm packages and let Grunt set
up trillions of config files:

<pre><code># git clone -b release \
https://github.com/sharelatex/sharelatex.git \
/var/www/sharelatex
# cd /var/www/sharelatex
# npm install
# grunt install
</code></pre>

That shouldn't show too many errors. Now, I followed the rest of the
instructions on the wiki page I linked earlier. In case you're too
lazy to go there, they're reproduced below (and edited for BSD):

Make a sharelatex user, and chown all files to it:

<pre><code># useradd -b /var/www/sharelatex -G sharelatex sharelatex
# chown -R sharelatex:sharelatex /var/www/sharelatex
</code></pre>

Move the config files to a better place:

<pre><code># mkdir /etc/sharelatex
# mv /var/www/sharelatex/config/settings.development.coffee \
/etc/sharelatex/settings.coffee
</code></pre>

Edit that config file and make sure the dir variables read as follows:

	DATA_DIR = '/var/lib/sharelatex/data'
	TMP_DIR  = '/var/lib/sharelatex/tmp'

Make all of the directories:

<pre><code># mkdir -p /var/lib/sharelatex/data/&#123;user_files,compiles,cache&#125;
# mkdir -p /var/lib/sharelatex/tmp/&#123;uploads,dumpFolder&#125;
# chown -R sharelatex:sharelatex /var/lib/sharelatex
</code></pre>

That's it, there are only a few things left to do.

##A problem and a fix
I tried to run ShareLaTeX, and it wouldn't work. It seems like
there's a well-known bug in Node.js (or something like that) that
causes it to fail unless you do the following:

<pre><code># cd /var/www/sharelatex
# rm -rf web/node_modules/bcrypt
# npm install
</code></pre>

Apparently, it's something to do with dependencies, but it doesn't
matter, I just ran the above command and everything ended up working.

##Init script
Obviously, the Upstart script supplied is impossible to use on
OpenBSD, but it's not too hard to whip up an rc.d script. Here's the
one I use:

<pre><code>#!/bin/sh
# /etc/rc.d/sharelatex
daemon=/usr/bin/tmux
daemon_user=sharelatex
daemon_flags=&quot;new-session -s sharelatex -d &#39;grunt run&#39;&quot;

. /etc/rc.d/rc.subr

rc_stop()&#123;
	$&#123;rcexec&#125; &quot;sudo -u sharelatex $&#123;daemon&#125; kill-server&quot;
&#125;

rc_cmd $1
</code></pre>

Obviously, it's a bit primitive, since it just runs in tmux with no
logging and is killed by killing tmux, but it does work, and you can
start and stop it.

The last thing I needed to do is to add everything to
/etc/rc.conf.local:

<pre><code>pkg_scripts=redis mongod sharelatex
</code></pre>

And start it up with:

<pre><code># /etc/rc.d/sharelatex start
</code></pre>

Now, it should work. If you've been following along, browse to port
3000 on your server to check it out. I advise setting up Apache or
Nginx (or some other web server) as a reverse proxy and using TLS to
access it. Configuring web servers isn't within the scope of this
post, though.

Check out <http://sharelatex.com>, though, it's really cool. Maybe
you'll want to set it up yourself, too (if you haven't already)!

---
title: ShareLaTeX on OpenBSD
comment: Setting up a self-hosted ShareLaTeX instance on an OpenBSD server
date: 2014-06-12
---

The other day, I was trying to access <http://sharelatex.com> at
school, and it didn't really work, probably due to a combination of
Internet Explorer and possibly an overzealous filter that could have
been blocking something. That's what I thought, anyway, until I tried
it on Chrome and it still didn't work. Odd. The best solution was
obviously to set up my own ShareLaTeX instance on my server.

##Dependencies
On the
[ShareLaTeX wiki page](https://github.com/sharelatex/sharelatex/wiki/Dependencies)
about dependencies, I saw that I needed Node.js, Grunt, Redis,
MongoDB, Aspell, and TeXLive. All of these are packaged in OpenBSD and
they can all be easily installed:

    # pkg_add node redis mongodb texlive_texmf-full aspell latexmk

After that, I installed Grunt using npm:

    # npm install -g grunt-cli
	
That's that. I read through the instructions and they said I needed to
configure MongoDB, but that's not actually necessary.

##The hard part
I chose to install into /var/www, as per the recommendations on the
wiki, so I cloned the repo into /var/www/sharelatex and ran the
commands. Before you do this, it's important to note that Grunt
expects the make binary in the PATH to be GNU make, and I couldn't be
bothered to find some way to change this expectation, so I moved BSD
make and symlinked GNU make in its place:

    # mv /usr/bin/make /usr/bin/bmake
	# ln -s /usr/local/bin/gmake /usr/bin/make

Now I was ready to install billions of npm packages and let Grunt set
up trillions of config files:

    # git clone -b release \
	https://github.com/sharelatex/sharelatex.git \
	/var/www/sharelatex
	# npm install
	# grunt install

That shouldn't show too many errors. Now, I followed the rest of the
instructions on the wiki page I linked earlier. In case you're too
lazy to go there, they're reproduced below (and edited for BSD):

Make a sharelatex user, and chown all files to it:

    # useradd -b /var/www/sharelatex -G sharelatex sharelatex
	# chown -R sharelatex:sharelatex /var/www/sharelatex

Move the config files to a better place:

    # mkdir /etc/sharelatex
	# mv /var/www/sharelatex/config/settings.development.coffee \
	/etc/sharelatex/settings.coffee

Edit that config file and make sure the dir variables read as follows:

    DATA_DIR = '/var/lib/sharelatex/data'
    TMP_DIR  = '/var/lib/sharelatex/tmp'

Make all of the directories:

    # mkdir -p /var/lib/sharelatex/data/{user_files,compiles,cache}
    # mkdir -p /var/lib/sharelatex/tmp/{uploads,dumpFolder}
	# chown -R sharelatex:sharelatex /var/lib/sharelatex

That's it, there are only a few things left to do.

##A problem and a fix
If you tried to run ShareLaTeX now, it wouldn't work. It seems like
there's a well-known bug in Node.js (or something like that) that
causes it to fail unless you do the following:

    # cd /var/www/sharelatex
	# rm -rf web/node_modules/bcrypt
	# npm install

Apparently, it's something to do with dependencies, but it doesn't
matter, just make sure you apply the fix above.

##Init script
Obviously, the Upstart script supplied is impossible to use on
OpenBSD, but it's not too hard to whip up an rc.d script. Here's the
one I use:

    #!/bin/sh
	# /etc/rc.d/sharelatex
	daemon=/usr/bin/tmux
	daemon_user=sharelatex
	daemon_flags="new-session -s sharelatex -d 'grunt run'"
	
	. /etc/rc.d/rc.subr
	
	rc_stop(){
        ${rcexec} "sudo -u sharelatex ${daemon} kill-server"
	}

    rc_cmd $1

Obviously, it's a bit primitive, since it just runs in tmux with no
logging and is killed by killing tmux, but it does work, and you can
start and stop it.

The last thing you need to do is to add everything to
/etc/rc.conf.local:

    pkg_scripts=redis mongod sharelatex

And start it up with:

    # /etc/rc.d/sharelatex start

Now, it should work. Browse to port 3000 on your server to check it
out. I advise setting up Apache or Nginx (or some other web server) as
a reverse proxy and using TLS to access it. Configuring web servers
isn't within the scope of this post, though.

Check out <http://sharelatex.com>, though, it's really cool. Maybe
you'll want to set it up yourself, too!

OpenShift vs Heroku: Haskell
2015-03-30

There are a few Platform as a Service (PaaS) services out there, and
the most famous is probably Heroku. I know I've had people come up to
me and suggest using Heroku for the next big thing they're
planning. There is a problem with Heroku: it's not free (libre). That
didn't stop me from at least trying it out to see what all the fuss
was about.
<!-- more -->

##Deploying a Haskell app on Heroku

Obviously, it wasn't as easy as just `git push heroku master`, or this
post wouldn't exist. No, it was more complicated than that.

I couldn't just push my Happstack app (a toy app, now hosted at
<http://quenya.kaashif.co.uk>) to Heroku, so I had to do a bit of
searching around. The best way to do this was to create an app with
the buildpack set to either
<https://github.com/begriffs/heroku-buildpack-ghc> or
<https://github.com/mietek/haskell-on-heroku>. The latter was
apparently faster, so I tried that out.

Setting the buildpack was as easy as:

<pre><code>$ heroku buildpack:set https://github.com/mietek/haskell-on-heroku.git -a quenya
</code></pre>

Apparently, it _was_ now just as easy as pushing. So I tried:

<pre><code>$ git push heroku master
</code></pre>

And it failed, with the following errors:

	*** ERROR: Cannot build sandbox directory
	*** ERROR: Failed to deploy app
	*** ERROR: Deploying buildpack only

There was some other output, too, but those were the important
lines. This wasn't supposed to happen. After looking around on the
tutorial page, I see that this error occurs when you're trying to
build using a one-off dyno. Maybe I could still make it work!

<pre><code>$ heroku run -s 1X build
</code></pre>

And that failed, telling me private storage was expected (an S3
bucket). Well, I should've expected that, I suppose, considering the
errors I got when the push failed are only supposed to occur when
trying to use private storage.

Well anyway, let's try the other buildpack.

##Trying the other buildpack

I set the buildpack to the first one I looked at:

<pre><code>$ heroku buildpack:set https://github.com/begriffs/heroku-buildpack-ghc -a quenya
</code></pre>

That worked fine. Then, I pushed.

<pre><code>$ git push heroku master
</code></pre>

It seemed to be going well, GHC was downloaded, cabal-install was
downloaded, and it was about to run when suddenly:

	cabal: error while loading shared libraries: libgmp.so.3: cannot open shared object file: No such file or directory

And it was going so well! If I got this error on my machine, I'd just
do some dirty symlink trick to make cabal think it had the right
library, but I couldn't do that on Heroku, since I didn't have
permission to do anything.

Now that I think about it, this means that the haskell-on-heroku
buildpack would have probably failed too, even if I set up an S3
bucket for it.

Anyway, I'd had enough of Heroku and decided to try out OpenShift.

##Getting started with OpenShift

I visited <https://openshift.redhat.com> and signed up for a free
account. It was simple enough to make an application, I just had to
select a "cartridge" (basically a set of preinstalled libraries and
programs), add my SSH key, and pick a name.

There was already a Happstack cartridge which came with
happstack-server, blaze-html, and all the libraries I needed, bar a
few. Before pushing my app, I decided to install `rhc`, the command
line tool for OpenShift, like `heroku`.

It has some cool features, like `rhc port-forward`, which sets up a
tunnel so you can access the internal services (Redis, PostgreSQL etc)
from your own computer, on localhost. For example, if you run `rhc
port-forward`, you could access PostgreSQL on 127.0.0.1:5432. Cool,
right? I suppose Heroku must have something similar, but this isn't
supposed to be a balanced blog post.

Anyway, I sshed into the application server to check things out, and
it looked like a normal RHEL server, without anything weird or
virtual-looking (although it probably is virtual). I had a home
directory in /var/lib/openshift/, alongside a number of
others. `cabal` was actually installed, unlike on Heroku, so I didn't
need any "buildpacks": the build would take place exactly as it would
do on my computer.

All I had to do was `git push <really long uri>` and it worked. All
that happened was that `cabal run <host> <port>` was run (or something
to that effect), and it started the app. Simple, right? 

##Moving a static site to OpenShift

This is easy, although there isn't a "static site" cartridge. All you
have to do it pick a PHP cartridge, and Apache just serves the root
directory of the repository (with mod_php enabled). So I just
generated my blog, pushed to a new PHP app, and Apache served my
site. Easy.

##Conclusions

OpenShift is good, use it instead of Heroku. I can't speak for the
paid options, but it's a lot easier to deploy Haskell on OpenShift.

Note: I have almost no experience in using either, except for what
I've written in this article.

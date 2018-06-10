Ideas for a project
2014-04-18


There is always a lot of buzz around the idea of "learning to program".
While I think it's very important that children learn logical thinking
and problem solving, I also realise that the majority of children, and
people in general, would probably not benefit from a very language
specific, rote learning based, generally old-fashioned approach to
<!-- more -->
teaching programming. Those that would enjoy programming largely have
the aptitude to learn it themselves, with a bit of guidance. I think
that the best way to learn is to do - write programs that serve some
purpose or have some goal in mind - like a game or web app.

Obviously, my aim isn't to convert people who hate computers into
hackers, but to offer some ideas and advice to the people who are trying
to learn, but only find books and articles which walk them through
syntax and have some exercises, but no larger projects to complete to
draw together everything that they know.

Since writing simulations and AI programs are probably a bit beyond most
beginners, and boring system utilities are generally too tedious for a
beginner to feel motivated to write, I think the best direction to go in
for a project would be a game.

##But games aren't serious programming!

I've never heard anyone say the above, but someone might. If they did,
they'd be completely and utterly wrong. If you're trying to learn about
object oriented programming, a game is perfect! OOP is intuitive if you
think of the objects as real things (a player, enemy, level etc). That
might not be entirely in the spirit OOP was conceived in, but it works
for teaching the basics of OOP: encapsulation, polymorphism etc.

The jist of it is that when you tell an object to do something and fetch
an answer, you don't need to know how the answer is obtained, just the
answer - the functionality is _encapsulated_. In turn, this means that
you can change the internal structure of the object without
repercussions, provided the interface remains the same. Here's an
example, in Python.

<!--```python-->
<pre><code>class Enemy:
	def search(self):
		# Very long and complicated search algorithm
		return direction_moved

enemy = Enemy()

while True:
	direction = enemy.search()
	print "Enemy moved " + direction + " while searching for you"
</code></pre>

In the while loop, the Enemy instance's search method is called and a
result obtained. The person programming the while loop does not have to
know anything about search algorithms or how to write one - the search
method takes care of all of that. Even better, if the programmer who
wrote the search algorithm changes his mind about it, whoever wrote the
while loop doesn't have to change anything.

Games, while they don't have a reputation as serious business among
laymen, can be very useful as vehicles for learning about maths, physics
and, most importantly for me, computer science.

##Where should I start?

That small example, while sort of neat to someone who's never seen a
working program before, isn't even valid code, so you can't use it as a
starting point.

Before you start a project, I'd advise you learn how to program and
solve some simple exercises. A good website for this, is
[codeabbey.com](http://codeabbey.com). Their exercises are quite good
for getting to grips with the basics of a language, and get harder as
you go.

After you've done that, you can start thinking about what sort of game
or program you want to make. Since it's best not to over-complicate
things, I suggest you write a text-based game. Since that is a bit
broad, let me suggest an adventure game, like "adventure" or
"battlestar", which you may have heard of. You probably haven't, so
[here](http://i.imgur.com/aHa3cN3.png) is an example. That's a
screenshot of Zork, a classic text-based adventure game. Essentially,
you get a bit of description of your surroundings, then you can type in
commands like "go north" or "eat leaflet", and the game responds
accordingly. Not very hard to program, but very entertaining.

To make such a game, all you need is to be familiar with standard input
and output. Everything else is optional, but it's very easy for such a
game to be very deep and complex, both for the programmer and player,
believe it or not. Here is a very basic example, in Python.

<!--```python-->
<pre><code>print("Welcome to Generic Text Adventure!")

running = True

while running:
	command = input("Enter command: ")
	if command == "quit":
		running = False
	elif command == "die":
		print("You died!")
		running = False
	else:
		print("Command not recognised!")
</code></pre>
		
That's not a very fun game, but it's easy to see how it could be
extended using your programming knowledge. Perhaps the command processor
should be extracted into a separate function? Maybe you could put it
into an object of its own and load the list of command from a text file?
The possibilities are endless, and the end result will always be
something workable, because the game is so simple that you'll never get
into problems with graphics drivers or network latency or anything you
didn't create yourself.

If you're looking for a simple project and you haven't already made a
game - make one! It can't hurt you, and it might expose some weaknesses
in your knowledge or just make you feel better about yourself.

Decomposing representations: a slice of computational group theory
2020-04-19


For my Master's degree, I (helped greatly by my supervisor)
implemented some algorithms and even invented some new algorithms to
decompose representations of finite groups. I wrote an extremely long
(well, relative to other things I've written) and technical thesis
about this, but I find myself increasingly unable to understand what
any of it means or why I even have a degree.

I thought being forced into a short-form blog post would help me
remember whatever it is I spent a few years studying to do. There are
some foundational questions:

* What is a group?
* What is a representation?
* What is a decomposition of a representation?

And some more interesting questions, involving some computational
tricks relevant to a wider audience:

* Why is this useful?
* How do you get a computer to do it?
* How do you get a computer to do it, quickly?

These are the questions I'll attempt to answer in this blog
post. It'll be fun!
<!--more-->

## What is a group?

Any undergraduate mathematician should know this, but if you don't do
algebra and need a refresher,
[read the Wikipedia page](https://en.wikipedia.org/wiki/Group_(mathematics)). For
non-mathematicians, a group is:

* A set $G$ (for example, the real numbers or the rational numbers)

* With a binary operation $\ast$ (like addition or multiplication)
  which takes two elements of $G$ and gives you another element of $G$

* Such that $\ast$ is associative, meaning bracketing doesn't matter -
  $(a \ast b) \ast c = a \ast (b \ast c)$. True for multiplication or
  addition, but NOT for subtraction

* Such that $G$ has an identity element with respect to $\ast$ - an
  element $e$ such that applying it with $\ast$ does nothing, like
  multiplying by 1 or adding 0

* Such that elements of $G$ all have inverses with respect to $\ast$,
  meaning any element $g \in G$ has another element $g^{-1} \in G$
  such that combining them with $\ast$ gives the identity. For
  addition, the inverse of $x$ is $-x$. For multiplication, it's
  $\frac1{x}$.

Examples of groups include the real numbers $\mathbb{R}$ with addition
and the non-zero rational numbers $\mathbb{Q}^\ast$ with
multiplication.

There are also finite examples, like the set $\{+1, -1\}$ with
multiplication, and the set of permutations of $n$ elements with
composition - $S_n$, the symmetric group on $n$ elements.

## What is a representation?

A representation $\rho : G \to \text{GL}(V)$ of a group $G$ is a
homomorphism (a function respecting the group structure) from $G$ to
the group of linear automorphisms of a vector space $V$. You can
imagine "vector space" to mean $\mathbb{C}^n$ (the space of vectors
with $n$ entries in the complex numbers), since that's usually what
we're using.

"Linear automorphism" is a coordinate-independent way of saying
"invertible matrix". So you can rephrase the definition of a
representation as a map from a group $G$ to $n \times n$ matrices (the
$n$ is fixed for all elements), such that the group structure is
respected.

Sue me if you don't like it, but you'll never get your vector space
into a computer without picking a basis, meaning you are forced to
think of linear maps as matrices.

## What is a decomposition of a representation?

An irreducible representation is a representation that doesn't have
any subrepresentations. What does that mean? We could go through the
definitions, but for complex representations of finite groups,
irreducible means indecomposable: you can't write the representation
as a direct sum of other representations. That is the same thing as
saying you can't simultaneously block diagonalise all $\rho(g)$ for $g
\in G$ such that the blocks are smaller than the whole matrix.

Now that the basics are out of the way, we can ask an interesting
question:

## Why is this useful?

Irreducible representations come up in all sorts of places:

* Solving problems in quantum chemistry involving symmetries (oh,
  hello $S_n$)

* Solving completely computationally intractable optimization problems
  by spotting interesting symmetries ($S_n$ again?), then reducing them

* Turning groups into linear algebra, then studying them through how
  their representations behave (this is just a vague statement
  encompassing all of representation theory)

Anything with symmetries probably has something to do with groups and
thus, something to do with representations.

I keep mentioning $S_n$, but the people who paid attention in algebra
might note that it is kind of misleading to talk about it as if it is
special. In fact, all finite groups appear as a subgroup of some
symmetric group due to
[Cayley's theorem](https://en.wikipedia.org/wiki/Cayley%27s_theorem). The
proof is actually fairly simple (for such a deep-seeming result) and I
encourage you to look at it. So really, saying $S_n$ pops up
everywhere in group theory is the same thing as saying groups pop up
everywhere in group theory - not surprising.

Sadly, physicists are interested in infinite groups and in particular,
Lie groups. You might have heard of the Lorentz group $\text{O}(1,3)$,
a certain special unitary group $\text{SU}(2)$, or some other Lie
groups - these pop up all the time in physics.

I say "sadly" because my algorithms are not directly applicable to
those cases, since they're too continuous to fit inside my poor
computer. It's very happy that there is such strong motivation to
study Lie groups coming from physics, but my work isn't really
relevant there.

## How do you get a computer to compute a decomposition?

The first question to ask is: how do you do it by hand?

Let's say we have a finite group $G$ and a representation $\rho : G
\to \text{GL}(V)$ where $V$ is a finite dimensional complex vector
space (I'm skipping over something very important here, see if you can
guess what it is, or skip to the end for the answer).

If you _already_ have the complete list of irreducible representations
of $G$ (up to isomorphism), $(\rho_i)$, you can just combine all
$\rho_i$ as blocks in all possible ways that give matrices of the
correct size. That is, you try to find some $i_j$ such that:

$$\rho(g) = \begin{pmatrix}
\rho_{i_1}(g) & & \\\\
& \ddots & \\\\
& & \rho_{i_m}(g)
\end{pmatrix}$$

Where anything not labelled above is zero. This is easy for some small
groups. For example, if you have the representation of $\{\pm 1\}$
given by:

$$\pm 1 \mapsto \begin{pmatrix}
1 & 0 \\\\
0 & \pm 1
\end{pmatrix}$$

It's easy to spot this is given by two $1 \times 1$ blocks. But that's
because we already know all of the irreducible representations (let's
shorten that to irrep, as is standard), there are only two. Even
better, the matrices are already given in the nicest basis
possible. It's not really obvious how to "observe" the structure of a
representation like this in general.

Here's the dirty trick. Define $\chi_\rho(g) :=
\text{trace}(\rho(g))$. $\chi_\rho$ is called the character of
$\rho$. It has the useful property that for a finite $G$, a finite
dimensional representation (meaning the matrices are finite size), and
coefficients in $\mathbb{C}$: $\rho$ is isomorphic to $\tau$ (as
representations) if and only if their characters are the same.

This gives rise to this simple algorithm:

1. List all irreps of $G$ using e.g. Dixon's algorithm
   (`IrreducibleRepresentationsDixon` in GAP)

2. Put them together in all possible ways adding up to the correct
   matrix dimension.

3. Compute all of the characters - there will be exactly one matching
   the character of $\rho$, that's your decomposition!

This involves a lot of listing and trial and error - it is highly
inefficient and intractable for even small groups and small
representations.

Even ignoring efficiency, this tells you the decomposition *up to
isomorphism*. Telling you the basis change from $\rho$ to actually get
the smallest possible blocks is another problem. There's one algorithm
due to Serre to do that, with a full proof of correctness in his
textbook on Linear Representations of Finite Groups.

## How do you get a computer to compute it, quickly?

I (and my supervisor) came up with a bag of tricks to speed up the
computation by taking advantage of some extra information that's
usually available.

### Trading space for speed

$G$ is always isomorphic to a permutation group, but in the real
world, $G$ is usually actually _already_ a permutation group. This
lets us specialise our algorithms using the vast array of tools
available for computing with permutation groups. The main trick is
that a good
[base and strong generating set](https://en.wikipedia.org/wiki/Base_(group_theory))
can be computed with the
[Schreier-Sims algorithm](https://en.wikipedia.org/wiki/Schreier%E2%80%93Sims_algorithm),
letting us sum over $G$ quickly (computing $\sum_g f(g)$ for certain
$f$). This lets us compute some of Serre's formulas much, much faster.

Let's say we have two representations $\rho$ and $\tau$ as earlier,
which are isomorphic but we don't know the basis change (the
isomorphism). $G$ is a permutation group, so summing over it is
fast. But how can we reduce finding an isomorphism to a sum over $G$?
It's quite a neat trick.

We're looking for a matrix $A$ such that $A^{-1} \tau(g) A = \rho(g)$
for all $g \in G$. the "dumb" way would be to forget the
representation structure and just try to simultaneously diagonalise
the $\tau(g)$ and $\rho(g)$. I spent a whole six months hammering away
before noticing the smart way, which is to notice that there's another
representation $\alpha$ defined by:

$$g \mapsto (X \mapsto \tau(g)X\rho(g^{-1}))$$

If we say that $\rho$ and $\tau$ map to $\text{GL}(V)$ then this is a
representation $G \to \text{GL}(\text{GL}(V))$. Crazy! The funny thing
is that this representation is actually $\tau \otimes \rho^\ast$, the
Kronecker product of $\tau$ and the conjugate transpose of $\rho$
(tensor product and dual representation, if you're still
coordinate-free). The key thing here is that $A$ is a suitable basis
change matrix if and only if $\alpha(g)A = A$ for all $g \in G$.

Even better, the projection from $\text{GL}(V)$ to the subspace of the
suitable $A$ is given by the map (a specialisation of a more general
theorem from Serre):

$$p = \sum_g \alpha(g)$$

*This* is where the key speed gain is: we can use the summation trick
from before. The sacrifice is that the matrices $\alpha(g)$ are
absolutely huge. If $n$ is the dimension of $V$, this uses
$\text{O}(n^4)$ space. Terrible unless you come up with another trick
to cut down the space usage.

### Be lazy where possible to get the space back

We don't actually care what the matrices $\alpha(g)$ are, we just need
to know how to apply them to compute the image of the $p$ above, then
pick an invertible element from this image.

Even better, you can pick a random $B$ and $pB$ will almost always be
invertible in a rigorous sense: the determinant map $B \mapsto
\text{det}(pB)$ is polynomial, nonzero (we know $pB$ is invertible for
*some* $B$), so its zero set is measure zero. In practice, you can
just generate a random matrix and it will always work as a basis
change matrix.

So in the end we don't need to ever compute the matrices $\tau \otimes
\rho^\ast$ in full all at the same time, we can just compute:

$$A = pB = \sum_g (\tau(g) \otimes \rho^\ast(g))(B)$$

The tensor product has the neat property that $(A \otimes B)(C \otimes
D) = AC \otimes BD$. We can write the matrix $B$ as $\sum_{i,j} B_{ij}
e\_i \otimes e\_j$, that is we "vectorise" the matrix $B$ into an
$n^2$ length vector. Then the equation above simplifies to:

$$A = \sum_g \sum_{i,j} B_{ij} \tau(g)e_i \otimes \rho^\ast(g)e_j$$

So we have ended up with a formula for $A$ (the basis change matrix)
which doesn't require the computation of any matrices with $n^4$
entries, and is amenable to the fast group summing method vaguely
mentioned earlier.

Since we still have to do matrix multiplications many times, this
whole algorithm is still at least $\text{O}(n^3)$ but using clever
tensor product tricks, we got it down from $\text{O}(n^4)$. Not bad!

One thing to mention is that the independences of the run time from
$|G|$ frees us to compute with huge groups. My algorithm can operate
on small degree representations of groups like $S_{10}$, with $10!$
elements (3.6 million elements).

Hopefully this has been some kind of insight into what computational
representation theory might involve. Take a look at the
[source code](https://github.com/gap-packages/RepnDecomp) for the
gritty details.

# What did you gloss over?

There's one huge thing I haven't explained. Have you guessed what it
is?

You can't represent $\mathbb{C}$ in a computer! Floating point is
useless for algebra! So how were we multiplying matrices and adding up
coefficients in $\mathbb{C}$?  There's another trick: restrict your
attention to the
[cyclotomic numbers](https://en.wikipedia.org/wiki/Cyclotomic_field).

But why is that good enough? Is multiplication still constant time?
Was it ever constant time?

I'll answer these questions at some point.

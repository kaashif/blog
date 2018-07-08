Sharing /home between OpenBSD and Debian
2016-05-03

Some people reading this might be thinking: "hey, it's really easy to
do this, why is he writing an article on this?". You are partially
right, this should be really easy, but there are some weird things
that happened while I set this up that I feel should have been written
down somewhere, so my fear that I was completely borking my system
would have been assuaged.
<!-- more -->

Anyway, let's get onto the first thing I had to do:

##Resizing my Windows partition

Yes, I have Windows on my laptop. Yes, Stallman wouldn't be too
impressed. But come on, occasionally (very occasionally nowadays), I
have to run a program that only has a Windows version. In any case, I
don't have to justify myself to you, dear reader.

Doing it from inside Windows turned out to be less than easy, since I
was unable to shrink the C partition even after disabling and cleaning
out the pagefile, some system restore points and a load of other files
the Disk Cleanup Utility removed. I don't know what sort of hidden,
immutable system files were around stopping me, but I decided to just
boot into Debian, forcibly shrink it, and worry about it later.

I booted into Debian, started up Gparted and resized the partition
without issue. I then created a new partition (around 5GB in size, I
don't really keep too much stuff in my /home, this was really too
big).

##What filesystem should I use?

This is the next big issue: there is not that much overlap between the
filesystems supported by OpenBSD and Linux. Well, certainly every
filesystem OpenBSD supports is supported by Linux to some extent, but
there are a few problems with most of them.

###FAT32

Really, I can't use FAT32. It's 2016, how can I be caught storing my
files on FAT?

Seriously, though, it has no journaling, it's slow, it's really crap
and my data would all be corrupted and irretrievable after 5 mins.

On the other hand, it is very well supported by both OSes, but no,
let's move on.

###NTFS

This is the really perverted option. Using NTFS to store the home
partition of 2 different Unix-like OSes. This doesn't seem like too
bad of an option really, since it is a fairly modern filesystem
compared to FAT32.

But most programs are designed with Unix permissions and
ownership. This is possible to setup with NTFS, but it doesn't work
out of the box, which is what I'd prefer. Moving on.

###ReiserFS, XFS, BTRFS, ZFS, others

OpenBSD has very limited (read: none) support for any of these.

###Just using the filesystems I already use

Why should I go through all of this hassle when I could just format
the partition with ext4 or UFS and be done with it?

Well, rw support for UFS does exist under Linux, but very little
attention is given to the default ufs (the FreeBSD one, as far as I
know), let alone the ones where you have to add the option
ufstype=something (44bsd for OpenBSD's UFS). I really don't trust the
Linux support for UFS at all. Obviously there is no support for soft
updates or any of the SSD stuff OpenBSD has either.

ext4 support technically does partially exist under OpenBSD, since you
can mount an ext4 partition with all of the ext4-specific features
(journaling, some extended attributes, basically everything in
/etc/mke2fs.conf) disabled, but then you might as well be using ext2.

###The only choice remaining

Yes, that's right, I decided that plain ext2 was the best for my
needs.

Now all I had to do was format it and change my /etc/fstabs to point
to the right place. A piece of cake, right? Not quite, I ran into some
compatibility issues.

##Partition table woes
I had already formatted the disk as ext2 while I was booted into
Debian, so I expected to just have to copy over my files, change the
fstab and that would be that.

Since OpenBSD was my main OS, I had to boot into OpenBSD to copy that
home directory to the partition I had made (and formatted) in
Debian. The partition was /dev/sda7.

Now, for those of you who don't use BSD, you won't be familiar with
BSD disklabels. They come from a time when you weren't allowed more
than 4 partitions per disk due to the limitations of the MBR (this is
still true for a good number of computers, so I guess it isn't a
complete anachronism). On my laptop, I can have a lot more partitions
that that thanks to GPT: I can have up to 128 partitions, more than
anyone reasonable would need, so really, the disklabels just get in my
way, since they are basically just a way of cramming 16 partitions
into 1 MBR or GPT partition.

I had 7 GPT partitions already and 10 OpenBSD "partitions" in the
disklabel in OpenBSD's little area of my disk. This makes a total
of 17.

This shouldn't really be a problem, except due to the way OpenBSD's
partitioning system works, it relies entirely on the disklabel to
tell it where the partitions are on the disk, and since Debian
obviously has no reason to update the OpenBSD disklabel, there is no
record of the ext2 partition we can see.

But what does that have to do with there being 17 partitions? Well,
disklabels can only keep track of 16 partitions, so I have to get rid
of some of the partitions we already know about.

Anyway, the only place I could get the info to add the ext2 partition
was in the GPT, the very place the partitions are stored. OpenBSD
should really just look at this itself every so often:

	$ doas fdisk sd0
	
	Disk: sd0       Usable LBA: 34 to 117231374 [117231408 Sectors]
       #: type                                 [       start:         size ]
    ----------------------------------------------------------------------
       0: EFI Sys                              [        2048:       204800 ]
       1: e3c9e316-0b5c-4db8-817d-f92df00215ae [      206848:       262144 ]
       2: DOS FAT-12                           [      468992:     68757504 ]
       3: Win Recovery                         [    95830016:       921600 ]
       4: 4f68bce3-e8cd-4db1-96e7-fbcaf984b709 [    96753664:     20475904 ]
       5: OpenBSD                              [    77418496:     18411520 ]
	   6: OpenBSD                              [    69226496:      8192000 ]

Yeah, I gave the partition the type OpenBSD in the hopes that this
would help OpenBSD see it. No, that didn't work.

Anyway, the actual solution was really simple, I just had to edit the
disklabel and add a record of this partition:

```bash
$ doas disklabel -e sd0
```

At that moment, my disklabel looked something like this:

	# /dev/rsd0c:
    type: SCSI
    disk: SCSI disk
    label: LITEON IT L8T-64
    duid: 0000000000000000
    flags:
    bytes/sector: 512
    sectors/track: 63
    tracks/cylinder: 255
    sectors/cylinder: 16065
    cylinders: 7297
    total sectors: 117231408
    boundstart: 77418496
    boundend: 95830016
    drivedata: 0 
    
    16 partitions:
    #                size           offset  fstype [fsize bsize  cpg]
      a:           350400         77418496  4.2BSD   2048 16384    1 # /
      b:           536980         77768896    swap                   
      c:        117231408                0  unused                   
      d:           544256         78305888  4.2BSD   2048 16384    1 # /tmp
      e:           648896         78850144  4.2BSD   2048 16384    1 # /var
      f:          2029760         79499040  4.2BSD   2048 16384    1 # /usr
      g:          1160512         81528800  4.2BSD   2048 16384    1 # /usr/X11R6
      h:          4567424         82689312  4.2BSD   2048 16384    1 # /usr/local
      i:           204800             2048   MSDOS                   
      j:           262144           206848 unknown                   
      k:         68757504           468992   MSDOS                   
      l:           921600         95830016 unknown                   
      m:         20475904         96753664 unknown                   
      n:          2171776         87256736  4.2BSD   2048 16384    1 # /usr/src
      o:          2811648         89428512  4.2BSD   2048 16384    1 # /usr/obj
      p:          3589760         92240160  4.2BSD   2048 16384    1 # /home

I just made this up, but you get the idea, it's basically a complete
mess and I've exhausted the number of partitions I can have. I just
deleted partitions m and n, since I'll probably never need those on
separate partitions, and added a line with the fdisk info for my new
partition:

    m:            8192000         69226496 ext2fs

Then I added the line to my /etc/fstab:

	/dev/sd0m /home ext2fs rw,nodev,nosuid 1 2

And I deleted the old /home line, of course.

After copying the files, I rebooted and everything worked.

Or did it?

##panic: system on fire

After booting into OpenBSD and using it for a bit, it worked, so I
decided to boot into Debian and see if it worked there. It did.

But then I tried to boot back into OpenBSD and got a rather scary
error:

    ** /dev/rsd0m
	BAD SUPER BLOCK: VALUES IN SUPER BLOCK DISAGREE WITH THOSE IN FIRST ALTERNATE
	/dev/rsd0m: BLOCK SIZE DETERMINED TO BE ZERO

The system could boot, though, but all of my files had mysteriously
vanished. Running `fsck_ext2fs` gave the same error, leading me to
believe something was actually wrong.

I booted back into Debian and forced a fsck of the partition, but it
came out clean.

The moral of the story is that Linux and OpenBSD sometimes do things
differently, and in this case it is harmless, since the filesystem
works correctly from both OSes, it's just that the number of backup
superblocks Debian decided to keep was different than the number
OpenBSD was expecting. I could look up the right way and the exact
options to use to have perfect interop, but it works as is, so I can
worry about that later.

All I needed to do was disable the fscking of the partition at boot in
the /etc/fstab on OpenBSD, and both OSes booted up fine.

##Conclusion

Hopefully, someone at some point will benefit from this post, even if
it's only that someone looks at this and decides it's too much trouble.

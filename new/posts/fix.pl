use strict;
use warnings;

while (<>) {
	if (/Title:/) {
		my ($title) = /\s+Title: (.*)$/;
		print "$title\n";
	} elsif (/Date:/) {
		my ($date) = /\s+Date: (\d\d\d\d-\d\d-\d\d).*/;
		print "$date\n";
	} else {
		print;
	}
}

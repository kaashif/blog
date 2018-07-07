#!/usr/bin/perl
use strict;
use warnings;

use HTML::Escape qw(escape_html);

my $in_code_block = 0;
while (<>) {
  if (/```/) {
    if ($in_code_block) {
      print "</code></pre>\n";
    } else {
      print "<pre><code>";
    }
    $in_code_block = !$in_code_block;
  } else {
    if ($in_code_block) {
      print(escape_html($_));
    } else {
      print($_);
    }
  }
}

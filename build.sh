#!/bin/sh
rm -rf _out/

raco frog -b
cp -r css _out/
cp -r static _out/
raco frog -p


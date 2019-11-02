#!/bin/bash
#

case "$(uname -s)" in 
    MINGW*) {
        LIB_EXT=".dll";
        LIB_PREFIX="";
    };;
    Darwin*) {
        LIB_EXT=".dylib";
        LIB_PREFIX="lib";
    };;
    *) {
        LIB_EXT=".so";
        LIB_PREFIX="lib";
    }
esac


OUTDIR=target/debug

BASE_NAME=signal_generator
BINARY=$OUTDIR/$LIB_PREFIX$BASE_NAME$LIB_EXT
TARGET=$OUTDIR/$BASE_NAME.lv2/
INSTALLDIR=~/.lv2/

echo BINARY name on this platform is $BINARY

cargo build
mkdir -p $TARGET

sed -e "s/@LIB_EXT@/$LIB_EXT/g" -e "s/@LIB_PREFIX@/$LIB_PREFIX/g" manifest.ttl.in > $OUTDIR/manifest.ttl
cp $OUTDIR/manifest.ttl $BASE_NAME.ttl $BINARY $TARGET

echo "plugin created in" $TARGET

echo cp -f -r $TARGET $INSTALLDIR

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

BASE_NAME=ebur128_compressor
LIBNAME=$LIB_PREFIX$BASE_NAME$LIB_EXT
BINARY=$OUTDIR/$LIBNAME
TARGET=$OUTDIR/$BASE_NAME.lv2/
INSTALLDIR=~/.lv2/

echo BINARY name on this platform is $BINARY

mkdir -p $TARGET

sed -e "s/@LIBNAME@/$LIBNAME/g" manifest.ttl.in > $OUTDIR/manifest.ttl
cp $OUTDIR/manifest.ttl plugin_def.ttl $BINARY $TARGET

echo "plugin created in" $TARGET

cp -f -r $TARGET $INSTALLDIR
echo "plugin installed at $INSTALLDIR"
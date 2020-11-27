
#!/bin/bash

# build a setup.sh for your apps using linux bookmarks folder
# running vala app on gtk3 lib using valac --pkg gtk+-3.0 app.vala
# https://github.com/wildonion/stomegranate :::::: low-level ops
# https://stackoverflow.com/questions/16081614/vala-to-python-and-back-again 
# sudo pacman -S gobject-introspection
# sudo pacman -S clutter


echo "Cleaning..."

rm -rf tmp
rm -rf lib
rm -rf type
rm -f test

mkdir tmp
mkdir lib
mkdir type

    echo "Building Vala library..."

    # Note 1: Ubuntu package for valac: valac-0.14
    # Note 2: Generates broken gir if --gir= has a directory prefixed to it
    # Note 3: The -X switches for gcc are necessary!
    # Note 4: The generated gir will include GObject-2.0. That gir is
    #         included in Ubuntu package: libgirepository1.0-dev

    valac \
  --pkg clutter-1.0 \
    --library=Palelib \
    --directory=tmp \
    --gir=Palelib-1.0.gir \
    --output=libpalelib.so \
    -X -shared \
    -X -fPIC \
    redsquare.vala

    mv tmp/libpalelib.so lib
    mv tmp/Palelib-1.0.gir type

    # Note: We cannot generate C code and compile in the same call
    #       (We don't need the C code to run the test, but we are curious
    #       as to what Vala is generating. The resulting code will be in
    #       logging.c)
    #valac \
    #--ccode \
    #redsquare.vala


echo "Building typelib..."

# Note 1: Ubuntu package for g-ir-compiler: gobject-introspection
# Note 2: The --shared-library switch is really only necessary when using
#         the gir produced by valac, because it does not include the
#         'shared-library' attribute in <namespace> tag.


g-ir-compiler \
--shared-library=libpalelib.so \
--output=type/Palelib-1.0.typelib \
type/Palelib-1.0.gir

echo "Test Python..."

# Note 1: Ubuntu's default path for typelib files seems to be:
#         /usr/lib/girepository-1.0/.
# Note 2: It is also possible to programmatically change the
#         GI_TYPELIB_PATH environment var in Python (os.environ API).
#         If you do so, make sure to set it before importing from
#         gi.repository.
LD_LIBRARY_PATH=lib \
GI_TYPELIB_PATH=type \
chmod +x ./test.py
./test.py

# fix the error of : ImportError: cannot import name Palelib, introspection typelib not found
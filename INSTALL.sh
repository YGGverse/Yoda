#!/bin/bash

# Init environment
PHP_VERSION="8.3.9"

DIR_APP=$(dirname $(realpath -s "$0"))

DIR_BUILD="$DIR_APP/build/$(uname)"

DIR_PHP_SRC_SOURCE="$DIR_BUILD/tmp/php-src"
DIR_PHP_CPP_SOURCE="$DIR_BUILD/tmp/php-cpp"
DIR_PHP_GTK_SOURCE="$DIR_BUILD/tmp/php-gtk"

DIR_PHP_SRC_TARGET="$DIR_BUILD/opt/php"
DIR_PHP_CPP_TARGET="$DIR_BUILD/usr/local"
DIR_DESKTOP_TARGET="$DIR_BUILD/usr/local/share/applications"

# Check platform builder compatibility
if [ $(uname) != "Linux" ]; then
    echo "$(uname) auto-build not implemented" && exit
fi

# Ask for system dependencies installation
until [[ $INSTALL_SYSTEM_DEPENDENCIES =~ (y|n) ]]; do
    read -rp "Install system dependencies? [y/n]: " -e INSTALL_SYSTEM_DEPENDENCIES
done

## @TODO check for package manager support
if [[ $INSTALL_SYSTEM_DEPENDENCIES == "y" ]]; then
    sudo apt install libpq-dev\
                     bison\
                     libreadline-dev\
                     git\
                     composer\
                     build-essential\
                     autoconf\
                     automake\
                     libtool\
                     re2c\
                     libxml2-dev\
                     libcurl4-openssl-dev\
                     libssl-dev\
                     libbz2-dev\
                     libjpeg-dev\
                     libpng-dev\
                     libxpm-dev\
                     libfreetype6-dev\
                     libzip-dev\
                     libsqlite3-dev\
                     libonig-dev\
                     libxslt1-dev\
                     libgtk-3-dev\
                     libgladeui-dev\
                     libgtksourceview-3.0-dev\
                     libwnck-dev
fi

# Install composer dependencies
cd $DIR_APP

composer update

# Build PHP-SRC

## Get latest sources
if [ ! -d $DIR_PHP_SRC_SOURCE ]; then
    git clone -b PHP-$PHP_VERSION https://github.com/php/php-src.git $DIR_PHP_SRC_SOURCE
fi

## Check sources directory received to continue
if [ ! -d "$DIR_PHP_SRC_SOURCE" ]; then
    echo "Could not get PHP-$PHP_VERSION" && exit
fi

## Navigate to PHP-SRC location
cd $DIR_PHP_SRC_SOURCE

## Switch to version branch
git checkout "PHP-$PHP_VERSION"

## Get repository updates
git pull

## Build configuration for new installation
if [ ! -f "$DIR_PHP_SRC_SOURCE/configure" ]; then
    ./buildconf --force
fi

## Force build for new installation
if [ ! -d $DIR_PHP_SRC_TARGET ]; then BUILD_PHP_SRC="y"
else # or ask for re-build
    until [[ $BUILD_PHP_SRC =~ (y|n) ]]; do
        read -rp "Rebuild latest PHP-SRC? [y/n]: " -e BUILD_PHP_SRC
    done
fi

## Install PHP-SRC
if [[ $BUILD_PHP_SRC == "y" ]]; then
    ./configure --prefix=$DIR_PHP_SRC_TARGET\
                --with-openssl\
                --with-gettext\
                --with-pdo-mysql\
                --disable-cgi\
                --disable-shared\
                --enable-static\
                --enable-sockets\
                --enable-mbstring\
                --enable-pcntl

    make clean

    make -j $(nproc)

    make install
fi

# Build PHP-CPP

## Get latest sources
if [ ! -d $DIR_PHP_CPP_SOURCE ]; then
    git clone https://github.com/fast-debug/php-cpp.git $DIR_PHP_CPP_SOURCE
fi

## Check sources directory received to continue
if [ ! -d "$DIR_PHP_CPP_SOURCE" ]; then
    echo "Could not get PHP-CPP" && exit
fi

## Navigate to PHP-CPP location
cd $DIR_PHP_CPP_SOURCE

## Get repository updates
git pull

## Replace installation paths in PHP-CPP Makefile
sed -i "/PHP_CONFIG			=	/c\
         PHP_CONFIG			=	$DIR_PHP_SRC_TARGET/bin/php-config" $DIR_PHP_CPP_SOURCE/Makefile

sed -i "/INSTALL_PREFIX		=	/c\
         INSTALL_PREFIX		=	$DIR_PHP_CPP_TARGET" $DIR_PHP_CPP_SOURCE/Makefile

## Force build for new installation
if [ ! -d $DIR_PHP_CPP_TARGET ]; then BUILD_PHP_CPP="y"
else # or ask for re-build
    until [[ $BUILD_PHP_CPP =~ (y|n) ]]; do
        read -rp "Rebuild latest PHP-CPP? [y/n]: " -e BUILD_PHP_CPP
    done
fi

## Install PHP-CPP
if [[ $BUILD_PHP_CPP == "y" ]]; then
    make clean

    make -j $(nproc)

    make install
fi

# Build PHP-GTK

## Get latest sources
if [ ! -d $DIR_PHP_GTK_SOURCE ]; then
    git clone https://github.com/scorninpc/php-gtk3.git $DIR_PHP_GTK_SOURCE
fi

## Check sources directory received to continue
if [ ! -d "$DIR_PHP_GTK_SOURCE" ]; then
    echo "Could not get PHP-GTK" && exit
fi

## Navigate to PHP-GTK location
cd $DIR_PHP_GTK_SOURCE

## Get repository updates
git pull

## Replace installation paths in PHP-CPP Makefile
sed -i "/EXTENSION_DIR       =   /c\
         EXTENSION_DIR       =   $($DIR_PHP_SRC_TARGET/bin/php-config --extension-dir)" $DIR_PHP_GTK_SOURCE/Makefile

sed -i "/INI_DIR     =   /c\
         INI_DIR     =   /dev/null" $DIR_PHP_GTK_SOURCE/Makefile

## Force build for new installation
if [ ! -f "$($DIR_PHP_SRC_TARGET/bin/php-config --extension-dir)/php-gtk3.so" ]; then BUILD_PHP_GTK="y"
else # or ask for re-build
    until [[ $BUILD_PHP_GTK =~ (y|n) ]]; do
        read -rp "Rebuild latest PHP-GTK? [y/n]: " -e BUILD_PHP_GTK
    done
fi

## Install PHP-GTK
if [[ $BUILD_PHP_GTK == "y" ]]; then
    make clean

    make -j $(nproc)

    make install
fi

# Init desktop location
mkdir -p $DIR_DESKTOP_TARGET

# Create launcher
cat > "$DIR_DESKTOP_TARGET/yoda.sh" <<EOL
#!/bin/bash
$DIR_PHP_SRC_TARGET/bin/php -dextension=php-gtk3.so $DIR_APP/src/Yoda.php \$@
EOL

chmod +x "$DIR_DESKTOP_TARGET/yoda.sh"

# Create desktop menu
cat > "$DIR_DESKTOP_TARGET/yoda.desktop" <<EOL
[Desktop Entry]
    Name=Yoda
    Comment=PHP-GTK Browser for Gemini Protocol
    Type=Application
    Exec=$DIR_DESKTOP_TARGET/yoda.sh
EOL

chmod +x "$DIR_DESKTOP_TARGET/yoda.desktop"

## Ask for global menu setup
until [[ $SETUP_DESKTOP_MENU =~ (y|n) ]]; do
    read -rp "Setup desktop menu? [y/n]: " -e SETUP_DESKTOP_MENU
done

### Activate menu
if [[ $SETUP_DESKTOP_MENU == "y" ]]; then
    desktop-file-install --dir=$HOME/.local/share/applications $DIR_DESKTOP_TARGET/yoda.desktop
    update-desktop-database $HOME/.local/share/applications
fi

# Dump result
echo "Build completed!"

if [[ $SETUP_DESKTOP_MENU == "y" ]]; then
    echo "run Yoda from application menu or"
fi
    echo "start with launcher: \"$DIR_DESKTOP_TARGET/yoda.sh\""
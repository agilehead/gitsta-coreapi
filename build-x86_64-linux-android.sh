BASEDIR=$(dirname "$0")

export OPENSSL_LIB_DIR=`realpath "$BASEDIR"/../external-deps/openssl`
export OPENSSL_INCLUDE_DIR=`realpath "$BASEDIR"/../external-deps/openssl/include`

echo Compiling for x86_64-linux-android...
cargo ndk --target x86_64-linux-android --android-platform 28 -- build --release

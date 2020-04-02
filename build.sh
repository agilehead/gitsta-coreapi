BASEDIR=$(dirname "$0")

export OPENSSL_LIB_DIR=`realpath "$BASEDIR"/deps/openssl`
export OPENSSL_INCLUDE_DIR=`realpath "$BASEDIR"/deps/openssl/include`

echo Compiling for armv7-linux-androideabi...
cargo ndk --target armv7-linux-androideabi --android-platform 29 -- build --release

echo Compiling for aarch64-linux-android...
cargo ndk --target aarch64-linux-android --android-platform 28 -- build --release --verbose

echo Compiling for i686-linux-android...
cargo ndk --target i686-linux-android --android-platform 28 -- build --release

echo Compiling for x86_64-linux-android...
cargo ndk --target x86_64-linux-android --android-platform 28 -- build --release

echo Compiling for the host platform...
cargo build --release
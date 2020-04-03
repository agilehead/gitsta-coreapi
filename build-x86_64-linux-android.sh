BASEDIR=$(dirname "$2")

export OPENSSL_STATIC=1
export OPENSSL_LIB_DIR=`realpath "$BASEDIR"/../openssl_for_ios_and_android/output/openssl_111_android_build/x86_64/usr/local/lib`
export OPENSSL_INCLUDE_DIR=`realpath "$BASEDIR"/../openssl_for_ios_and_android/output/openssl_111_android_build/x86_64/usr/local/include`

# export OPENSSL_LIB_DIR=/home/jeswin/repos/jeswin/openssl_for_ios_and_android/output/openssl_111_android_build/x86_64/usr/local/lib
# export OPENSSL_INCLUDE_DIR=/home/jeswin/repos/jeswin/openssl_for_ios_and_android/output/openssl_111_android_build/x86_64/usr/local/include

# echo $OPENSSL_LIB_DIR
# echo $OPENSSL_INCLUDE_DIR

echo Compiling for x86_64-linux-android...
cargo ndk --target x86_64-linux-android --android-platform 28 -- build --release

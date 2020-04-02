export ANDROID_NDK_HOME="/home/"$USER"/Android/Sdk/ndk/21.0.6113669"
export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$ANDROID_NDK_ROOT/toolchains/arm-linux-androideabi-4.9/prebuilt/linux-x86_64/bin:$PATH

#./Configure android-arm64 -D__ANDROID_API__=29
#make
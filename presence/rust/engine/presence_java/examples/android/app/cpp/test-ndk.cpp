#include <jni.h>

extern "C" JNIEXPORT jlong JNICALL Java_com_goog_nearby_presence_1test_TestNdk_newNdk
    (JNIEnv *, jclass) {
  return 3;
}
find_package(Java REQUIRED)
include(UseJava)

enable_testing()

add_jar(PresenceEngine PresenceEngine.java)

get_target_property(_jarFile PresenceEngine JAR_FILE)
get_target_property(_classDir PresenceEngine CLASSDIR)

message(STATUS "Jar file ${_jarFile}")
message(STATUS "Class compiled to ${_classDir}")

add_test(NAME TestPresenceEngine
     COMMAND ${Java_JAVA_EXECUTABLE} -cp ${_jarFile} PresenceEngine)

add_custom_target(presence_java_example ALL
    COMMAND ${Java_JAVA_EXECUTABLE}
        -Djava.library.path=/usr/local/google/home/chenw/nearby_repo/dataclass/nearby/presence/rust/engine/target/debug
        -cp ${_jarFile} com.google.nearby.presence.engine.PresenceEngine)
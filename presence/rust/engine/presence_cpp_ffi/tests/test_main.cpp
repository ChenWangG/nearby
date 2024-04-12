// Copyright 2024 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// Provide a main() by Catch - only one main per cpp file
#define CATCH_CONFIG_MAIN
#include "catch2/catch.hpp"

#include "presence.h"

TEST_CASE( "Presence Engine", "DiscoveryEngineRequest Echo" ) {
    auto builder_ptr = presence_request_builder_new(10);
    presence_request_builder_add_condition(builder_ptr,
        1, PresenceIdentityType::Private, PresenceMeasurementAccuracy::CoarseAccuracy);
    auto request_ptr =  presence_request_builder_build(builder_ptr);
     presence_request_debug_print(request_ptr);
    REQUIRE(0 == 0);
}
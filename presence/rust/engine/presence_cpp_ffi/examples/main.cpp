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

#include <iostream>
#include "presence_ffi.h"
#include "cpp/presence_platform.h"

using namespace std;

void start_ble_scan(PresenceBleScanRequest request) {
   cout << "C System API: start BLE scan with Priority: " << request.priority << endl;
}

void presence_discovery_callback(int32_t input) {
    cout << "presence discovery callback input: " << input << endl;
}

int main(int argc, char **argv) {
   cout << "C main starts." << endl;

   PresencePlatform platform;
   platform.start_ble_scan = start_ble_scan;
   auto engine_ptr = presence_engine_new(&platform);

   auto builder_ptr = presence_request_builder_new(10 /* priority */);
   presence_request_builder_add_condition(builder_ptr,
       1 /* action */, PresenceIdentityType::Private, PresenceMeasurementAccuracy::CoarseAccuracy);
   auto request_ptr =  presence_request_builder_build(builder_ptr);
   presence_engine_start_discovery(engine_ptr, request_ptr, presence_discovery_callback);
}
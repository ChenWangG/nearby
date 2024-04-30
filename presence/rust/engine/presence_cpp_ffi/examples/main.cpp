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
#include <thread>
#include "presence_ffi.h"
#include "cpp/presence_platform.h"

using namespace std;

// BLE system API.
void start_ble_scan(PresenceBleScanRequest request, void (*platform_ble_scan_callback)(int)) {
   cout << "C System API: start BLE scan with Priority: " << request.priority << endl;
   // Echo back the priority.
   platform_ble_scan_callback(request.priority);
}

// Client callback to receive discovery results.
void presence_discovery_callback(int32_t priority) {
    cout << "C presence discovery callback with priority: " << priority << endl;
}

int main(int argc, char **argv) {
   cout << "C main starts." << endl;

   PresencePlatform platform;
   platform.start_ble_scan = start_ble_scan;
   auto engine = presence_engine_new(&platform, presence_discovery_callback);

   thread engine_thread { [=]() { presence_engine_run(engine); }};

   auto request_builder = presence_request_builder_new(111 /* priority */);
   presence_request_builder_add_condition(request_builder,
       1 /* action */, PresenceIdentityType::Private, PresenceMeasurementAccuracy::CoarseAccuracy);
   auto request =  presence_request_builder_build(request_builder);

   presence_engine_set_request(engine, request);

   engine_thread.join();
}
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

#include <condition_variable>
#include <mutex>
#include <thread>
#include "spdlog/spdlog.h"
extern "C" {
#include "presence.h"
}

using namespace std;

struct PresenceEngine* engine;

std::mutex callback_mutex;
std::condition_variable callback_notification;
PlatformBleScanCallback platform_callback;
PresenceBleScanRequest scan_request;


// BLE system API.
void start_ble_scan(PresenceBleScanRequest* request, PlatformBleScanCallback callback) {
    spdlog::info("Start BLE scan with Priority: {}",  request->priority);
    for (auto& action : request->actions) {
      spdlog::info("action: {}",  action);
    }
    {
        std::unique_lock<std::mutex> lock(callback_mutex);
        platform_callback = callback;
        scan_request = *request;
    }
    callback_notification.notify_all();
}

// Sends a BLE scan result in a separated thread.
 thread platform_thread { []() {
     while(true) {
        std::unique_lock<std::mutex> lock(callback_mutex);
        callback_notification.wait(lock);
        spdlog::info("Returns scan result.");
        presence_ble_scan_callback(engine, scan_request.priority);
        // platform_callback(scan_request.priority);
     }
 }};

// Client callback to receive discovery results.
void presence_discovery_callback(PresenceDiscoveryResult* result) {
    spdlog::info("Received discovery result with medium: {}, action: {}", (int)result->medium, result->device.actions[0]);
}

int main(int argc, char **argv) {
   PresencePlatform platform;
   platform.start_ble_scan = start_ble_scan;
   engine = presence_engine_new(&platform, presence_discovery_callback);

   thread engine_thread { [=]() { presence_engine_run(engine); }};

   auto request_builder = presence_request_builder_new(111 /* priority */);
   presence_request_builder_add_condition(request_builder,
       201 /* action */, PresenceIdentityType::Private, PresenceMeasurementAccuracy::CoarseAccuracy);
   presence_request_builder_add_condition(request_builder,
       202 /* action */, PresenceIdentityType::Private, PresenceMeasurementAccuracy::CoarseAccuracy);
   auto request =  presence_request_builder_build(request_builder);
   presence_engine_set_request(engine, request);

   engine_thread.join();
}
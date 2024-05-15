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

#include "presence.h"

// Provide a main() by Catch - only one main per cpp file
#define CATCH_CONFIG_MAIN
#include "catch2/catch.hpp"
#include "spdlog/spdlog.h"

#include <condition_variable>
#include <mutex>
#include <thread>

using namespace std;

const int EXPECTED_PRIORITY = 111;
const PresenceMedium EXPECTED_MEDIUM = PresenceMedium::BLE;
const int EXPECTED_ACTION_ZERO = 100;
const int EXPECTED_ACTION_ONE = 101;

struct PresenceEngine* engine;
PresenceBleScanRequest scan_request;
PresenceDiscoveryResult discovery_result;

std::mutex scan_mutex;
std::condition_variable scan_notification;

std::mutex discovery_mutex;
std::condition_variable discovery_notification;

// Function to hook the platform BLE system API to Presence Engine.
// For a simple end-to-end demo, The implementation below returns a pre-defined
// scan result directly.
// Fo real cases, The BLE system API should be called inside this function.
void start_ble_scan(PresenceBleScanRequest* request) {
  spdlog::info("Start BLE scan with Priority: {}", request->priority);
  {
    std::unique_lock<std::mutex> lock(scan_mutex);
    scan_request = *request;
  }
  // Trigger ble_scan_callback in a separated thread.
  scan_notification.notify_all();
}

// Return a pre-defined BLE scan result to Engine.
void ble_scan_callback() {
  spdlog::info("Returns scan result.");
  auto scan_result_builder =
      presence_ble_scan_result_builder_new(PresenceMedium::BLE);
  for (auto action: scan_request.actions) {
    presence_ble_scan_result_builder_add_action(scan_result_builder, action);
  }
  auto scan_result =
      presence_ble_scan_result_builder_build(scan_result_builder);
  presence_on_scan_result(engine, scan_result);
}

// Sends a BLE scan result in a separated thread.
thread platform_thread{[]() {
  while (true) {
    std::unique_lock<std::mutex> lock(scan_mutex);
    scan_notification.wait(lock);
    ble_scan_callback();
    break;
  }
}};

// Callback for the Engine to return discovery results.
void presence_discovery_callback(PresenceDiscoveryResult* result) {
  spdlog::info("Received discovery result.");
  {
    unique_lock<std::mutex> lock(discovery_mutex);
    discovery_result = *result;
  }
  discovery_notification.notify_all();
}

TEST_CASE("Presence Engine CPP", "Test discovery request/result E2E") {
  spdlog::info("Test Presence Engine C APIs End to End.");

  engine = presence_engine_new(presence_discovery_callback, start_ble_scan);

  thread engine_thread{[=]() { presence_engine_run(engine); }};

  auto request_builder = presence_request_builder_new(EXPECTED_PRIORITY);
  presence_request_builder_add_condition(request_builder,
                                         EXPECTED_ACTION_ZERO,
                                         PresenceIdentityType::Private,
                                         PresenceMeasurementAccuracy::CoarseAccuracy);
  presence_request_builder_add_condition(request_builder,
                                         EXPECTED_ACTION_ONE,
                                         PresenceIdentityType::Private,
                                         PresenceMeasurementAccuracy::CoarseAccuracy);
  auto request = presence_request_builder_build(request_builder);

  std::unique_lock<std::mutex> lock(discovery_mutex);
  presence_engine_set_discovery_request(engine, request);
  discovery_notification.wait(lock);

  presence_engine_stop(engine);
  engine_thread.join();
  platform_thread.join();

  REQUIRE(scan_request.priority == EXPECTED_PRIORITY);
  REQUIRE(scan_request.actions.size() == 2);
  REQUIRE(scan_request.actions[0] == EXPECTED_ACTION_ZERO);
  REQUIRE(scan_request.actions[1] == EXPECTED_ACTION_ONE);

  REQUIRE(discovery_result.medium == EXPECTED_MEDIUM);
  REQUIRE(discovery_result.device.actions.size() == 2);
  REQUIRE(discovery_result.device.actions[0] == EXPECTED_ACTION_ZERO);
  REQUIRE(discovery_result.device.actions[1] == EXPECTED_ACTION_ONE);
}
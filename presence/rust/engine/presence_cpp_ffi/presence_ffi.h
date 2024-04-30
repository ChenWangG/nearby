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

#ifndef presence_bindings_h
#define presence_bindings_h

/**
 * Warning, this file is autogenerated by cbindgen. Don't modify this manually.
 */

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>
#include "presence.h"


enum class PresenceIdentityType : uint32_t {
  Private = 0,
  Trusted,
  Public,
};

enum class PresenceMeasurementAccuracy : uint32_t {
  Unknown = 0,
  CoarseAccuracy,
  BestAvailable,
};

/// Struct to send a discovery request to the Engine.
struct PresenceDiscoveryRequest;

struct PresenceDiscoveryRequestBuilder;

struct PresenceEngine;

using PresenceDiscoveryCallback = void(*)(PresenceDiscoveryResult);


extern "C" {

PresenceEngine *presence_engine_new(void *platform,
                                    PresenceDiscoveryCallback presence_discovery_callback);

void presence_engine_run(PresenceEngine *engine);

void presence_engine_set_request(PresenceEngine *engine, PresenceDiscoveryRequest *request);

PresenceDiscoveryRequestBuilder *presence_request_builder_new(int32_t priority);

void presence_request_builder_add_condition(PresenceDiscoveryRequestBuilder *builder,
                                            uint32_t action,
                                            PresenceIdentityType identity_type,
                                            PresenceMeasurementAccuracy measurement_accuracy);

PresenceDiscoveryRequest *presence_request_builder_build(PresenceDiscoveryRequestBuilder *builder);

void presence_request_debug_print(const PresenceDiscoveryRequest *request);

} // extern "C"

#endif // presence_bindings_h

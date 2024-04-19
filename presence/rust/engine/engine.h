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

/**
 * Warning, this file is autogenerated by cbindgen. Don't modify this manually.
 */

#include <cstdarg>
#include <cstddef>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>


enum class IdentityType {
  Private = 0,
  Trusted,
  Public,
};

enum class MeasurementAccuracy {
  Unknown = 0,
  CoarseAccuracy,
  BestAvailable,
};

/// Struct to hold an action, identity type and their associated discovery condition.
struct DiscoveryCondition {
  uint32_t action;
  IdentityType identity_type;
  MeasurementAccuracy measurement_accuracy;
};

/// Struct to hold a list of DiscoveryCondition.
///
/// The `count` is the numer of items in the list.
struct DiscoveryConditionList {
  const DiscoveryCondition *items;
  size_t count;
};

/// Struct to send a discovery request to the Engine.
struct DiscoveryEngineRequest {
  int32_t priority;
  DiscoveryConditionList conditions;
};


extern "C" {

/// Echoes back a [DiscoveryEngineRequest](struct.DiscoveryEngineRequest.html).
/// The `*request_ptr` is cloned to the returned result.
/// Caller owns both the input and output and is responsible to free the memory by calling
/// [free_engine_request()](fn.free_engine_request.html)
const DiscoveryEngineRequest *echo_request(const DiscoveryEngineRequest *request_ptr);

/// Free the memory associated with the [`DiscoveryEngineRequest`](struct.DiscoveryEngineRequest.html).
void free_engine_request(const DiscoveryEngineRequest *request_ptr);

} // extern "C"
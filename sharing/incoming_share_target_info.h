// Copyright 2022 Google LLC
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

#ifndef THIRD_PARTY_NEARBY_SHARING_INCOMING_SHARE_TARGET_INFO_H_
#define THIRD_PARTY_NEARBY_SHARING_INCOMING_SHARE_TARGET_INFO_H_

#include "sharing/share_target_info.h"

namespace nearby {
namespace sharing {

class IncomingShareTargetInfo : public ShareTargetInfo {
 public:
  IncomingShareTargetInfo();
  IncomingShareTargetInfo(IncomingShareTargetInfo&&);
  IncomingShareTargetInfo& operator=(IncomingShareTargetInfo&&);
  ~IncomingShareTargetInfo() override;
};

}  // namespace sharing
}  // namespace nearby

#endif  // THIRD_PARTY_NEARBY_SHARING_INCOMING_SHARE_TARGET_INFO_H_

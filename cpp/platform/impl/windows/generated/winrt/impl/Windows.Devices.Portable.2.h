// Copyright 2020 Google LLC
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

// WARNING: Please don't edit this file. It was generated by C++/WinRT v2.0.210505.3

#ifndef WINRT_Windows_Devices_Portable_2_H
#define WINRT_Windows_Devices_Portable_2_H
#include "winrt/impl/Windows.Devices.Portable.1.h"
WINRT_EXPORT namespace winrt::Windows::Devices::Portable
{
    struct ServiceDevice
    {
        ServiceDevice() = delete;
        static auto GetDeviceSelector(winrt::Windows::Devices::Portable::ServiceDeviceType const& serviceType);
        static auto GetDeviceSelectorFromServiceId(winrt::guid const& serviceId);
    };
    struct StorageDevice
    {
        StorageDevice() = delete;
        static auto FromId(param::hstring const& deviceId);
        static auto GetDeviceSelector();
    };
}
#endif
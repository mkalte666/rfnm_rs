//
// Created by mkalte on 10/13/24.
//
#include "librfnm_wrap.hpp"

using namespace rfnm;

extern "C" {
  /// Discover all available usb rfnm devices
  /// If max_info_count is 0, or dst_infos nullptr, will return the total number of connected devices
  /// Otherwise it will return how many devices were actually copied into dst_infos
  size_t find_usb_devices(rfnm_dev_hwinfo* dst_infos, size_t max_info_count)
  {
    const auto infos = device::find(transport::TRANSPORT_USB);
    if (max_info_count==0 || dst_infos == nullptr) {
      return infos.size();
    }

    size_t to_copy = std::min(max_info_count,infos.size());
    for (size_t i = 0; i < to_copy; ++i) {
      dst_infos[i] = infos[i];
    }

    return to_copy;
  }
}
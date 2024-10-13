//
// Created by mkalte on 10/13/24.
//

#ifndef RFNM_RS_LIBRFNM_WRAP_H
#define RFNM_RS_LIBRFNM_WRAP_H

#include <cstddef>
#include <librfnm/constants.h>
#include <librfnm/device.h>

#ifdef __cplusplus
extern "C" {
#endif

size_t find_usb_devices(rfnm_dev_hwinfo* dst_infos, size_t max_info_count);

#ifdef __cplusplus
}
#endif

#endif // RFNM_RS_LIBRFNM_WRAP_H

//
// Created by mkalte on 10/13/24.
//

#ifndef RFNM_RS_LIBRFNM_WRAP_H
#define RFNM_RS_LIBRFNM_WRAP_H

#include <cstddef>
#include <librfnm/constants.h>
#include <librfnm/rfnm_fw_api.h>

#ifdef __cplusplus
extern "C" {
#endif

struct WrappedThrownError
{
  char message[256];
};

size_t find_usb_devices(rfnm_dev_hwinfo* dst_infos, size_t max_info_count);

struct DeviceWrapper;
DeviceWrapper* device_connect_usb(WrappedThrownError* err);
void device_free(DeviceWrapper* dev);
void device_get_hwinfo(DeviceWrapper* dev, rfnm_dev_hwinfo* dst);
rfnm_api_failcode device_get_rx_channel(DeviceWrapper* dev, uint32_t num, rfnm_api_rx_ch* dst);
rfnm_api_failcode device_set_stream_format(DeviceWrapper* dev, rfnm::stream_format format, size_t* bufsize);
uint32_t device_get_rx_channel_count(DeviceWrapper* dev);
uint32_t device_get_tx_channel_count(DeviceWrapper* dev);
rfnm_api_failcode device_rx_work_stop(DeviceWrapper* dev);
rfnm_api_failcode device_tx_work_stop(DeviceWrapper* dev);
rfnm_api_failcode device_set(DeviceWrapper* dev, uint16_t applies, bool confirm_execution, uint32_t timeout_us);
rfnm_api_failcode device_set_rx_channel_active(DeviceWrapper* dev, uint32_t channel, rfnm_ch_enable enable, rfnm_ch_stream stream, bool apply);
rfnm_api_failcode device_set_tx_channel_active(DeviceWrapper* dev, uint32_t channel, rfnm_ch_enable enable, rfnm_ch_stream stream, bool apply);
rfnm_api_failcode device_set_rx_channel_samp_freq_div(DeviceWrapper* dev, uint32_t channel, int16_t m, int16_t n, bool apply);
rfnm_api_failcode device_set_rx_channel_gain(DeviceWrapper* dev, uint32_t channel, int8_t gain, bool apply);
rfnm_api_failcode device_set_rx_channel_freq(DeviceWrapper* dev, uint32_t channel, int64_t freq, bool apply);
rfnm_api_failcode device_set_rx_channel_agc(DeviceWrapper* dev, uint32_t channel, rfnm_agc_type agc, bool apply);
rfnm_api_failcode device_set_rx_channel_fm_notch(DeviceWrapper* dev, uint32_t channel, rfnm_fm_notch fm_notch, bool apply);
rfnm_api_failcode device_set_rx_channel_bias_tee(DeviceWrapper* dev, uint32_t channel, rfnm_bias_tee bias_tee, bool apply);
rfnm_api_failcode device_set_rx_channel_path(DeviceWrapper* dev, uint32_t channel, rfnm_rf_path path, bool apply);

struct StreamWrapper;
StreamWrapper* stream_create(DeviceWrapper* dev, uint8_t ch_ids, WrappedThrownError* err);
void stream_free(StreamWrapper* stream);
rfnm_api_failcode stream_start(StreamWrapper* stream);
rfnm_api_failcode stream_stop(StreamWrapper* stream);
void stream_set_auto_dc_offset(StreamWrapper* stream, bool enabled, uint8_t ch_ids);
rfnm_api_failcode stream_read(StreamWrapper* stream, void* const* buffs, size_t elements_to_read, size_t& elements_read, uint64_t& timestamp_ns, uint32_t timeout_us);


#ifdef __cplusplus
}
#endif

#endif // RFNM_RS_LIBRFNM_WRAP_H

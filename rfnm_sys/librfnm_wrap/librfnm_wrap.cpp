//
// Created by mkalte on 10/13/24.
//
#include "librfnm_wrap.hpp"
#include <memory>
#include <stdexcept>
#include <cstring>


#include <librfnm/device.h>
#include <librfnm/rx_stream.h>

using namespace rfnm;

void clear_thrown_err_wrapper(WrappedThrownError* err) {
  for (auto &c : err->message) {
    c = 0;
  }
}

void err_msg(WrappedThrownError* err, const std::runtime_error& exception) {

  const size_t err_chars = strlen(exception.what());
  const size_t to_copy = std::min(err_chars,sizeof(err->message)-1);
  memcpy(err->message,exception.what(),to_copy);
  err->message[to_copy] = '\0';
}

extern "C" {

  struct DeviceWrapper {
    std::unique_ptr<device> dev;
  };

  struct StreamWrapper {
    std::unique_ptr<rx_stream> stream;
  };

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


 DeviceWrapper* device_connect_usb(WrappedThrownError* err)
 {
   clear_thrown_err_wrapper(err);
    try {
      auto new_device = std::make_unique<device>(transport::TRANSPORT_USB,"",DEBUG_NONE);
      DeviceWrapper* wrapper = new DeviceWrapper();
      wrapper->dev = std::move(new_device);
      return wrapper;
    } catch (const std::runtime_error& e) {
      err_msg(err,e);
      return nullptr;
    }
  }

  void device_free(DeviceWrapper* dev) {
    delete dev;
  }


void device_get_hwinfo(DeviceWrapper* dev, rfnm_dev_hwinfo* dst) {
      memcpy(dst,dev->dev->get_hwinfo(), sizeof(rfnm_dev_hwinfo));
}
rfnm_api_failcode device_get_rx_channel(DeviceWrapper* dev, uint32_t num, rfnm_api_rx_ch* dst)
{
  const auto info = dev->dev->get_rx_channel(num);
  if (info == nullptr) {
    return rfnm_api_failcode::RFNM_API_NOT_SUPPORTED;
  } else {
    memcpy(dst,info,sizeof(rfnm_api_rx_ch));
    return RFNM_API_OK;
  }
}

rfnm_api_failcode device_set_stream_format(DeviceWrapper* dev, stream_format format, size_t* bufsize)
  {
  return dev->dev->set_stream_format(format,bufsize);
}


rfnm_api_failcode device_rx_work_stop(DeviceWrapper* dev) {
  return dev->dev->rx_work_stop();
}
rfnm_api_failcode device_tx_work_stop(DeviceWrapper* dev) {
  return dev->dev->tx_work_stop();
}

rfnm_api_failcode device_set(DeviceWrapper* dev, uint16_t applies, bool confirm_execution, uint32_t timeout_us) {
  return dev->dev->set(applies,confirm_execution, timeout_us);
}

uint32_t device_get_rx_channel_count(DeviceWrapper* dev)
{
  return dev->dev->get_rx_channel_count();
}
uint32_t device_get_tx_channel_count(DeviceWrapper* dev) {
  return dev->dev->get_tx_channel_count();
}


rfnm_api_failcode device_set_rx_channel_active(DeviceWrapper* dev, uint32_t channel, rfnm_ch_enable enable, rfnm_ch_stream stream, bool apply)
{
  return dev->dev->set_rx_channel_active(channel,enable,stream,apply);
}
rfnm_api_failcode device_set_tx_channel_active(DeviceWrapper* dev, uint32_t channel, rfnm_ch_enable enable, rfnm_ch_stream stream, bool apply)
{
  return dev->dev->set_tx_channel_active(channel,enable,stream,apply);
}


rfnm_api_failcode device_set_rx_channel_freq(DeviceWrapper* dev, uint32_t channel, int64_t freq, bool apply) {
  return dev->dev->set_rx_channel_freq(channel,freq, apply);
}


rfnm_api_failcode device_set_rx_channel_samp_freq_div(DeviceWrapper* dev, uint32_t channel, int16_t m, int16_t n, bool apply)
{
  return dev->dev->set_rx_channel_samp_freq_div(channel,m,n,apply);
}

rfnm_api_failcode device_set_rx_channel_gain(DeviceWrapper* dev, uint32_t channel, int8_t gain, bool apply)
{
  return dev->dev->set_rx_channel_gain(channel,gain,apply);
}

  StreamWrapper* stream_create(DeviceWrapper* dev, uint8_t ch_ids, WrappedThrownError* err)
  {
    clear_thrown_err_wrapper(err);
    try {
      auto new_stream = std::make_unique<rx_stream>(*dev->dev, ch_ids);
      StreamWrapper* wrapper = new StreamWrapper();
      wrapper->stream = std::move(new_stream);
      return wrapper;
    } catch (const std::runtime_error& e) {
      err_msg(err,e);
      return nullptr;
    }
  }


void stream_free(StreamWrapper* stream) {
    delete stream;
  }


rfnm_api_failcode stream_start(StreamWrapper* stream){
  return stream->stream->start();
}
rfnm_api_failcode stream_stop(StreamWrapper* stream){
  return stream->stream->stop();
}
void stream_set_auto_dc_offset(StreamWrapper* stream, bool enabled, uint8_t ch_ids){
  stream->stream->set_auto_dc_offset(enabled,ch_ids);
}
rfnm_api_failcode stream_read(StreamWrapper* stream, void* const* buffs, size_t elements_to_read, size_t& elements_read, uint64_t& timestamp_ns, uint32_t timeout_us){
  return stream->stream->read(buffs, elements_to_read, elements_read, timestamp_ns, timeout_us);
}
}
#pragma once

#include <queue>
#include <condition_variable>
#include <mutex>
#include <string>
#include <thread>
#include <array>
#include <vector>

#include "constants.h"
#include "rfnm_fw_api.h"

#if defined(__GNUC__)
#define MSDLL
#elif defined(_MSC_VER)
#define MSDLL __declspec(dllexport)
#endif

#define RFNM_MHZ_TO_HZ(MHz) (MHz * 1000 * 1000ul)
#define RFNM_HZ_TO_MHZ(Hz) (Hz / (1000ul * 1000ul))
#define RFNM_HZ_TO_KHZ(Hz) (Hz / 1000ul)

namespace rfnm {
    struct transport_status {
        enum transport transport;
        int usb_boost_connected;
        int theoretical_mbps;
        enum stream_format rx_stream_format;
        enum stream_format tx_stream_format;
        int boost_pp_tx;
        int boost_pp_rx;
    };

    struct status {
        struct transport_status transport_status;

        struct rfnm_dev_hwinfo hwinfo;
        struct rfnm_dev_tx_ch_list tx;
        struct rfnm_dev_rx_ch_list rx;

        struct rfnm_dev_status dev_status;

        std::chrono::time_point<std::chrono::high_resolution_clock> last_dev_time;
    };

    struct rx_buf {
        uint8_t* buf;
        uint32_t phytimer;
        uint32_t adc_cc;
        uint64_t usb_cc;
        uint32_t adc_id;
    };

    struct tx_buf {
        uint8_t* buf;
        uint32_t phytimer;
        uint32_t dac_cc;
        uint64_t usb_cc;
        uint32_t dac_id;
    };

    class rx_buf_compare {
    public:
        bool operator()(struct rx_buf* lra, struct rx_buf* lrb) {
            return (lra->usb_cc) > (lrb->usb_cc);
        }
    };

    class rx_buf_s {
    public:
        rx_buf_s();

        std::queue<struct rx_buf*> in;
        std::priority_queue<struct rx_buf*, std::vector<struct rx_buf*>, rx_buf_compare> out[4];
        std::mutex in_mutex;
        std::mutex out_mutex;
        std::condition_variable cv;
        uint64_t usb_cc[4] = {};
        uint64_t qbuf_cnt = 0;

        uint64_t usb_cc_benchmark[4] = {};
        std::mutex benchmark_mutex;
        uint8_t last_benchmark_adc = 0;
    };

    class tx_buf_s {
    public:
        tx_buf_s();

        std::queue<struct tx_buf*> in;
        std::queue<struct tx_buf*> out;
        std::mutex in_mutex;
        std::mutex out_mutex;
        //std::mutex cc_mutex;
        std::condition_variable cv;
        uint64_t usb_cc = 0;
        uint64_t qbuf_cnt = 0;
    };

    class thread_data_s {
    public:
        thread_data_s();

        int ep_id = 0;
        int tx_active = 0;
        int rx_active = 0;
        int shutdown_req = 0;
        std::condition_variable cv;
        std::mutex cv_mutex;
    };

    struct _usb_handle;

    class rx_stream;

    class device {
    public:
        MSDLL explicit device(enum transport transport, std::string address = "", enum debug_level dbg = DEBUG_NONE);
        MSDLL ~device();

        MSDLL static std::vector<struct rfnm_dev_hwinfo> find(enum transport transport, std::string address = "", int bind = 0);

        MSDLL rfnm_api_failcode get(enum req_type type);

        MSDLL rfnm_api_failcode set(uint16_t applies, bool confirm_execution = true, uint32_t timeout_us = 1000000);

        // Getters
        MSDLL const struct rfnm_dev_hwinfo * get_hwinfo();
        MSDLL const struct rfnm_dev_status * get_dev_status();
        MSDLL const struct transport_status * get_transport_status();
        MSDLL const struct rfnm_api_rx_ch * get_rx_channel(uint32_t channel);
        MSDLL const struct rfnm_api_tx_ch * get_tx_channel(uint32_t channel);
        MSDLL uint32_t get_rx_channel_count();
        MSDLL uint32_t get_tx_channel_count();

        // General setters
        MSDLL rfnm_api_failcode set_stream_format(enum stream_format format, size_t *bufsize);

        // RX channel setters
        MSDLL rfnm_api_failcode set_rx_channel_samp_freq_div(uint32_t channel, int16_t m, int16_t n, bool apply = true);
        MSDLL rfnm_api_failcode set_rx_channel_freq(uint32_t channel, int64_t freq, bool apply = true);
        MSDLL rfnm_api_failcode set_rx_channel_rfic_lpf_bw(uint32_t channel, int16_t bw, bool apply = true);
        MSDLL rfnm_api_failcode set_rx_channel_gain(uint32_t channel, int8_t gain, bool apply = true);
        // not exposing setter for rfic_dc_i and rfic_dc_q because that functionality will need to change
        // use the stream class for ADC interleaving aware DC offset removal instead
        MSDLL rfnm_api_failcode set_rx_channel_agc(uint32_t channel, enum rfnm_agc_type agc, bool apply = true);
        MSDLL rfnm_api_failcode set_rx_channel_fm_notch(uint32_t channel, enum rfnm_fm_notch fm_notch, bool apply = true);
        MSDLL rfnm_api_failcode set_rx_channel_bias_tee(uint32_t channel, enum rfnm_bias_tee bias_tee, bool apply = true);
        MSDLL rfnm_api_failcode set_rx_channel_path(uint32_t channel, enum rfnm_rf_path path, bool apply = true);
        // not exposing setter for data_type because this library only handles complex samples for now

        // TX channel setters
        MSDLL rfnm_api_failcode set_tx_channel_samp_freq_div(uint32_t channel, int16_t m, int16_t n, bool apply = true);
        MSDLL rfnm_api_failcode set_tx_channel_freq(uint32_t channel, int64_t freq, bool apply = true);
        MSDLL rfnm_api_failcode set_tx_channel_rfic_lpf_bw(uint32_t channel, int16_t bw, bool apply = true);
        MSDLL rfnm_api_failcode set_tx_channel_power(uint32_t channel, int8_t power, bool apply = true);
        MSDLL rfnm_api_failcode set_tx_channel_bias_tee(uint32_t channel, enum rfnm_bias_tee bias_tee, bool apply = true);
        MSDLL rfnm_api_failcode set_tx_channel_path(uint32_t channel, enum rfnm_rf_path path, bool apply = true);
        // not exposing setter for data_type because this library only handles complex samples for now

        // High level stream API
        MSDLL rx_stream * rx_stream_create(uint8_t ch_ids);

        // Low level RX stream API
        MSDLL rfnm_api_failcode rx_work_start();
        MSDLL rfnm_api_failcode rx_work_stop();
        MSDLL rfnm_api_failcode rx_qbuf(struct rx_buf* buf, bool new_buffer = false);
        MSDLL rfnm_api_failcode rx_dqbuf(struct rx_buf** buf, uint8_t ch_ids = 0, uint32_t timeout_us = 20000);
        MSDLL rfnm_api_failcode rx_flush(uint32_t timeout_us = 20000, uint8_t ch_ids = 0xFF);
        MSDLL rfnm_api_failcode set_rx_channel_active(uint32_t channel, enum rfnm_ch_enable enable, enum rfnm_ch_stream stream, bool apply = true);

        // Low level TX stream API
        MSDLL rfnm_api_failcode tx_work_start(enum tx_latency_policy policy = TX_LATENCY_POLICY_DEFAULT);
        MSDLL rfnm_api_failcode tx_work_stop();
        MSDLL rfnm_api_failcode tx_qbuf(struct tx_buf* buf, uint32_t timeout_us = 20000);
        MSDLL rfnm_api_failcode tx_dqbuf(struct tx_buf** buf);
        MSDLL rfnm_api_failcode set_tx_channel_active(uint32_t channel, enum rfnm_ch_enable enable, enum rfnm_ch_stream stream, bool apply = true);

        // RF path (antenna) name conversion
        MSDLL static enum rfnm_rf_path string_to_rf_path(std::string path);
        MSDLL static std::string rf_path_to_string(enum rfnm_rf_path path);

    private:
        void threadfn(size_t thread_index);

        MSDLL bool unpack_12_to_cs16(uint8_t* dest, uint8_t* src, size_t sample_cnt);
        MSDLL bool unpack_12_to_cf32(uint8_t* dest, uint8_t* src, size_t sample_cnt);
        MSDLL bool unpack_12_to_cs8(uint8_t* dest, uint8_t* src, size_t sample_cnt);
        MSDLL void pack_cs16_to_12(uint8_t* dest, uint8_t* src8, int sample_cnt);

        MSDLL int single_ch_id_bitmap_to_adc_id(uint8_t ch_ids);
        MSDLL void dqbuf_overwrite_cc(uint8_t adc_id, int acquire_lock);
        MSDLL int dqbuf_is_cc_continuous(uint8_t adc_id, int acquire_lock);

        _usb_handle *usb_handle = nullptr;

        std::mutex s_dev_status_mutex;
        std::mutex s_transport_pp_mutex;

        struct status* s = nullptr;

        rx_buf_s rx_s;
        tx_buf_s tx_s;
        thread_data_s thread_data[THREAD_COUNT];

        std::array<std::thread, THREAD_COUNT> thread_c{};

        uint32_t cc_tx = 0;
        uint32_t cc_rx = 0;
        int last_dqbuf_ch = 0;

        int rx_stream_count = 0;
        bool rx_buffers_allocated = false;
        bool stream_format_locked = false;
    };
}

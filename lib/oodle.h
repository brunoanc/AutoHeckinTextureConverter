#ifndef OODLE_H
#define OODLE_H

#ifdef __cplusplus
extern "C" {
#endif

typedef struct __attribute__((__packed__))
{
    unsigned int unused_was_verbosity;
    int min_match_len;
    int seek_chunk_reset;
    int seek_chunk_len;
    int profile;
    int dictionary_size;
    int space_speed_tradeoff_bytes;
    int unused_was_max_huffmans_per_chunk;
    int send_quantum_crcs;
    int max_local_dictionary_size;
    int make_long_range_matcher;
    int match_table_size_log2;
    int jobify;
    void *jobify_user_ptr;
    int far_match_min_len;
    int far_match_offset_log2;
    unsigned int reserved[4];
} OodleLZ_CompressOptions;

int OodleLZ_Compress(int compressor, const void *raw_buf, int raw_len, void *comp_buf, int level,
                     const OodleLZ_CompressOptions *p_options, const void *dictionary_base, const void *lrm,
                     void *scratch_mem, int scratch_size);

#ifdef __cplusplus
}
#endif

#endif

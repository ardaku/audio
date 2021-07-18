#include <stdio.h>
#include <stdint.h>
#include "opusenc.h"

#define READ_SIZE 256

int main(int argc, char **argv) {
    FILE *fin;
    OggOpusEnc *enc;
    OggOpusComments *comments;
    int error;
    if (argc != 2) {
        fprintf(stderr, "usage: %s <raw pcm input> <Ogg Opus output>\n", argv[0]);
        return 1;
    }
    fin = fopen(argv[1], "rb");
    if (!fin) {
        fprintf(stderr, "cannot open input file: %s\n", argv[1]);
        return 1;
    }
    comments = ope_comments_create();
    ope_comments_add(comments, "ARTIST", "Someone");
    ope_comments_add(comments, "TITLE", "Some track");
    enc = ope_encoder_create_pull(
        comments,   // Stream Info
        48000,      // Sample Rate
        2,          // # of Channels
        0,          // 0 for mono/stereo, 1 for surround
        &error      // Error code.
    );
    if (!enc) {
        fprintf(stderr, "error encoding from file %s: %s\n", argv[1], ope_strerror(error));
        ope_comments_destroy(comments);
        fclose(fin);
        return 1;
    }

    // [out]
    unsigned char* page_data;
    unsigned int page_size;

    while (1) {
        short buf[2*READ_SIZE];
        int ret = fread(buf, 2*sizeof(short), READ_SIZE, fin);
        if (ret > 0) {
            ope_encoder_write(enc, buf, ret);
        } else {
            break;
        }
        int page_available = ope_encoder_get_page(
            enc,
            &page_data,
            &page_size,
            0
        );
        if(page_available) {
            printf("Found page! size:%d\n", page_size);
        }
    }
    int page_available = ope_encoder_get_page(
        enc,
        &page_data,
        &page_size,
        1
    );
    if(page_available) {
        printf("Found final page! size:%d\n", page_size);
    }

    ope_encoder_drain(enc);
    ope_encoder_destroy(enc);
    ope_comments_destroy(comments);
    fclose(fin);
    return 0;
}

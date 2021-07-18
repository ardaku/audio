gcc ogg_packer.c opusenc.c opus_header.c picture.c resample.c unicode_support.c -I../opus -DPACKAGE_VERSION=\"rust\" -DPACKAGE_NAME=\"caved\" -lm -lopus

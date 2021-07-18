gcc stream_opusfile.c -IC/libopusenc-0.2.1/include/ -IC/opus-1.3.1/include/ -Wl,-Bstatic -lopusenc -lopus -LC/libopusenc-0.2.1/.libs -LC/opus-1.3.1/.libs/ -static -lm -o stream_opusfile

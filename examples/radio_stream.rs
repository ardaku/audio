//! This example creates an internet radio stream over HTTP.

use actix_web::{HttpServer, App as ActixApp, HttpRequest, HttpResponse, web::{self, Bytes}};
use listenfd::ListenFd;
use futures::sync::mpsc;
use std::sync::{Arc, Mutex};
use wavy::{MicrophoneSystem, SampleRate};
use oped;

fn load_404<'a>(html: String) -> impl actix_web::Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body("404")
}

// All of the senders for sending audio data.
#[derive(Clone)]
struct Recorder {
    senders: Arc<Mutex<Vec<mpsc::UnboundedSender<Bytes>>>>,
}

impl Recorder {
    fn new() -> Self {
        Recorder {
            senders: Arc::new(Mutex::new(vec![])),
        }
    }
}

// This is the thread that records the audio and compresses it.
fn recording_thread(rec: Recorder) {
    let mut mic;
    let mut buffer = Vec::new();
    let mut stream_encoder = oped::StreamEncoder::new(/*
        48000,              // Sample rate (Hz)
        Channels::Stereo,   // Stereo
        Application::Audio, // High quality audio (for music, rather than voice)
    */);
    let mut trash = vec![];

    println!("Opening microphone system");
    mic = MicrophoneSystem::new(SampleRate::Normal).unwrap();
    println!("Opened microphone system");

    loop {
        buffer.clear();
        mic.record(&mut |_index, l, r| {
            buffer.push((l, r));
        });

        let data = if let Some(data) = stream_encoder.encode(&buffer) {
            data
        } else {
            continue;
        };

        // Send audio to each listener.
        let mut sends = rec.senders.lock().unwrap();
        for send in 0..sends.len() {
            let mut bytes = Bytes::new();
            bytes.extend_from_slice(data);
            if (*sends)[send].unbounded_send(bytes).is_err() {
                trash.push(send);
            }
        }

        // Remove senders for people who have stopped listening.
        while let Some(trash) = trash.pop() {
            (*sends).remove(trash);
        }

/*        speaker.play(&mut || {
            if let Some((lsample, rsample)) = buffer.pop_front() {
                AudioSample::stereo(lsample, rsample)
            } else {
                AudioSample::stereo(0, 0)
            }
        });*/
    }
}

fn main() {
    let mut ip_port = "0.0.0.0:8080";
    let mut recorder = Recorder::new();

    println!("Starting radio stream on {}…", ip_port);

    let rec = recorder.clone();
    std::thread::spawn(move || recording_thread(rec));

    let mut listenfd = ListenFd::from_env();
    let senders = recorder.senders.clone();
    let mut server = HttpServer::new(move || {
        let senders = senders.clone();
        ActixApp::new()
            .service(web::resource("/listen").route(
                web::get().to(move |_req: HttpRequest| {
                    HttpResponse::Ok()
                        .content_type("application/ogg")
                        .streaming({
                            let (send, recv) = mpsc::unbounded();
                            let mut sends = senders.lock().unwrap();

                            sends.push(send);
                            recv
                        })
                }
            )))
            .default_service(
                web::get().to( load_404 )
            )
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(ip_port).unwrap()
    };

    server.run().unwrap();
}

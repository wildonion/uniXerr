




/*


                                RTP TASKS

    an event streaming platform like VoD and FoD protocol like arvancloud based 
    for streaming over video (Vod) chunks using ffmpeg and gstreamer in rust 
    based on hls which is an adaptive bitrate method
         
         - convert uploaded videos to hls for multi bitrate conversion
         - create an endpoint api to point where the hls is
         - call the api to load in html5 video tag
         - a file on demand protocol to upload files and return their id


*/



pub mod ws;
pub mod wrtc;
pub mod socks;
pub mod grpc;
pub mod mq;
pub mod p2p;
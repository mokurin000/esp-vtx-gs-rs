gst-launch-1.0 udpsrc port=5600  ! application/x-rtp, payload=26 ! rtpjpegdepay ! jpegdec ! videoconvert ! autovideosink
gst-launch-1.0 udpsrc port=5600  ! application/x-rtp, payload=26 ! rtpjpegdepay ! jpegdec ! \
videoscale ! videoconvert  ! videorate ! video/x-raw,framerate=30/1 ! tee name=t \
t. ! queue ! x264enc tune=zerolatency  !  qtmux ! filesink location=video.mp4 -e \
t. ! queue ! autovideosink  sync=false

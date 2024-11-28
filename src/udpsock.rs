use std::net::UdpSocket;

fn socket_create (src_bind: String) -> Result<UdpSocket, String> {
    let socket = UdpSocket::bind(&src_bind).map_err(|e| {
        format!("UDP Socket binding Error on {}: {}", src_bind, e)
    })?;

    socket.set_nonblocking(false).map_err(|e| {
        format!("Failed to set non-blocking mode: {}", e)
    })?;

    Ok(socket)
}
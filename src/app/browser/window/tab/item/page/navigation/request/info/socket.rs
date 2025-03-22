use gtk::gio::SocketAddress;

pub struct Socket {
    pub local_address: SocketAddress,
    pub remote_address: SocketAddress,
}

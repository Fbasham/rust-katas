use preloaded::{TcpEvent, TcpState};

pub fn traverse_tcp_states(events: &[TcpEvent]) -> TcpState {
    let mut state = TcpState::CLOSED;
    for s in events {
        state = match state {
            TcpState::CLOSED => match s {
                TcpEvent::APP_PASSIVE_OPEN => TcpState::LISTEN,
                TcpEvent::APP_ACTIVE_OPEN => TcpState::SYN_SENT,
                _ => TcpState::ERROR,
            },
            TcpState::LISTEN => match s {
                TcpEvent::RCV_SYN => TcpState::SYN_RCVD,
                TcpEvent::APP_SEND => TcpState::SYN_SENT,
                TcpEvent::APP_CLOSE => TcpState::CLOSED,
                _ => TcpState::ERROR,
            },
            TcpState::SYN_RCVD => match s {
                TcpEvent::APP_CLOSE => TcpState::FIN_WAIT_1,
                TcpEvent::RCV_ACK => TcpState::ESTABLISHED,
                _ => TcpState::ERROR,
            },
            TcpState::SYN_SENT => match s {
                TcpEvent::RCV_SYN => TcpState::SYN_RCVD,
                TcpEvent::RCV_SYN_ACK => TcpState::ESTABLISHED,
                TcpEvent::APP_CLOSE => TcpState::CLOSED,
                _ => TcpState::ERROR,
            },
            TcpState::ESTABLISHED => match s {
                TcpEvent::APP_CLOSE => TcpState::FIN_WAIT_1,
                TcpEvent::RCV_FIN => TcpState::CLOSE_WAIT,
                _ => TcpState::ERROR,
            },
            TcpState::FIN_WAIT_1 => match s {
                TcpEvent::RCV_FIN => TcpState::CLOSING,
                TcpEvent::RCV_FIN_ACK => TcpState::TIME_WAIT,
                TcpEvent::RCV_ACK => TcpState::FIN_WAIT_2,
                _ => TcpState::ERROR,
            },
            TcpState::CLOSING => match s {
                TcpEvent::RCV_ACK => TcpState::TIME_WAIT,
                _ => TcpState::ERROR,
            },
            TcpState::FIN_WAIT_2 => match s {
                TcpEvent::RCV_FIN => TcpState::TIME_WAIT,
                _ => TcpState::ERROR,
            },
            TcpState::TIME_WAIT => match s {
                TcpEvent::APP_TIMEOUT => TcpState::CLOSED,
                _ => TcpState::ERROR,
            },
            TcpState::CLOSE_WAIT => match s {
                TcpEvent::APP_CLOSE => TcpState::LAST_ACK,
                _ => TcpState::ERROR,
            },
            TcpState::LAST_ACK => match s {
                TcpEvent::RCV_ACK => TcpState::CLOSED,
                _ => TcpState::ERROR,
            },
            _ => TcpState::ERROR,
        }
    }
    state
}

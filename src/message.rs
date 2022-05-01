/// A single reply unit, as mentioned in
/// [Bird's Documentation](https://gitlab.nic.cz/labs/bird/-/blob/master/doc/reply_codes)
#[derive(Debug, Clone)]
pub enum Message {
    // 0xxx series
    Ok,
    Welcome(String),
    ReadingConfiguration(String),
    Reconfigured(String),
    ReconfigurationInProgress(String),
    ReconfigurationQueued(String),
    ReconfigurationIgnoredShutdown(String),
    ShutdownOrdered(String),
    AlreadyDisable(String),
    Disabled(String),
    AlreadyEnabled(String),
    Enabled(String),
    Restarted(String),
    StatusReport(String),
    RouteCount(String),
    Reloading(String),
    AccessRestricted(String),
    ReconfigurationUnQueued(String),
    ReconfigurationConfirmed(String),
    NothingToDo(String),
    ConfigurationOk(String),
    UndoRequested(String),
    UndoScheduled(String),
    EvaluatingExpression(String),
    GracefulRestartStatus(String),
    GracefulRestartOrdered(String),
    // 1xxx series - table entries
    BirdVersion(String),
    InterfaceList(String),
    ProtocolList(String),
    InterfaceAddress(String),
    InterfaceFlags(String),
    InterfaceSummary(String),
    ProtocolDetails(String),
    RouteList(String),
    RouteDetails(String),
    StaticRouteList(String),
    SymbolList(String),
    Uptime(String),
    RouteExtendedAttributeList(String),
    OspfNeighbors(String),
    Ospf(String),
    OspfInterface(String),
    OspfState(String),
    OspfLsadb(String),
    Memory(String),
    RoaList(String),
    BfdSessions(String),
    RipInterfaces(String),
    RipNeighbors(String),
    BabelInterfaces(String),
    BabelNeighbors(String),
    BabelEntries(String),
    // 2xxx series - table headers
    ProtocolListHeader(String),
    InterfaceSummaryHeader(String),
    // we bundle up all 2xxx series headers here
    TableHeader(u32, String),
    // 8xxx series errors - server side errors
    ReplyTooLong(String),
    RouteNotFound(String),
    ConfigurationFileError(String),
    NoProtocolsMatch(String),
    StoppedDueToReconfiguration(String),
    ProtocolDown(String),
    ReloadFailed(String),
    AccessDenied(String),
    // we bundle up all 8xxx series errors into this
    RuntimeError(u32, String),
    // 9xxx series errors - client side errors
    CommandTooLong(String),
    ParseError(String),
    InvalidSymbol(String),
    // we bundle up all 9xxx series errors into this
    ClientError(u32, String),
    // fallback
    Unknown(u32, String),
}

impl Message {
    pub fn from_code(code: u32, message: &str) -> Message {
        let msg = message.to_owned();

        match code {
            // 0xxx series
            0 => Self::Ok,
            1 => Self::Welcome(msg),
            2 => Self::ReadingConfiguration(msg),
            3 => Self::Reconfigured(msg),
            4 => Self::ReconfigurationInProgress(msg),
            5 => Self::ReconfigurationQueued(msg),
            6 => Self::ReconfigurationIgnoredShutdown(msg),
            7 => Self::ShutdownOrdered(msg),
            8 => Self::AlreadyDisable(msg),
            9 => Self::Disabled(msg),
            10 => Self::AlreadyEnabled(msg),
            11 => Self::Enabled(msg),
            12 => Self::Restarted(msg),
            13 => Self::StatusReport(msg),
            14 => Self::RouteCount(msg),
            15 => Self::Reloading(msg),
            16 => Self::AccessRestricted(msg),
            17 => Self::ReconfigurationUnQueued(msg),
            18 => Self::ReconfigurationConfirmed(msg),
            19 => Self::NothingToDo(msg),
            20 => Self::ConfigurationOk(msg),
            21 => Self::UndoRequested(msg),
            22 => Self::UndoScheduled(msg),
            23 => Self::EvaluatingExpression(msg),
            24 => Self::GracefulRestartStatus(msg),
            25 => Self::GracefulRestartOrdered(msg),
            // 1xxx series
            1000 => Self::BirdVersion(msg),
            1001 => Self::InterfaceList(msg),
            1002 => Self::ProtocolList(msg),
            1003 => Self::InterfaceAddress(msg),
            1004 => Self::InterfaceFlags(msg),
            1005 => Self::InterfaceSummary(msg),
            1006 => Self::ProtocolDetails(msg),
            1007 => Self::RouteList(msg),
            1008 => Self::RouteDetails(msg),
            1009 => Self::StaticRouteList(msg),
            1010 => Self::SymbolList(msg),
            1011 => Self::Uptime(msg),
            1012 => Self::RouteExtendedAttributeList(msg),
            1013 => Self::OspfNeighbors(msg),
            1014 => Self::Ospf(msg),
            1015 => Self::OspfInterface(msg),
            1016 => Self::OspfState(msg),
            1017 => Self::OspfLsadb(msg),
            1018 => Self::Memory(msg),
            1019 => Self::RoaList(msg),
            1020 => Self::BfdSessions(msg),
            1021 => Self::RipInterfaces(msg),
            1022 => Self::RipNeighbors(msg),
            1023 => Self::BabelInterfaces(msg),
            1024 => Self::BabelNeighbors(msg),
            1025 => Self::BabelEntries(msg),
            2002 => Self::ProtocolListHeader(msg),
            2005 => Self::InterfaceSummaryHeader(msg),
            2000..=2999 => Self::TableHeader(code, msg),
            8000 => Self::ReplyTooLong(msg),
            8001 => Self::RouteNotFound(msg),
            8002 => Self::ConfigurationFileError(msg),
            8003 => Self::NoProtocolsMatch(msg),
            8004 => Self::StoppedDueToReconfiguration(msg),
            8005 => Self::ProtocolDown(msg),
            8006 => Self::ReloadFailed(msg),
            8007 => Self::AccessDenied(msg),
            8008..=8999 => Self::RuntimeError(code, msg),
            9000 => Self::CommandTooLong(msg),
            9001 => Self::ParseError(msg),
            9002 => Self::InvalidSymbol(msg),
            9003..=9999 => Self::ClientError(code, msg),
            _ => Self::Unknown(code, msg),
        }
    }

    pub fn code(&self) -> u32 {
        match self {
            Message::Ok => 0,
            Message::Welcome(_) => 1,
            Message::ReadingConfiguration(_) => 2,
            Message::Reconfigured(_) => 3,
            Message::ReconfigurationInProgress(_) => 4,
            Message::ReconfigurationQueued(_) => 5,
            Message::ReconfigurationIgnoredShutdown(_) => 6,
            Message::ShutdownOrdered(_) => 7,
            Message::AlreadyDisable(_) => 8,
            Message::Disabled(_) => 9,
            Message::AlreadyEnabled(_) => 10,
            Message::Enabled(_) => 11,
            Message::Restarted(_) => 12,
            Message::StatusReport(_) => 13,
            Message::RouteCount(_) => 14,
            Message::Reloading(_) => 15,
            Message::AccessRestricted(_) => 16,
            Message::ReconfigurationUnQueued(_) => 17,
            Message::ReconfigurationConfirmed(_) => 18,
            Message::NothingToDo(_) => 19,
            Message::ConfigurationOk(_) => 20,
            Message::UndoRequested(_) => 21,
            Message::UndoScheduled(_) => 22,
            Message::EvaluatingExpression(_) => 23,
            Message::GracefulRestartStatus(_) => 24,
            Message::GracefulRestartOrdered(_) => 25,
            Message::BirdVersion(_) => 1000,
            Message::InterfaceList(_) => 1001,
            Message::ProtocolList(_) => 1002,
            Message::InterfaceAddress(_) => 1003,
            Message::InterfaceFlags(_) => 1004,
            Message::InterfaceSummary(_) => 1005,
            Message::ProtocolDetails(_) => 1006,
            Message::RouteList(_) => 1007,
            Message::RouteDetails(_) => 1008,
            Message::StaticRouteList(_) => 1009,
            Message::SymbolList(_) => 1010,
            Message::Uptime(_) => 1011,
            Message::RouteExtendedAttributeList(_) => 1012,
            Message::OspfNeighbors(_) => 1013,
            Message::Ospf(_) => 1014,
            Message::OspfInterface(_) => 1015,
            Message::OspfState(_) => 1016,
            Message::OspfLsadb(_) => 1017,
            Message::Memory(_) => 1018,
            Message::RoaList(_) => 1019,
            Message::BfdSessions(_) => 1020,
            Message::RipInterfaces(_) => 1021,
            Message::RipNeighbors(_) => 1022,
            Message::BabelInterfaces(_) => 1023,
            Message::BabelNeighbors(_) => 1024,
            Message::BabelEntries(_) => 1025,
            Message::ProtocolListHeader(_) => 2002,
            Message::InterfaceSummaryHeader(_) => 2005,
            Message::TableHeader(c, _) => *c,
            Message::ReplyTooLong(_) => 8000,
            Message::RouteNotFound(_) => 8001,
            Message::ConfigurationFileError(_) => 8000,
            Message::NoProtocolsMatch(_) => 8003,
            Message::StoppedDueToReconfiguration(_) => 8004,
            Message::ProtocolDown(_) => 8005,
            Message::ReloadFailed(_) => 8006,
            Message::AccessDenied(_) => 8007,
            Message::RuntimeError(c, _) => *c,
            Message::CommandTooLong(_) => 9000,
            Message::ParseError(_) => 9001,
            Message::InvalidSymbol(_) => 9002,
            Message::ClientError(c, _) => *c,
            Message::Unknown(c, _) => *c,
        }
    }
}

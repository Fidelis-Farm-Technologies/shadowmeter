pub struct Record {
    pub sid: String,
    pub stime: i64,
    pub ltime: i64,
    pub proto: i64,
    pub saddr: String,
    pub sport: i64,
    pub daddr: String,
    pub dport: i64,
    pub sutcp: String,
    pub dutcp: String,
    pub sitcp: String,
    pub ditcp: String,
    pub spd: String,
    pub vlan: i64,
    pub sdata: i64,
    pub ddata: i64,
    pub sbytes: i64,
    pub dbytes: i64,
    pub spkts: i64,
    pub dpkts: i64,
    pub sentropy: i64,
    pub dentropy: i64,
    pub siat: i64,
    pub diat: i64,
    pub reason: String,
    pub applabel: String
}


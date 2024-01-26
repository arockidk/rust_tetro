pub type PieceOffset = (int,int);
struct KickData {
    NE: Vec<PieceOffset>,
    ES: Vec<PieceOffset>,
    SW: Vec<PieceOffset>,
    WN: Vec<PieceOffset>,
    NW: Vec<PieceOffset>,
    WS: Vec<PieceOffset>,
    SE: Vec<PieceOffset>,
    EN: Vec<PieceOffset>,
    NS: Vec<PieceOffset>,
    EW: Vec<PieceOffset>,
    SN: Vec<PieceOffset>,
    WE: Vec<PieceOffset>
}
impl KickData {
    pub fn blank() -> KickData {
        KickData { 
            NE: Vec::new(),
            ES: Vec::new(),
            SW: Vec::new(),
            WN: Vec::new(),
            NW: Vec::new(),
            WS: Vec::new(),
            SE: Vec::new(),
            EN: Vec::new(),
            NS: Vec::new(),
            EW: Vec::new(),
            SN: Vec::new(),
            WE: Vec::new()
        }
    }
}

pub static L: KickData;
pub static J: KickData;
pub static T: KickData;
pub static O: KickData;
pub static S: KickData;
pub static Z: KickData;
pub static I: KickData;
pub fn init() {
    L = KickData::blank();
    J = KickData::blank();
    T = KickData::blank();
    O = KickData::blank();
    S = KickData::blank();
    Z = KickData::blank();
    I = KickData::blank();
    L.NE = [( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)].to_vec();
    L.ES = [( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)].to_vec();
    L.SW = [( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1, 2)].to_vec();
    L.WN = [( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)].to_vec();

    L.NW = [( 0, 0),( 1, 0),( 1, 1),( 0,-2),( 1,-2)].to_vec();
    L.WS = [( 0, 0),(-1, 0),(-1,-1),( 0, 2),(-1, 2)].to_vec();
    L.SE = [( 0, 0),(-1, 0),(-1, 1),( 0,-2),(-1,-2)].to_vec();
    L.EN = [( 0, 0),( 1, 0),( 1,-1),( 0, 2),( 1, 2)].to_vec();

    L.NS=[( 0, 0),( 0, 1)].to_vec();
    L.EW=[( 0, 0),( 1, 0)].to_vec();
    L.SN=[( 0, 0),( 0,-1)].to_vec();
    L.WE=[( 0, 0),(-1, 0)].to_vec();

    // J

    J.NE = L.NE.clone();
    J.ES = L.ES.clone();
    J.SW = L.SW.clone();
    J.WN = L.WN.clone();
    
    J.NW = L.NW.clone();
    J.WS = L.WS.clone();
    J.SE = L.SE.clone();
    J.EN = L.EN.clone();

    J.NS = L.NS.clone();
    J.EW = L.EW.clone();
    J.SN = L.SN.clone();
    J.SW = L.SW.clone();
    
    // T
    T.NE = L.NE.clone();
    T.ES = L.ES.clone();
    T.SW = L.SW.clone();
    T.WN = L.WN.clone();
    
    T.NW = L.NW.clone();
    T.WS = L.WS.clone();
    T.SE = L.SE.clone();
    T.EN = L.EN.clone();

    T.NS = L.NS.clone();
    T.EW = L.EW.clone();
    T.SN = L.SN.clone();
    T.SW = L.SW.clone();


    // S

    S.NE = L.NE.clone();
    S.ES = L.ES.clone();
    S.SW = L.SW.clone();
    S.WN = L.WN.clone();
    
    S.NW = L.NW.clone();
    S.WS = L.WS.clone();
    S.SE = L.SE.clone();
    S.EN = L.EN.clone();

    S.NS = L.NS.clone();
    S.EW = L.EW.clone();
    S.SN = L.SN.clone();
    S.SW = L.SW.clone();

    // Z

    Z.NE = L.NE.clone();
    Z.ES = L.ES.clone();
    Z.SW = L.SW.clone();
    Z.WN = L.WN.clone();
    
    Z.NW = L.NW.clone();
    Z.WS = L.WS.clone();
    Z.SE = L.SE.clone();
    Z.EN = L.EN.clone();

    Z.NS = L.NS.clone();
    Z.EW = L.EW.clone();
    Z.SN = L.SN.clone();
    Z.SW = L.SW.clone();

    I.NE = [( 1, 0),(-1, 0),( 2, 0),(-1,-1),( 2, 2)].to_vec();
    I.ES = [( 0,-1),(-1,-1),( 2,-1),(-1, 1),( 2,-2)].to_vec();
    I.SW = [(-1, 0),( 1, 0),(-2, 0),( 1, 1),(-2,-2)].to_vec();
    I.WN = [( 0, 1),( 1, 1),(-2, 1),( 1,-1),(-2, 2)].to_vec();
    
    I.NW = [( 0,-1),(-1,-1),( 2,-1),(-1, 1),( 2,-2)].to_vec();
    I.WS = [( 1, 0),(-1, 0),( 2, 0),(-1,-1),( 2, 2)].to_vec();
    I.SE = [( 0, 1),( 1, 1),(-2, 1),( 1,-1),(-2, 2)].to_vec();
    I.EN = [(-1, 0),( 1, 0),(-2, 0),( 1, 1),(-2,-2)].to_vec();
    
    I.NS = [( 1,-1),( 1,0)].to_vec();
    I.EW = [(-1,-1),(0,-1)].to_vec();
    I.SN = [(-1, 1),(-1,0)].to_vec();
    I.WE = [( 1, 1),(0, 1)].to_vec();
}
pub(crate) struct Art {
    pub(crate) big: &'static str,
    pub(crate) small: &'static str,
}

pub(crate) struct AllArt {
    pub(crate) arch: Art,
    pub(crate) windows: Art,
}

pub(crate) const ALL_ART: AllArt = AllArt {
    arch: Art {
        big: include_str!("art_collection/arch/big.ascii"),
        small: include_str!("art_collection/arch/small.ascii"),
    },
    windows: Art {
        big: include_str!("art_collection/windows/big.ascii"),
        small: include_str!("art_collection/windows/small.ascii"),
    },
};
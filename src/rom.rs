#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Entry"]
    pub entry: [ENTRY; 3],
    #[doc = "0x0c - End of Table Marker Register"]
    pub tablemark: TABLEMARK,
    _reserved2: [u8; 4028usize],
    #[doc = "0xfcc - System Access Register"]
    pub sysaccess: SYSACCESS,
    #[doc = "0xfd0 - Peripheral ID Register"]
    pub periphid4: PERIPHID,
    #[doc = "0xfd4 - Peripheral ID Register"]
    pub periphid5: PERIPHID,
    #[doc = "0xfd8 - Peripheral ID Register"]
    pub periphid6: PERIPHID,
    #[doc = "0xfdc - Peripheral ID Register"]
    pub periphid7: PERIPHID,
    #[doc = "0xfe0 - Peripheral ID Register"]
    pub periphid0: PERIPHID,
    #[doc = "0xfe4 - Peripheral ID Register"]
    pub periphid1: PERIPHID,
    #[doc = "0xfe8 - Peripheral ID Register"]
    pub periphid2: PERIPHID,
    #[doc = "0xfec - Peripheral ID Register"]
    pub periphid3: PERIPHID,
    #[doc = "0xff0 - Component ID Register"]
    pub compid: [COMPID; 4],
}
#[doc = "Entry\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [entry](entry) module"]
pub type ENTRY = crate::Reg<u32, _ENTRY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ENTRY;
#[doc = "`read()` method returns [entry::R](entry::R) reader structure"]
impl crate::Readable for ENTRY {}
#[doc = "Entry"]
pub mod entry;
#[doc = "End of Table Marker Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tablemark](tablemark) module"]
pub type TABLEMARK = crate::Reg<u32, _TABLEMARK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TABLEMARK;
#[doc = "`read()` method returns [tablemark::R](tablemark::R) reader structure"]
impl crate::Readable for TABLEMARK {}
#[doc = "End of Table Marker Register"]
pub mod tablemark;
#[doc = "System Access Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysaccess](sysaccess) module"]
pub type SYSACCESS = crate::Reg<u32, _SYSACCESS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSACCESS;
#[doc = "`read()` method returns [sysaccess::R](sysaccess::R) reader structure"]
impl crate::Readable for SYSACCESS {}
#[doc = "System Access Register"]
pub mod sysaccess;
#[doc = "Peripheral ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periphid](periphid) module"]
pub type PERIPHID = crate::Reg<u32, _PERIPHID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERIPHID;
#[doc = "`read()` method returns [periphid::R](periphid::R) reader structure"]
impl crate::Readable for PERIPHID {}
#[doc = "Peripheral ID Register"]
pub mod periphid;
#[doc = "Component ID Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [compid](compid) module"]
pub type COMPID = crate::Reg<u32, _COMPID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _COMPID;
#[doc = "`read()` method returns [compid::R](compid::R) reader structure"]
impl crate::Readable for COMPID {}
#[doc = "Component ID Register"]
pub mod compid;

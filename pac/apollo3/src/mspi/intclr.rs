#[doc = "Register `INTCLR` reader"]
pub struct R(crate::R<INTCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTCLR` writer"]
pub struct W(crate::W<INTCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INTCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCRERR` reader - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
pub struct SCRERR_R(crate::FieldReader<bool, bool>);
impl SCRERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCRERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCRERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCRERR` writer - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
pub struct SCRERR_W<'a> {
    w: &'a mut W,
}
impl<'a> SCRERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `CQERR` reader - Command Queue Error Interrupt"]
pub struct CQERR_R(crate::FieldReader<bool, bool>);
impl CQERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQERR` writer - Command Queue Error Interrupt"]
pub struct CQERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CQERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `CQPAUSED` reader - Command Queue is Paused."]
pub struct CQPAUSED_R(crate::FieldReader<bool, bool>);
impl CQPAUSED_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQPAUSED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQPAUSED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQPAUSED` writer - Command Queue is Paused."]
pub struct CQPAUSED_W<'a> {
    w: &'a mut W,
}
impl<'a> CQPAUSED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `CQUPD` reader - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
pub struct CQUPD_R(crate::FieldReader<bool, bool>);
impl CQUPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQUPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQUPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQUPD` writer - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
pub struct CQUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> CQUPD_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `CQCMP` reader - Command Queue Complete Interrupt"]
pub struct CQCMP_R(crate::FieldReader<bool, bool>);
impl CQCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CQCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CQCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CQCMP` writer - Command Queue Complete Interrupt"]
pub struct CQCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CQCMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DERR` reader - DMA Error Interrupt"]
pub struct DERR_R(crate::FieldReader<bool, bool>);
impl DERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DERR` writer - DMA Error Interrupt"]
pub struct DERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DERR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `DCMP` reader - DMA Complete Interrupt"]
pub struct DCMP_R(crate::FieldReader<bool, bool>);
impl DCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCMP` writer - DMA Complete Interrupt"]
pub struct DCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> DCMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RXF` reader - Receive FIFO full"]
pub struct RXF_R(crate::FieldReader<bool, bool>);
impl RXF_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXF` writer - Receive FIFO full"]
pub struct RXF_W<'a> {
    w: &'a mut W,
}
impl<'a> RXF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `RXO` reader - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
pub struct RXO_R(crate::FieldReader<bool, bool>);
impl RXO_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXO` writer - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
pub struct RXO_W<'a> {
    w: &'a mut W,
}
impl<'a> RXO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `RXU` reader - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
pub struct RXU_R(crate::FieldReader<bool, bool>);
impl RXU_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXU` writer - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
pub struct RXU_W<'a> {
    w: &'a mut W,
}
impl<'a> RXU_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `TXO` reader - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
pub struct TXO_R(crate::FieldReader<bool, bool>);
impl TXO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXO` writer - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
pub struct TXO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXO_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `TXE` reader - Transmit FIFO empty."]
pub struct TXE_R(crate::FieldReader<bool, bool>);
impl TXE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE` writer - Transmit FIFO empty."]
pub struct TXE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CMDCMP` reader - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously"]
pub struct CMDCMP_R(crate::FieldReader<bool, bool>);
impl CMDCMP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMDCMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDCMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDCMP` writer - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously"]
pub struct CMDCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDCMP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 12 - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub fn screrr(&self) -> SCRERR_R {
        SCRERR_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Command Queue Error Interrupt"]
    #[inline(always)]
    pub fn cqerr(&self) -> CQERR_R {
        CQERR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Command Queue is Paused."]
    #[inline(always)]
    pub fn cqpaused(&self) -> CQPAUSED_R {
        CQPAUSED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub fn cqupd(&self) -> CQUPD_R {
        CQUPD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Command Queue Complete Interrupt"]
    #[inline(always)]
    pub fn cqcmp(&self) -> CQCMP_R {
        CQCMP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA Error Interrupt"]
    #[inline(always)]
    pub fn derr(&self) -> DERR_R {
        DERR_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - DMA Complete Interrupt"]
    #[inline(always)]
    pub fn dcmp(&self) -> DCMP_R {
        DCMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
    #[inline(always)]
    pub fn rxo(&self) -> RXO_R {
        RXO_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
    #[inline(always)]
    pub fn rxu(&self) -> RXU_R {
        RXU_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously"]
    #[inline(always)]
    pub fn cmdcmp(&self) -> CMDCMP_R {
        CMDCMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Scrambling Alignment Error. Scrambling operations must be aligned to word (4-byte) start address."]
    #[inline(always)]
    pub fn screrr(&mut self) -> SCRERR_W {
        SCRERR_W { w: self }
    }
    #[doc = "Bit 11 - Command Queue Error Interrupt"]
    #[inline(always)]
    pub fn cqerr(&mut self) -> CQERR_W {
        CQERR_W { w: self }
    }
    #[doc = "Bit 10 - Command Queue is Paused."]
    #[inline(always)]
    pub fn cqpaused(&mut self) -> CQPAUSED_W {
        CQPAUSED_W { w: self }
    }
    #[doc = "Bit 9 - Command Queue Update Interrupt. Issued whenever the CQ performs an operation where address bit\\[0\\]
is set. Useful for triggering CURIDX interrupts."]
    #[inline(always)]
    pub fn cqupd(&mut self) -> CQUPD_W {
        CQUPD_W { w: self }
    }
    #[doc = "Bit 8 - Command Queue Complete Interrupt"]
    #[inline(always)]
    pub fn cqcmp(&mut self) -> CQCMP_W {
        CQCMP_W { w: self }
    }
    #[doc = "Bit 7 - DMA Error Interrupt"]
    #[inline(always)]
    pub fn derr(&mut self) -> DERR_W {
        DERR_W { w: self }
    }
    #[doc = "Bit 6 - DMA Complete Interrupt"]
    #[inline(always)]
    pub fn dcmp(&mut self) -> DCMP_W {
        DCMP_W { w: self }
    }
    #[doc = "Bit 5 - Receive FIFO full"]
    #[inline(always)]
    pub fn rxf(&mut self) -> RXF_W {
        RXF_W { w: self }
    }
    #[doc = "Bit 4 - Receive FIFO overflow (cannot happen in MSPI design -- MSPI bus pins will stall)"]
    #[inline(always)]
    pub fn rxo(&mut self) -> RXO_W {
        RXO_W { w: self }
    }
    #[doc = "Bit 3 - Receive FIFO underflow (only occurs when SW reads from an empty FIFO)"]
    #[inline(always)]
    pub fn rxu(&mut self) -> RXU_W {
        RXU_W { w: self }
    }
    #[doc = "Bit 2 - Transmit FIFO Overflow (only occurs when SW writes to a full FIFO)."]
    #[inline(always)]
    pub fn txo(&mut self) -> TXO_W {
        TXO_W { w: self }
    }
    #[doc = "Bit 1 - Transmit FIFO empty."]
    #[inline(always)]
    pub fn txe(&mut self) -> TXE_W {
        TXE_W { w: self }
    }
    #[doc = "Bit 0 - Transfer complete. Note that DMA and CQ operations are layered, so CMDCMP, DCMP, and CQ* can all be signalled simultaneously"]
    #[inline(always)]
    pub fn cmdcmp(&mut self) -> CMDCMP_W {
        CMDCMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI Master Interrupts: Clear\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intclr](index.html) module"]
pub struct INTCLR_SPEC;
impl crate::RegisterSpec for INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intclr::R](R) reader structure"]
impl crate::Readable for INTCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intclr::W](W) writer structure"]
impl crate::Writable for INTCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for INTCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERBYTES` reader - Number of bytes to transmit or receive (based on TXRX bit)"]
pub struct XFERBYTES_R(crate::FieldReader<u16, u16>);
impl XFERBYTES_R {
    pub(crate) fn new(bits: u16) -> Self {
        XFERBYTES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFERBYTES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFERBYTES` writer - Number of bytes to transmit or receive (based on TXRX bit)"]
pub struct XFERBYTES_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERBYTES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
#[doc = "Field `PIOSCRAMBLE` reader - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
pub struct PIOSCRAMBLE_R(crate::FieldReader<bool, bool>);
impl PIOSCRAMBLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PIOSCRAMBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIOSCRAMBLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PIOSCRAMBLE` writer - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
pub struct PIOSCRAMBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> PIOSCRAMBLE_W<'a> {
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
#[doc = "Field `TXRX` reader - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
pub struct TXRX_R(crate::FieldReader<bool, bool>);
impl TXRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRX` writer - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
pub struct TXRX_W<'a> {
    w: &'a mut W,
}
impl<'a> TXRX_W<'a> {
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
#[doc = "Field `SENDI` reader - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
pub struct SENDI_R(crate::FieldReader<bool, bool>);
impl SENDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        SENDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENDI` writer - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
pub struct SENDI_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDI_W<'a> {
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
#[doc = "Field `SENDA` reader - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
pub struct SENDA_R(crate::FieldReader<bool, bool>);
impl SENDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SENDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SENDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SENDA` writer - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
pub struct SENDA_W<'a> {
    w: &'a mut W,
}
impl<'a> SENDA_W<'a> {
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
#[doc = "Field `ENTURN` reader - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
pub struct ENTURN_R(crate::FieldReader<bool, bool>);
impl ENTURN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENTURN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENTURN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENTURN` writer - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
pub struct ENTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> ENTURN_W<'a> {
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
#[doc = "Field `BIGENDIAN` reader - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
pub struct BIGENDIAN_R(crate::FieldReader<bool, bool>);
impl BIGENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BIGENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIGENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIGENDIAN` writer - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
pub struct BIGENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIGENDIAN_W<'a> {
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
#[doc = "Field `QUADCMD` reader - Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
pub struct QUADCMD_R(crate::FieldReader<bool, bool>);
impl QUADCMD_R {
    pub(crate) fn new(bits: bool) -> Self {
        QUADCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUADCMD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QUADCMD` writer - Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
pub struct QUADCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> QUADCMD_W<'a> {
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
#[doc = "Field `BUSY` reader - Command status: 1 indicates controller is busy (command in progress)"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUSY` writer - Command status: 1 indicates controller is busy (command in progress)"]
pub struct BUSY_W<'a> {
    w: &'a mut W,
}
impl<'a> BUSY_W<'a> {
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
#[doc = "Field `STATUS` reader - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
pub struct STATUS_R(crate::FieldReader<bool, bool>);
impl STATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STATUS` writer - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
pub struct STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> STATUS_W<'a> {
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
#[doc = "Field `START` reader - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
pub struct START_R(crate::FieldReader<bool, bool>);
impl START_R {
    pub(crate) fn new(bits: bool) -> Self {
        START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `START` writer - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
pub struct START_W<'a> {
    w: &'a mut W,
}
impl<'a> START_W<'a> {
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
    #[doc = "Bits 16:31 - Number of bytes to transmit or receive (based on TXRX bit)"]
    #[inline(always)]
    pub fn xferbytes(&self) -> XFERBYTES_R {
        XFERBYTES_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 11 - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[inline(always)]
    pub fn pioscramble(&self) -> PIOSCRAMBLE_R {
        PIOSCRAMBLE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
    #[inline(always)]
    pub fn txrx(&self) -> TXRX_R {
        TXRX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
    #[inline(always)]
    pub fn sendi(&self) -> SENDI_R {
        SENDI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
    #[inline(always)]
    pub fn senda(&self) -> SENDA_R {
        SENDA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[inline(always)]
    pub fn enturn(&self) -> ENTURN_R {
        ENTURN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[inline(always)]
    pub fn bigendian(&self) -> BIGENDIAN_R {
        BIGENDIAN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
    #[inline(always)]
    pub fn quadcmd(&self) -> QUADCMD_R {
        QUADCMD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Command status: 1 indicates controller is busy (command in progress)"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:31 - Number of bytes to transmit or receive (based on TXRX bit)"]
    #[inline(always)]
    pub fn xferbytes(&mut self) -> XFERBYTES_W {
        XFERBYTES_W { w: self }
    }
    #[doc = "Bit 11 - Enables data scrambling for PIO opertions. This should only be used for data operations and never for commands to a device."]
    #[inline(always)]
    pub fn pioscramble(&mut self) -> PIOSCRAMBLE_W {
        PIOSCRAMBLE_W { w: self }
    }
    #[doc = "Bit 10 - 1 Indicates a TX operation, 0 indicates an RX operation of XFERBYTES"]
    #[inline(always)]
    pub fn txrx(&mut self) -> TXRX_W {
        TXRX_W { w: self }
    }
    #[doc = "Bit 9 - Indicates whether an instruction phase should be sent (see INSTR field and ISIZE field in CFG register)"]
    #[inline(always)]
    pub fn sendi(&mut self) -> SENDI_W {
        SENDI_W { w: self }
    }
    #[doc = "Bit 8 - Indicates whether an address phase should be sent (see ADDR register and ASIZE field in CFG register)"]
    #[inline(always)]
    pub fn senda(&mut self) -> SENDA_W {
        SENDA_W { w: self }
    }
    #[doc = "Bit 7 - Indicates whether TX->RX turnaround cycles should be enabled for this operation (see TURNAROUND field in CFG register)."]
    #[inline(always)]
    pub fn enturn(&mut self) -> ENTURN_W {
        ENTURN_W { w: self }
    }
    #[doc = "Bit 6 - 1 indicates data in FIFO is in big endian format (MSB first); 0 indicates little endian data (default, LSB first)."]
    #[inline(always)]
    pub fn bigendian(&mut self) -> BIGENDIAN_W {
        BIGENDIAN_W { w: self }
    }
    #[doc = "Bit 3 - Flag indicating that the operation is a command that should be replicated to both devices in paired QUAD mode. This is typically only used when reading/writing configuration registers in paired flash devices (do not set for memory transfers)."]
    #[inline(always)]
    pub fn quadcmd(&mut self) -> QUADCMD_W {
        QUADCMD_W { w: self }
    }
    #[doc = "Bit 2 - Command status: 1 indicates controller is busy (command in progress)"]
    #[inline(always)]
    pub fn busy(&mut self) -> BUSY_W {
        BUSY_W { w: self }
    }
    #[doc = "Bit 1 - Command status: 1 indicates command has completed. Cleared by writing 1 to this bit or starting a new transfer."]
    #[inline(always)]
    pub fn status(&mut self) -> STATUS_W {
        STATUS_W { w: self }
    }
    #[doc = "Bit 0 - Write to 1 to initiate a PIO transaction on the bus (typically the entire register should be written at once with this bit set)."]
    #[inline(always)]
    pub fn start(&mut self) -> START_W {
        START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI PIO Transfer Control/Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

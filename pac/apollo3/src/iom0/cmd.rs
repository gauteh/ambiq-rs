#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSETLO` reader - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
pub struct OFFSETLO_R(crate::FieldReader<u8, u8>);
impl OFFSETLO_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFSETLO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETLO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETLO` writer - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
pub struct OFFSETLO_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETLO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `CMDSEL` reader - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
pub struct CMDSEL_R(crate::FieldReader<u8, u8>);
impl CMDSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMDSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMDSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSEL` writer - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
pub struct CMDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `TSIZE` reader - Defines the transaction size in bytes. The offset transfer is not included in this size."]
pub struct TSIZE_R(crate::FieldReader<u16, u16>);
impl TSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSIZE` writer - Defines the transaction size in bytes. The offset transfer is not included in this size."]
pub struct TSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Field `CONT` reader - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
pub struct CONT_R(crate::FieldReader<bool, bool>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
#[doc = "Field `OFFSETCNT` reader - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted first, then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
pub struct OFFSETCNT_R(crate::FieldReader<u8, u8>);
impl OFFSETCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        OFFSETCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSETCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSETCNT` writer - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted first, then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
pub struct OFFSETCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSETCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
#[doc = "Command for submodule.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMD_A {
    #[doc = "1: Write command using count of offset bytes specified in the OFFSETCNT field value."]
    WRITE = 1,
    #[doc = "2: Read command using count of offset bytes specified in the OFFSETCNT field value."]
    READ = 2,
    #[doc = "3: SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field value."]
    TMW = 3,
    #[doc = "4: SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input value."]
    TMR = 4,
}
impl From<CMD_A> for u8 {
    #[inline(always)]
    fn from(variant: CMD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` reader - Command for submodule."]
pub struct CMD_R(crate::FieldReader<u8, CMD_A>);
impl CMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMD_A> {
        match self.bits {
            1 => Some(CMD_A::WRITE),
            2 => Some(CMD_A::READ),
            3 => Some(CMD_A::TMW),
            4 => Some(CMD_A::TMR),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        **self == CMD_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        **self == CMD_A::READ
    }
    #[doc = "Checks if the value of the field is `TMW`"]
    #[inline(always)]
    pub fn is_tmw(&self) -> bool {
        **self == CMD_A::TMW
    }
    #[doc = "Checks if the value of the field is `TMR`"]
    #[inline(always)]
    pub fn is_tmr(&self) -> bool {
        **self == CMD_A::TMR
    }
}
impl core::ops::Deref for CMD_R {
    type Target = crate::FieldReader<u8, CMD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMD` writer - Command for submodule."]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Write command using count of offset bytes specified in the OFFSETCNT field value."]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(CMD_A::WRITE)
    }
    #[doc = "Read command using count of offset bytes specified in the OFFSETCNT field value."]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(CMD_A::READ)
    }
    #[doc = "SPI only. Test mode to do constant write operations. Useful for debug and power measurements. Will continually send data in OFFSET field value."]
    #[inline(always)]
    pub fn tmw(self) -> &'a mut W {
        self.variant(CMD_A::TMW)
    }
    #[doc = "SPI Only. Test mode to do constant read operations. Useful for debug and power measurements. Will continually read data from external input value."]
    #[inline(always)]
    pub fn tmr(self) -> &'a mut W {
        self.variant(CMD_A::TMR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[inline(always)]
    pub fn offsetlo(&self) -> OFFSETLO_R {
        OFFSETLO_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 20:21 - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
    #[inline(always)]
    pub fn cmdsel(&self) -> CMDSEL_R {
        CMDSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 5:6 - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted first, then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline(always)]
    pub fn offsetcnt(&self) -> OFFSETCNT_R {
        OFFSETCNT_R::new(((self.bits >> 5) & 0x03) as u8)
    }
    #[doc = "Bits 0:4 - Command for submodule."]
    #[inline(always)]
    pub fn cmd(&self) -> CMD_R {
        CMD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31 - This register holds the low order byte of offset to be used in the transaction. The number of offset bytes to use is set with bits 1:0 of the command."]
    #[inline(always)]
    pub fn offsetlo(&mut self) -> OFFSETLO_W {
        OFFSETLO_W { w: self }
    }
    #[doc = "Bits 20:21 - Command Specific selection information. Not used in Master I2C. Used as CEn select for Master SPI transactions"]
    #[inline(always)]
    pub fn cmdsel(&mut self) -> CMDSEL_W {
        CMDSEL_W { w: self }
    }
    #[doc = "Bits 8:19 - Defines the transaction size in bytes. The offset transfer is not included in this size."]
    #[inline(always)]
    pub fn tsize(&mut self) -> TSIZE_W {
        TSIZE_W { w: self }
    }
    #[doc = "Bit 7 - Contine to hold the bus after the current transaction if set to a 1 with a new command issued."]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Bits 5:6 - Number of offset bytes to use for the command - 0, 1, 2, 3 are valid selections. The second (byte 1) and third byte (byte 2) are read from the OFFSETHI register, and the low order byte is pulled from this register in the OFFSETLO field. Offset bytes are transmitted highest byte first. EG if offsetcnt == 3, OFFSETHI\\[15:8\\]
will be transmitted first, then OFFSETHI\\[7:0\\]
then OFFSETLO. If offsetcnt == 2, OFFSETHI\\[7:0\\]
will be transmitted, then OFFSETLO. If offsetcnt == 1, only OFFSETLO will be transmitted. Offset bytes are always transmitted MSB first, regardless of the value of the LSB control bit within the module configuration."]
    #[inline(always)]
    pub fn offsetcnt(&mut self) -> OFFSETCNT_W {
        OFFSETCNT_W { w: self }
    }
    #[doc = "Bits 0:4 - Command for submodule."]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command and offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

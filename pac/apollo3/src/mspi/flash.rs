#[doc = "Register `FLASH` reader"]
pub struct R(crate::R<FLASH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLASH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLASH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLASH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLASH` writer"]
pub struct W(crate::W<FLASH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLASH_SPEC>;
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
impl From<crate::W<FLASH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLASH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READINSTR` reader - Read command sent to flash for DMA/XIP operations"]
pub struct READINSTR_R(crate::FieldReader<u8, u8>);
impl READINSTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        READINSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READINSTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `READINSTR` writer - Read command sent to flash for DMA/XIP operations"]
pub struct READINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> READINSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
#[doc = "Field `WRITEINSTR` reader - Write command sent for DMA operations"]
pub struct WRITEINSTR_R(crate::FieldReader<u8, u8>);
impl WRITEINSTR_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRITEINSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITEINSTR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WRITEINSTR` writer - Write command sent for DMA operations"]
pub struct WRITEINSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEINSTR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `XIPMIXED` reader - Reserved. Set to 0x0"]
pub struct XIPMIXED_R(crate::FieldReader<u8, u8>);
impl XIPMIXED_R {
    pub(crate) fn new(bits: u8) -> Self {
        XIPMIXED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIPMIXED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPMIXED` writer - Reserved. Set to 0x0"]
pub struct XIPMIXED_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPMIXED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `XIPSENDI` reader - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
pub struct XIPSENDI_R(crate::FieldReader<bool, bool>);
impl XIPSENDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        XIPSENDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIPSENDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPSENDI` writer - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
pub struct XIPSENDI_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPSENDI_W<'a> {
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
#[doc = "Field `XIPSENDA` reader - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
pub struct XIPSENDA_R(crate::FieldReader<bool, bool>);
impl XIPSENDA_R {
    pub(crate) fn new(bits: bool) -> Self {
        XIPSENDA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIPSENDA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPSENDA` writer - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
pub struct XIPSENDA_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPSENDA_W<'a> {
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
#[doc = "Field `XIPENTURN` reader - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
pub struct XIPENTURN_R(crate::FieldReader<bool, bool>);
impl XIPENTURN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XIPENTURN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIPENTURN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPENTURN` writer - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
pub struct XIPENTURN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPENTURN_W<'a> {
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
#[doc = "Field `XIPBIGENDIAN` reader - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
pub struct XIPBIGENDIAN_R(crate::FieldReader<bool, bool>);
impl XIPBIGENDIAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XIPBIGENDIAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIPBIGENDIAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPBIGENDIAN` writer - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
pub struct XIPBIGENDIAN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPBIGENDIAN_W<'a> {
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
#[doc = "Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum XIPACK_A {
    #[doc = "0: No acknowledege sent.  Data IOs are tristated the first turnaround cycle value."]
    NOACK = 0,
    #[doc = "2: Positive acknowledege sent.  Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode value."]
    ACK = 2,
    #[doc = "3: Negative acknowledege sent.  Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode.  XIPSENDI should be reenabled for the next transfer value."]
    TERMINATE = 3,
}
impl From<XIPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: XIPACK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `XIPACK` reader - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
pub struct XIPACK_R(crate::FieldReader<u8, XIPACK_A>);
impl XIPACK_R {
    pub(crate) fn new(bits: u8) -> Self {
        XIPACK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<XIPACK_A> {
        match self.bits {
            0 => Some(XIPACK_A::NOACK),
            2 => Some(XIPACK_A::ACK),
            3 => Some(XIPACK_A::TERMINATE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NOACK`"]
    #[inline(always)]
    pub fn is_noack(&self) -> bool {
        **self == XIPACK_A::NOACK
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        **self == XIPACK_A::ACK
    }
    #[doc = "Checks if the value of the field is `TERMINATE`"]
    #[inline(always)]
    pub fn is_terminate(&self) -> bool {
        **self == XIPACK_A::TERMINATE
    }
}
impl core::ops::Deref for XIPACK_R {
    type Target = crate::FieldReader<u8, XIPACK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPACK` writer - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
pub struct XIPACK_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPACK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: XIPACK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No acknowledege sent. Data IOs are tristated the first turnaround cycle value."]
    #[inline(always)]
    pub fn noack(self) -> &'a mut W {
        self.variant(XIPACK_A::NOACK)
    }
    #[doc = "Positive acknowledege sent. Data IOs are driven to 0 the first turnaround cycle to acknowledge XIP mode value."]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(XIPACK_A::ACK)
    }
    #[doc = "Negative acknowledege sent. Data IOs are driven to 1 the first turnaround cycle to terminate XIP mode. XIPSENDI should be reenabled for the next transfer value."]
    #[inline(always)]
    pub fn terminate(self) -> &'a mut W {
        self.variant(XIPACK_A::TERMINATE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `XIPEN` reader - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
pub struct XIPEN_R(crate::FieldReader<bool, bool>);
impl XIPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        XIPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XIPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XIPEN` writer - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
pub struct XIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> XIPEN_W<'a> {
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
    #[doc = "Bits 24:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline(always)]
    pub fn readinstr(&self) -> READINSTR_R {
        READINSTR_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write command sent for DMA operations"]
    #[inline(always)]
    pub fn writeinstr(&self) -> WRITEINSTR_R {
        WRITEINSTR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Reserved. Set to 0x0"]
    #[inline(always)]
    pub fn xipmixed(&self) -> XIPMIXED_R {
        XIPMIXED_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsendi(&self) -> XIPSENDI_R {
        XIPSENDI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsenda(&self) -> XIPSENDA_R {
        XIPSENDA_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline(always)]
    pub fn xipenturn(&self) -> XIPENTURN_R {
        XIPENTURN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline(always)]
    pub fn xipbigendian(&self) -> XIPBIGENDIAN_R {
        XIPBIGENDIAN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline(always)]
    pub fn xipack(&self) -> XIPACK_R {
        XIPACK_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline(always)]
    pub fn xipen(&self) -> XIPEN_R {
        XIPEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:31 - Read command sent to flash for DMA/XIP operations"]
    #[inline(always)]
    pub fn readinstr(&mut self) -> READINSTR_W {
        READINSTR_W { w: self }
    }
    #[doc = "Bits 16:23 - Write command sent for DMA operations"]
    #[inline(always)]
    pub fn writeinstr(&mut self) -> WRITEINSTR_W {
        WRITEINSTR_W { w: self }
    }
    #[doc = "Bits 8:10 - Reserved. Set to 0x0"]
    #[inline(always)]
    pub fn xipmixed(&mut self) -> XIPMIXED_W {
        XIPMIXED_W { w: self }
    }
    #[doc = "Bit 7 - Indicates whether XIP/AUTO DMA operations should send an instruction (see READINSTR field and ISIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsendi(&mut self) -> XIPSENDI_W {
        XIPSENDI_W { w: self }
    }
    #[doc = "Bit 6 - Indicates whether XIP/AUTO DMA operations should send an an address phase (see DMADEVADDR register and ASIZE field in CFG)"]
    #[inline(always)]
    pub fn xipsenda(&mut self) -> XIPSENDA_W {
        XIPSENDA_W { w: self }
    }
    #[doc = "Bit 5 - Indicates whether XIP/AUTO DMA operations should enable TX->RX turnaround cycles"]
    #[inline(always)]
    pub fn xipenturn(&mut self) -> XIPENTURN_W {
        XIPENTURN_W { w: self }
    }
    #[doc = "Bit 4 - Indicates whether XIP/AUTO DMA data transfers are in big or little endian format"]
    #[inline(always)]
    pub fn xipbigendian(&mut self) -> XIPBIGENDIAN_W {
        XIPBIGENDIAN_W { w: self }
    }
    #[doc = "Bits 2:3 - Controls transmission of Micron XIP acknowledge cycles (Micron Flash devices only)"]
    #[inline(always)]
    pub fn xipack(&mut self) -> XIPACK_W {
        XIPACK_W { w: self }
    }
    #[doc = "Bit 0 - Enable the XIP (eXecute In Place) function which effectively enables the address decoding of the MSPI device in the flash/cache address space at address 0x04000000-0x07FFFFFF."]
    #[inline(always)]
    pub fn xipen(&mut self) -> XIPEN_W {
        XIPEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration for XIP/DMA support of SPI flash modules.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flash](index.html) module"]
pub struct FLASH_SPEC;
impl crate::RegisterSpec for FLASH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flash::R](R) reader structure"]
impl crate::Readable for FLASH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flash::W](W) writer structure"]
impl crate::Writable for FLASH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLASH to value 0x0b06_0000"]
impl crate::Resettable for FLASH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0b06_0000
    }
}

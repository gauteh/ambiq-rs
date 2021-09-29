#[doc = "Register `CTRUP` reader"]
pub struct R(crate::R<CTRUP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRUP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRUP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRUP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRUP` writer"]
pub struct W(crate::W<CTRUP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRUP_SPEC>;
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
impl From<crate::W<CTRUP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRUP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTERR_A {
    #[doc = "0: No read error occurred value."]
    NOERR = 0,
    #[doc = "1: Read error occurred value."]
    RDERR = 1,
}
impl From<CTERR_A> for bool {
    #[inline(always)]
    fn from(variant: CTERR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTERR` reader - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
pub struct CTERR_R(crate::FieldReader<bool, CTERR_A>);
impl CTERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTERR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTERR_A {
        match self.bits {
            false => CTERR_A::NOERR,
            true => CTERR_A::RDERR,
        }
    }
    #[doc = "Checks if the value of the field is `NOERR`"]
    #[inline(always)]
    pub fn is_noerr(&self) -> bool {
        **self == CTERR_A::NOERR
    }
    #[doc = "Checks if the value of the field is `RDERR`"]
    #[inline(always)]
    pub fn is_rderr(&self) -> bool {
        **self == CTERR_A::RDERR
    }
}
impl core::ops::Deref for CTERR_R {
    type Target = crate::FieldReader<bool, CTERR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTERR` writer - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
pub struct CTERR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTERR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTERR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No read error occurred value."]
    #[inline(always)]
    pub fn noerr(self) -> &'a mut W {
        self.variant(CTERR_A::NOERR)
    }
    #[doc = "Read error occurred value."]
    #[inline(always)]
    pub fn rderr(self) -> &'a mut W {
        self.variant(CTERR_A::RDERR)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Century enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CEB_A {
    #[doc = "0: Disable the Century bit from changing value."]
    DIS = 0,
    #[doc = "1: Enable the Century bit to change value."]
    EN = 1,
}
impl From<CEB_A> for bool {
    #[inline(always)]
    fn from(variant: CEB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CEB` reader - Century enable"]
pub struct CEB_R(crate::FieldReader<bool, CEB_A>);
impl CEB_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEB_A {
        match self.bits {
            false => CEB_A::DIS,
            true => CEB_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CEB_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CEB_A::EN
    }
}
impl core::ops::Deref for CEB_R {
    type Target = crate::FieldReader<bool, CEB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEB` writer - Century enable"]
pub struct CEB_W<'a> {
    w: &'a mut W,
}
impl<'a> CEB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CEB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Century bit from changing value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CEB_A::DIS)
    }
    #[doc = "Enable the Century bit to change value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CEB_A::EN)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Century\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CB_A {
    #[doc = "0: Century is 2000s value."]
    _2000 = 0,
    #[doc = "1: Century is 1900s/2100s value."]
    _1900_2100 = 1,
}
impl From<CB_A> for bool {
    #[inline(always)]
    fn from(variant: CB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CB` reader - Century"]
pub struct CB_R(crate::FieldReader<bool, CB_A>);
impl CB_R {
    pub(crate) fn new(bits: bool) -> Self {
        CB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CB_A {
        match self.bits {
            false => CB_A::_2000,
            true => CB_A::_1900_2100,
        }
    }
    #[doc = "Checks if the value of the field is `_2000`"]
    #[inline(always)]
    pub fn is_2000(&self) -> bool {
        **self == CB_A::_2000
    }
    #[doc = "Checks if the value of the field is `_1900_2100`"]
    #[inline(always)]
    pub fn is_1900_2100(&self) -> bool {
        **self == CB_A::_1900_2100
    }
}
impl core::ops::Deref for CB_R {
    type Target = crate::FieldReader<bool, CB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CB` writer - Century"]
pub struct CB_W<'a> {
    w: &'a mut W,
}
impl<'a> CB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Century is 2000s value."]
    #[inline(always)]
    pub fn _2000(self) -> &'a mut W {
        self.variant(CB_A::_2000)
    }
    #[doc = "Century is 1900s/2100s value."]
    #[inline(always)]
    pub fn _1900_2100(self) -> &'a mut W {
        self.variant(CB_A::_1900_2100)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CTRWKDY` reader - Weekdays Counter"]
pub struct CTRWKDY_R(crate::FieldReader<u8, u8>);
impl CTRWKDY_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRWKDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRWKDY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRWKDY` writer - Weekdays Counter"]
pub struct CTRWKDY_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRWKDY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `CTRYR` reader - Years Counter"]
pub struct CTRYR_R(crate::FieldReader<u8, u8>);
impl CTRYR_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRYR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRYR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRYR` writer - Years Counter"]
pub struct CTRYR_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRYR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `CTRMO` reader - Months Counter"]
pub struct CTRMO_R(crate::FieldReader<u8, u8>);
impl CTRMO_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRMO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRMO` writer - Months Counter"]
pub struct CTRMO_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRMO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `CTRDATE` reader - Date Counter"]
pub struct CTRDATE_R(crate::FieldReader<u8, u8>);
impl CTRDATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CTRDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRDATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRDATE` writer - Date Counter"]
pub struct CTRDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRDATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[inline(always)]
    pub fn cterr(&self) -> CTERR_R {
        CTERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Century enable"]
    #[inline(always)]
    pub fn ceb(&self) -> CEB_R {
        CEB_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Century"]
    #[inline(always)]
    pub fn cb(&self) -> CB_R {
        CB_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 24:26 - Weekdays Counter"]
    #[inline(always)]
    pub fn ctrwkdy(&self) -> CTRWKDY_R {
        CTRWKDY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:23 - Years Counter"]
    #[inline(always)]
    pub fn ctryr(&self) -> CTRYR_R {
        CTRYR_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Months Counter"]
    #[inline(always)]
    pub fn ctrmo(&self) -> CTRMO_R {
        CTRMO_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 0:5 - Date Counter"]
    #[inline(always)]
    pub fn ctrdate(&self) -> CTRDATE_R {
        CTRDATE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - Counter read error status. Error is triggered when software reads the lower word of the counters, and fails to read the upper counter within 1/100 second. This is because when the lower counter is read, the upper counter is held off from incrementing until it is read so that the full time stamp can be read."]
    #[inline(always)]
    pub fn cterr(&mut self) -> CTERR_W {
        CTERR_W { w: self }
    }
    #[doc = "Bit 28 - Century enable"]
    #[inline(always)]
    pub fn ceb(&mut self) -> CEB_W {
        CEB_W { w: self }
    }
    #[doc = "Bit 27 - Century"]
    #[inline(always)]
    pub fn cb(&mut self) -> CB_W {
        CB_W { w: self }
    }
    #[doc = "Bits 24:26 - Weekdays Counter"]
    #[inline(always)]
    pub fn ctrwkdy(&mut self) -> CTRWKDY_W {
        CTRWKDY_W { w: self }
    }
    #[doc = "Bits 16:23 - Years Counter"]
    #[inline(always)]
    pub fn ctryr(&mut self) -> CTRYR_W {
        CTRYR_W { w: self }
    }
    #[doc = "Bits 8:12 - Months Counter"]
    #[inline(always)]
    pub fn ctrmo(&mut self) -> CTRMO_W {
        CTRMO_W { w: self }
    }
    #[doc = "Bits 0:5 - Date Counter"]
    #[inline(always)]
    pub fn ctrdate(&mut self) -> CTRDATE_W {
        CTRDATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Counters Upper\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrup](index.html) module"]
pub struct CTRUP_SPEC;
impl crate::RegisterSpec for CTRUP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrup::R](R) reader structure"]
impl crate::Readable for CTRUP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrup::W](W) writer structure"]
impl crate::Writable for CTRUP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRUP to value 0"]
impl crate::Resettable for CTRUP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

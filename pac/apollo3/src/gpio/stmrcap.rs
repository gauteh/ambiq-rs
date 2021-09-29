#[doc = "Register `STMRCAP` reader"]
pub struct R(crate::R<STMRCAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STMRCAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STMRCAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STMRCAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STMRCAP` writer"]
pub struct W(crate::W<STMRCAP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STMRCAP_SPEC>;
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
impl From<crate::W<STMRCAP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STMRCAP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "STIMER Capture 3 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL3_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL3_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL3` reader - STIMER Capture 3 Polarity."]
pub struct STPOL3_R(crate::FieldReader<bool, STPOL3_A>);
impl STPOL3_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPOL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL3_A {
        match self.bits {
            false => STPOL3_A::CAPLH,
            true => STPOL3_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        **self == STPOL3_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        **self == STPOL3_A::CAPHL
    }
}
impl core::ops::Deref for STPOL3_R {
    type Target = crate::FieldReader<bool, STPOL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPOL3` writer - STIMER Capture 3 Polarity."]
pub struct STPOL3_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL3_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL3_A::CAPHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `STSEL3` reader - STIMER Capture 3 Select."]
pub struct STSEL3_R(crate::FieldReader<u8, u8>);
impl STSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        STSEL3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSEL3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSEL3` writer - STIMER Capture 3 Select."]
pub struct STSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 24)) | ((value as u32 & 0x3f) << 24);
        self.w
    }
}
#[doc = "STIMER Capture 2 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL2_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL2_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL2` reader - STIMER Capture 2 Polarity."]
pub struct STPOL2_R(crate::FieldReader<bool, STPOL2_A>);
impl STPOL2_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPOL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL2_A {
        match self.bits {
            false => STPOL2_A::CAPLH,
            true => STPOL2_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        **self == STPOL2_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        **self == STPOL2_A::CAPHL
    }
}
impl core::ops::Deref for STPOL2_R {
    type Target = crate::FieldReader<bool, STPOL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPOL2` writer - STIMER Capture 2 Polarity."]
pub struct STPOL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL2_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL2_A::CAPHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `STSEL2` reader - STIMER Capture 2 Select."]
pub struct STSEL2_R(crate::FieldReader<u8, u8>);
impl STSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        STSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSEL2` writer - STIMER Capture 2 Select."]
pub struct STSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "STIMER Capture 1 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL1_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL1` reader - STIMER Capture 1 Polarity."]
pub struct STPOL1_R(crate::FieldReader<bool, STPOL1_A>);
impl STPOL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPOL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL1_A {
        match self.bits {
            false => STPOL1_A::CAPLH,
            true => STPOL1_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        **self == STPOL1_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        **self == STPOL1_A::CAPHL
    }
}
impl core::ops::Deref for STPOL1_R {
    type Target = crate::FieldReader<bool, STPOL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPOL1` writer - STIMER Capture 1 Polarity."]
pub struct STPOL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL1_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL1_A::CAPHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `STSEL1` reader - STIMER Capture 1 Select."]
pub struct STSEL1_R(crate::FieldReader<u8, u8>);
impl STSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        STSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSEL1` writer - STIMER Capture 1 Select."]
pub struct STSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "STIMER Capture 0 Polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STPOL0_A {
    #[doc = "0: Capture on low to high GPIO transition value."]
    CAPLH = 0,
    #[doc = "1: Capture on high to low GPIO transition value."]
    CAPHL = 1,
}
impl From<STPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: STPOL0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STPOL0` reader - STIMER Capture 0 Polarity."]
pub struct STPOL0_R(crate::FieldReader<bool, STPOL0_A>);
impl STPOL0_R {
    pub(crate) fn new(bits: bool) -> Self {
        STPOL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STPOL0_A {
        match self.bits {
            false => STPOL0_A::CAPLH,
            true => STPOL0_A::CAPHL,
        }
    }
    #[doc = "Checks if the value of the field is `CAPLH`"]
    #[inline(always)]
    pub fn is_caplh(&self) -> bool {
        **self == STPOL0_A::CAPLH
    }
    #[doc = "Checks if the value of the field is `CAPHL`"]
    #[inline(always)]
    pub fn is_caphl(&self) -> bool {
        **self == STPOL0_A::CAPHL
    }
}
impl core::ops::Deref for STPOL0_R {
    type Target = crate::FieldReader<bool, STPOL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STPOL0` writer - STIMER Capture 0 Polarity."]
pub struct STPOL0_W<'a> {
    w: &'a mut W,
}
impl<'a> STPOL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STPOL0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Capture on low to high GPIO transition value."]
    #[inline(always)]
    pub fn caplh(self) -> &'a mut W {
        self.variant(STPOL0_A::CAPLH)
    }
    #[doc = "Capture on high to low GPIO transition value."]
    #[inline(always)]
    pub fn caphl(self) -> &'a mut W {
        self.variant(STPOL0_A::CAPHL)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `STSEL0` reader - STIMER Capture 0 Select."]
pub struct STSEL0_R(crate::FieldReader<u8, u8>);
impl STSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        STSEL0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STSEL0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STSEL0` writer - STIMER Capture 0 Select."]
pub struct STSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> STSEL0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - STIMER Capture 3 Polarity."]
    #[inline(always)]
    pub fn stpol3(&self) -> STPOL3_R {
        STPOL3_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 24:29 - STIMER Capture 3 Select."]
    #[inline(always)]
    pub fn stsel3(&self) -> STSEL3_R {
        STSEL3_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - STIMER Capture 2 Polarity."]
    #[inline(always)]
    pub fn stpol2(&self) -> STPOL2_R {
        STPOL2_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bits 16:21 - STIMER Capture 2 Select."]
    #[inline(always)]
    pub fn stsel2(&self) -> STSEL2_R {
        STSEL2_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - STIMER Capture 1 Polarity."]
    #[inline(always)]
    pub fn stpol1(&self) -> STPOL1_R {
        STPOL1_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - STIMER Capture 1 Select."]
    #[inline(always)]
    pub fn stsel1(&self) -> STSEL1_R {
        STSEL1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 6 - STIMER Capture 0 Polarity."]
    #[inline(always)]
    pub fn stpol0(&self) -> STPOL0_R {
        STPOL0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - STIMER Capture 0 Select."]
    #[inline(always)]
    pub fn stsel0(&self) -> STSEL0_R {
        STSEL0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - STIMER Capture 3 Polarity."]
    #[inline(always)]
    pub fn stpol3(&mut self) -> STPOL3_W {
        STPOL3_W { w: self }
    }
    #[doc = "Bits 24:29 - STIMER Capture 3 Select."]
    #[inline(always)]
    pub fn stsel3(&mut self) -> STSEL3_W {
        STSEL3_W { w: self }
    }
    #[doc = "Bit 22 - STIMER Capture 2 Polarity."]
    #[inline(always)]
    pub fn stpol2(&mut self) -> STPOL2_W {
        STPOL2_W { w: self }
    }
    #[doc = "Bits 16:21 - STIMER Capture 2 Select."]
    #[inline(always)]
    pub fn stsel2(&mut self) -> STSEL2_W {
        STSEL2_W { w: self }
    }
    #[doc = "Bit 14 - STIMER Capture 1 Polarity."]
    #[inline(always)]
    pub fn stpol1(&mut self) -> STPOL1_W {
        STPOL1_W { w: self }
    }
    #[doc = "Bits 8:13 - STIMER Capture 1 Select."]
    #[inline(always)]
    pub fn stsel1(&mut self) -> STSEL1_W {
        STSEL1_W { w: self }
    }
    #[doc = "Bit 6 - STIMER Capture 0 Polarity."]
    #[inline(always)]
    pub fn stpol0(&mut self) -> STPOL0_W {
        STPOL0_W { w: self }
    }
    #[doc = "Bits 0:5 - STIMER Capture 0 Select."]
    #[inline(always)]
    pub fn stsel0(&mut self) -> STSEL0_W {
        STSEL0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "STIMER Capture Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stmrcap](index.html) module"]
pub struct STMRCAP_SPEC;
impl crate::RegisterSpec for STMRCAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stmrcap::R](R) reader structure"]
impl crate::Readable for STMRCAP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stmrcap::W](W) writer structure"]
impl crate::Writable for STMRCAP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets STMRCAP to value 0x3f3f_3f3f"]
impl crate::Resettable for STMRCAP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f3f_3f3f
    }
}

#[doc = "Register `CLKCFG` reader"]
pub struct R(crate::R<CLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCFG` writer"]
pub struct W(crate::W<CLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCFG_SPEC>;
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
impl From<crate::W<CLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV3` reader - Enable of the divide by 3 of the source IOCLK."]
pub struct DIV3_R(crate::FieldReader<bool, bool>);
impl DIV3_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIV3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIV3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIV3` writer - Enable of the divide by 3 of the source IOCLK."]
pub struct DIV3_W<'a> {
    w: &'a mut W,
}
impl<'a> DIV3_W<'a> {
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
#[doc = "Field `CLK32KEN` reader - Enable for the 32Khz clock to the BLE module"]
pub struct CLK32KEN_R(crate::FieldReader<bool, bool>);
impl CLK32KEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLK32KEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLK32KEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK32KEN` writer - Enable for the 32Khz clock to the BLE module"]
pub struct CLK32KEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK32KEN_W<'a> {
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
#[doc = "Select the input clock frequency.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FSEL_A {
    #[doc = "0: Selects the minimum power clock.  This setting should be used whenever the IOM is not active. value."]
    MIN_PWR = 0,
    #[doc = "1: Selects the HFRC as the input clock. value."]
    HFRC = 1,
    #[doc = "2: Selects the HFRC / 2 as the input clock. value."]
    HFRC_DIV2 = 2,
    #[doc = "3: Selects the HFRC / 4 as the input clock. value."]
    HFRC_DIV4 = 3,
    #[doc = "4: Selects the HFRC / 8 as the input clock. value."]
    HFRC_DIV8 = 4,
    #[doc = "5: Selects the HFRC / 16 as the input clock. value."]
    HFRC_DIV16 = 5,
    #[doc = "6: Selects the HFRC / 32 as the input clock. value."]
    HFRC_DIV32 = 6,
    #[doc = "7: Selects the HFRC / 64 as the input clock. value."]
    HFRC_DIV64 = 7,
}
impl From<FSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FSEL` reader - Select the input clock frequency."]
pub struct FSEL_R(crate::FieldReader<u8, FSEL_A>);
impl FSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSEL_A {
        match self.bits {
            0 => FSEL_A::MIN_PWR,
            1 => FSEL_A::HFRC,
            2 => FSEL_A::HFRC_DIV2,
            3 => FSEL_A::HFRC_DIV4,
            4 => FSEL_A::HFRC_DIV8,
            5 => FSEL_A::HFRC_DIV16,
            6 => FSEL_A::HFRC_DIV32,
            7 => FSEL_A::HFRC_DIV64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MIN_PWR`"]
    #[inline(always)]
    pub fn is_min_pwr(&self) -> bool {
        **self == FSEL_A::MIN_PWR
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        **self == FSEL_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV2`"]
    #[inline(always)]
    pub fn is_hfrc_div2(&self) -> bool {
        **self == FSEL_A::HFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == FSEL_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        **self == FSEL_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == FSEL_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV32`"]
    #[inline(always)]
    pub fn is_hfrc_div32(&self) -> bool {
        **self == FSEL_A::HFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV64`"]
    #[inline(always)]
    pub fn is_hfrc_div64(&self) -> bool {
        **self == FSEL_A::HFRC_DIV64
    }
}
impl core::ops::Deref for FSEL_R {
    type Target = crate::FieldReader<u8, FSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSEL` writer - Select the input clock frequency."]
pub struct FSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Selects the minimum power clock. This setting should be used whenever the IOM is not active. value."]
    #[inline(always)]
    pub fn min_pwr(self) -> &'a mut W {
        self.variant(FSEL_A::MIN_PWR)
    }
    #[doc = "Selects the HFRC as the input clock. value."]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC)
    }
    #[doc = "Selects the HFRC / 2 as the input clock. value."]
    #[inline(always)]
    pub fn hfrc_div2(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV2)
    }
    #[doc = "Selects the HFRC / 4 as the input clock. value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV4)
    }
    #[doc = "Selects the HFRC / 8 as the input clock. value."]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV8)
    }
    #[doc = "Selects the HFRC / 16 as the input clock. value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV16)
    }
    #[doc = "Selects the HFRC / 32 as the input clock. value."]
    #[inline(always)]
    pub fn hfrc_div32(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV32)
    }
    #[doc = "Selects the HFRC / 64 as the input clock. value."]
    #[inline(always)]
    pub fn hfrc_div64(self) -> &'a mut W {
        self.variant(FSEL_A::HFRC_DIV64)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `IOCLKEN` reader - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
pub struct IOCLKEN_R(crate::FieldReader<bool, bool>);
impl IOCLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOCLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOCLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOCLKEN` writer - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
pub struct IOCLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IOCLKEN_W<'a> {
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
    #[doc = "Bit 12 - Enable of the divide by 3 of the source IOCLK."]
    #[inline(always)]
    pub fn div3(&self) -> DIV3_R {
        DIV3_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Enable for the 32Khz clock to the BLE module"]
    #[inline(always)]
    pub fn clk32ken(&self) -> CLK32KEN_R {
        CLK32KEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[inline(always)]
    pub fn ioclken(&self) -> IOCLKEN_R {
        IOCLKEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Enable of the divide by 3 of the source IOCLK."]
    #[inline(always)]
    pub fn div3(&mut self) -> DIV3_W {
        DIV3_W { w: self }
    }
    #[doc = "Bit 11 - Enable for the 32Khz clock to the BLE module"]
    #[inline(always)]
    pub fn clk32ken(&mut self) -> CLK32KEN_W {
        CLK32KEN_W { w: self }
    }
    #[doc = "Bits 8:10 - Select the input clock frequency."]
    #[inline(always)]
    pub fn fsel(&mut self) -> FSEL_W {
        FSEL_W { w: self }
    }
    #[doc = "Bit 0 - Enable for the interface clock. Must be enabled prior to executing any IO operations."]
    #[inline(always)]
    pub fn ioclken(&mut self) -> IOCLKEN_W {
        IOCLKEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkcfg](index.html) module"]
pub struct CLKCFG_SPEC;
impl crate::RegisterSpec for CLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkcfg::R](R) reader structure"]
impl crate::Readable for CLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkcfg::W](W) writer structure"]
impl crate::Writable for CLKCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCFG to value 0"]
impl crate::Resettable for CLKCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `SUBMODCTRL` reader"]
pub struct R(crate::R<SUBMODCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUBMODCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUBMODCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUBMODCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUBMODCTRL` writer"]
pub struct W(crate::W<SUBMODCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUBMODCTRL_SPEC>;
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
impl From<crate::W<SUBMODCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUBMODCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Submodule 0 module type. This is the I2C Master interface\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD1TYPE_A {
    #[doc = "0: SPI Master submodule value."]
    MSPI = 0,
    #[doc = "1: MI2C submodule value."]
    I2C_MASTER = 1,
    #[doc = "2: SPI Slave submodule value."]
    SSPI = 2,
    #[doc = "3: I2C Slave submodule value."]
    SI2C = 3,
    #[doc = "7: NOT INSTALLED value."]
    NA = 7,
}
impl From<SMOD1TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD1TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMOD1TYPE` reader - Submodule 0 module type. This is the I2C Master interface"]
pub struct SMOD1TYPE_R(crate::FieldReader<u8, SMOD1TYPE_A>);
impl SMOD1TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMOD1TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD1TYPE_A> {
        match self.bits {
            0 => Some(SMOD1TYPE_A::MSPI),
            1 => Some(SMOD1TYPE_A::I2C_MASTER),
            2 => Some(SMOD1TYPE_A::SSPI),
            3 => Some(SMOD1TYPE_A::SI2C),
            7 => Some(SMOD1TYPE_A::NA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MSPI`"]
    #[inline(always)]
    pub fn is_mspi(&self) -> bool {
        **self == SMOD1TYPE_A::MSPI
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        **self == SMOD1TYPE_A::I2C_MASTER
    }
    #[doc = "Checks if the value of the field is `SSPI`"]
    #[inline(always)]
    pub fn is_sspi(&self) -> bool {
        **self == SMOD1TYPE_A::SSPI
    }
    #[doc = "Checks if the value of the field is `SI2C`"]
    #[inline(always)]
    pub fn is_si2c(&self) -> bool {
        **self == SMOD1TYPE_A::SI2C
    }
    #[doc = "Checks if the value of the field is `NA`"]
    #[inline(always)]
    pub fn is_na(&self) -> bool {
        **self == SMOD1TYPE_A::NA
    }
}
impl core::ops::Deref for SMOD1TYPE_R {
    type Target = crate::FieldReader<u8, SMOD1TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOD1TYPE` writer - Submodule 0 module type. This is the I2C Master interface"]
pub struct SMOD1TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD1TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD1TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SPI Master submodule value."]
    #[inline(always)]
    pub fn mspi(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::MSPI)
    }
    #[doc = "MI2C submodule value."]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::I2C_MASTER)
    }
    #[doc = "SPI Slave submodule value."]
    #[inline(always)]
    pub fn sspi(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::SSPI)
    }
    #[doc = "I2C Slave submodule value."]
    #[inline(always)]
    pub fn si2c(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::SI2C)
    }
    #[doc = "NOT INSTALLED value."]
    #[inline(always)]
    pub fn na(self) -> &'a mut W {
        self.variant(SMOD1TYPE_A::NA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `SMOD1EN` reader - Submodule 1 enable (1) or disable (0)"]
pub struct SMOD1EN_R(crate::FieldReader<bool, bool>);
impl SMOD1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMOD1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMOD1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOD1EN` writer - Submodule 1 enable (1) or disable (0)"]
pub struct SMOD1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD1EN_W<'a> {
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
#[doc = "Submodule 0 module type. This is the SPI Master interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD0TYPE_A {
    #[doc = "0: MSPI submodule value."]
    SPI_MASTER = 0,
    #[doc = "1: I2C Master submodule value."]
    I2C_MASTER = 1,
    #[doc = "2: SPI Slave submodule value."]
    SSPI = 2,
    #[doc = "3: I2C Slave submodule value."]
    SI2C = 3,
    #[doc = "7: NOT INSTALLED value."]
    NA = 7,
}
impl From<SMOD0TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD0TYPE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMOD0TYPE` reader - Submodule 0 module type. This is the SPI Master interface."]
pub struct SMOD0TYPE_R(crate::FieldReader<u8, SMOD0TYPE_A>);
impl SMOD0TYPE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMOD0TYPE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD0TYPE_A> {
        match self.bits {
            0 => Some(SMOD0TYPE_A::SPI_MASTER),
            1 => Some(SMOD0TYPE_A::I2C_MASTER),
            2 => Some(SMOD0TYPE_A::SSPI),
            3 => Some(SMOD0TYPE_A::SI2C),
            7 => Some(SMOD0TYPE_A::NA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MASTER`"]
    #[inline(always)]
    pub fn is_spi_master(&self) -> bool {
        **self == SMOD0TYPE_A::SPI_MASTER
    }
    #[doc = "Checks if the value of the field is `I2C_MASTER`"]
    #[inline(always)]
    pub fn is_i2c_master(&self) -> bool {
        **self == SMOD0TYPE_A::I2C_MASTER
    }
    #[doc = "Checks if the value of the field is `SSPI`"]
    #[inline(always)]
    pub fn is_sspi(&self) -> bool {
        **self == SMOD0TYPE_A::SSPI
    }
    #[doc = "Checks if the value of the field is `SI2C`"]
    #[inline(always)]
    pub fn is_si2c(&self) -> bool {
        **self == SMOD0TYPE_A::SI2C
    }
    #[doc = "Checks if the value of the field is `NA`"]
    #[inline(always)]
    pub fn is_na(&self) -> bool {
        **self == SMOD0TYPE_A::NA
    }
}
impl core::ops::Deref for SMOD0TYPE_R {
    type Target = crate::FieldReader<u8, SMOD0TYPE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOD0TYPE` writer - Submodule 0 module type. This is the SPI Master interface."]
pub struct SMOD0TYPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD0TYPE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMOD0TYPE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "MSPI submodule value."]
    #[inline(always)]
    pub fn spi_master(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::SPI_MASTER)
    }
    #[doc = "I2C Master submodule value."]
    #[inline(always)]
    pub fn i2c_master(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::I2C_MASTER)
    }
    #[doc = "SPI Slave submodule value."]
    #[inline(always)]
    pub fn sspi(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::SSPI)
    }
    #[doc = "I2C Slave submodule value."]
    #[inline(always)]
    pub fn si2c(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::SI2C)
    }
    #[doc = "NOT INSTALLED value."]
    #[inline(always)]
    pub fn na(self) -> &'a mut W {
        self.variant(SMOD0TYPE_A::NA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "Field `SMOD0EN` reader - Submodule 0 enable (1) or disable (0)"]
pub struct SMOD0EN_R(crate::FieldReader<bool, bool>);
impl SMOD0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMOD0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMOD0EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMOD0EN` writer - Submodule 0 enable (1) or disable (0)"]
pub struct SMOD0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMOD0EN_W<'a> {
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
    #[doc = "Bits 5:7 - Submodule 0 module type. This is the I2C Master interface"]
    #[inline(always)]
    pub fn smod1type(&self) -> SMOD1TYPE_R {
        SMOD1TYPE_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod1en(&self) -> SMOD1EN_R {
        SMOD1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline(always)]
    pub fn smod0type(&self) -> SMOD0TYPE_R {
        SMOD0TYPE_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod0en(&self) -> SMOD0EN_R {
        SMOD0EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 5:7 - Submodule 0 module type. This is the I2C Master interface"]
    #[inline(always)]
    pub fn smod1type(&mut self) -> SMOD1TYPE_W {
        SMOD1TYPE_W { w: self }
    }
    #[doc = "Bit 4 - Submodule 1 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod1en(&mut self) -> SMOD1EN_W {
        SMOD1EN_W { w: self }
    }
    #[doc = "Bits 1:3 - Submodule 0 module type. This is the SPI Master interface."]
    #[inline(always)]
    pub fn smod0type(&mut self) -> SMOD0TYPE_W {
        SMOD0TYPE_W { w: self }
    }
    #[doc = "Bit 0 - Submodule 0 enable (1) or disable (0)"]
    #[inline(always)]
    pub fn smod0en(&mut self) -> SMOD0EN_W {
        SMOD0EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Submodule control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [submodctrl](index.html) module"]
pub struct SUBMODCTRL_SPEC;
impl crate::RegisterSpec for SUBMODCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [submodctrl::R](R) reader structure"]
impl crate::Readable for SUBMODCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [submodctrl::W](W) writer structure"]
impl crate::Writable for SUBMODCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUBMODCTRL to value 0x20"]
impl crate::Resettable for SUBMODCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}

#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "IOSLAVE interface enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCEN_A {
    #[doc = "0: Disable the IOSLAVE value."]
    DIS = 0,
    #[doc = "1: Enable the IOSLAVE value."]
    EN = 1,
}
impl From<IFCEN_A> for bool {
    #[inline(always)]
    fn from(variant: IFCEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCEN` reader - IOSLAVE interface enable."]
pub struct IFCEN_R(crate::FieldReader<bool, IFCEN_A>);
impl IFCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFCEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFCEN_A {
        match self.bits {
            false => IFCEN_A::DIS,
            true => IFCEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == IFCEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == IFCEN_A::EN
    }
}
impl core::ops::Deref for IFCEN_R {
    type Target = crate::FieldReader<bool, IFCEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFCEN` writer - IOSLAVE interface enable."]
pub struct IFCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFCEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the IOSLAVE value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(IFCEN_A::DIS)
    }
    #[doc = "Enable the IOSLAVE value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(IFCEN_A::EN)
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
#[doc = "Field `I2CADDR` reader - 7-bit or 10-bit I2C device address."]
pub struct I2CADDR_R(crate::FieldReader<u16, u16>);
impl I2CADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        I2CADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CADDR` writer - 7-bit or 10-bit I2C device address."]
pub struct I2CADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
#[doc = "This bit holds the cycle to initiate an I/O RAM read.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STARTRD_A {
    #[doc = "0: Initiate I/O RAM read late in each transferred byte. value."]
    LATE = 0,
    #[doc = "1: Initiate I/O RAM read early in each transferred byte. value."]
    EARLY = 1,
}
impl From<STARTRD_A> for bool {
    #[inline(always)]
    fn from(variant: STARTRD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STARTRD` reader - This bit holds the cycle to initiate an I/O RAM read."]
pub struct STARTRD_R(crate::FieldReader<bool, STARTRD_A>);
impl STARTRD_R {
    pub(crate) fn new(bits: bool) -> Self {
        STARTRD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTRD_A {
        match self.bits {
            false => STARTRD_A::LATE,
            true => STARTRD_A::EARLY,
        }
    }
    #[doc = "Checks if the value of the field is `LATE`"]
    #[inline(always)]
    pub fn is_late(&self) -> bool {
        **self == STARTRD_A::LATE
    }
    #[doc = "Checks if the value of the field is `EARLY`"]
    #[inline(always)]
    pub fn is_early(&self) -> bool {
        **self == STARTRD_A::EARLY
    }
}
impl core::ops::Deref for STARTRD_R {
    type Target = crate::FieldReader<bool, STARTRD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STARTRD` writer - This bit holds the cycle to initiate an I/O RAM read."]
pub struct STARTRD_W<'a> {
    w: &'a mut W,
}
impl<'a> STARTRD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STARTRD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Initiate I/O RAM read late in each transferred byte. value."]
    #[inline(always)]
    pub fn late(self) -> &'a mut W {
        self.variant(STARTRD_A::LATE)
    }
    #[doc = "Initiate I/O RAM read early in each transferred byte. value."]
    #[inline(always)]
    pub fn early(self) -> &'a mut W {
        self.variant(STARTRD_A::EARLY)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "This bit selects the transfer bit ordering.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LSB_A {
    #[doc = "0: Data is assumed to be sent and received with MSB first. value."]
    MSB_FIRST = 0,
    #[doc = "1: Data is assumed to be sent and received with LSB first. value."]
    LSB_FIRST = 1,
}
impl From<LSB_A> for bool {
    #[inline(always)]
    fn from(variant: LSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSB` reader - This bit selects the transfer bit ordering."]
pub struct LSB_R(crate::FieldReader<bool, LSB_A>);
impl LSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        LSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSB_A {
        match self.bits {
            false => LSB_A::MSB_FIRST,
            true => LSB_A::LSB_FIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSB_FIRST`"]
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        **self == LSB_A::MSB_FIRST
    }
    #[doc = "Checks if the value of the field is `LSB_FIRST`"]
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        **self == LSB_A::LSB_FIRST
    }
}
impl core::ops::Deref for LSB_R {
    type Target = crate::FieldReader<bool, LSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LSB` writer - This bit selects the transfer bit ordering."]
pub struct LSB_W<'a> {
    w: &'a mut W,
}
impl<'a> LSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Data is assumed to be sent and received with MSB first. value."]
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(LSB_A::MSB_FIRST)
    }
    #[doc = "Data is assumed to be sent and received with LSB first. value."]
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(LSB_A::LSB_FIRST)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "This bit selects SPI polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL_A {
    #[doc = "0: Polarity 0, handles SPI modes 0 and 3. value."]
    SPI_MODES_0_3 = 0,
    #[doc = "1: Polarity 1, handles SPI modes 1 and 2. value."]
    SPI_MODES_1_2 = 1,
}
impl From<SPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL` reader - This bit selects SPI polarity."]
pub struct SPOL_R(crate::FieldReader<bool, SPOL_A>);
impl SPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            false => SPOL_A::SPI_MODES_0_3,
            true => SPOL_A::SPI_MODES_1_2,
        }
    }
    #[doc = "Checks if the value of the field is `SPI_MODES_0_3`"]
    #[inline(always)]
    pub fn is_spi_modes_0_3(&self) -> bool {
        **self == SPOL_A::SPI_MODES_0_3
    }
    #[doc = "Checks if the value of the field is `SPI_MODES_1_2`"]
    #[inline(always)]
    pub fn is_spi_modes_1_2(&self) -> bool {
        **self == SPOL_A::SPI_MODES_1_2
    }
}
impl core::ops::Deref for SPOL_R {
    type Target = crate::FieldReader<bool, SPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPOL` writer - This bit selects SPI polarity."]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Polarity 0, handles SPI modes 0 and 3. value."]
    #[inline(always)]
    pub fn spi_modes_0_3(self) -> &'a mut W {
        self.variant(SPOL_A::SPI_MODES_0_3)
    }
    #[doc = "Polarity 1, handles SPI modes 1 and 2. value."]
    #[inline(always)]
    pub fn spi_modes_1_2(self) -> &'a mut W {
        self.variant(SPOL_A::SPI_MODES_1_2)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "This bit selects the I/O interface.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IFCSEL_A {
    #[doc = "0: Selects I2C interface for the IO Slave. value."]
    I2C = 0,
    #[doc = "1: Selects SPI interface for the IO Slave. value."]
    SPI = 1,
}
impl From<IFCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: IFCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IFCSEL` reader - This bit selects the I/O interface."]
pub struct IFCSEL_R(crate::FieldReader<bool, IFCSEL_A>);
impl IFCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        IFCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IFCSEL_A {
        match self.bits {
            false => IFCSEL_A::I2C,
            true => IFCSEL_A::SPI,
        }
    }
    #[doc = "Checks if the value of the field is `I2C`"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        **self == IFCSEL_A::I2C
    }
    #[doc = "Checks if the value of the field is `SPI`"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        **self == IFCSEL_A::SPI
    }
}
impl core::ops::Deref for IFCSEL_R {
    type Target = crate::FieldReader<bool, IFCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IFCSEL` writer - This bit selects the I/O interface."]
pub struct IFCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IFCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IFCSEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Selects I2C interface for the IO Slave. value."]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut W {
        self.variant(IFCSEL_A::I2C)
    }
    #[doc = "Selects SPI interface for the IO Slave. value."]
    #[inline(always)]
    pub fn spi(self) -> &'a mut W {
        self.variant(IFCSEL_A::SPI)
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - IOSLAVE interface enable."]
    #[inline(always)]
    pub fn ifcen(&self) -> IFCEN_R {
        IFCEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 8:19 - 7-bit or 10-bit I2C device address."]
    #[inline(always)]
    pub fn i2caddr(&self) -> I2CADDR_R {
        I2CADDR_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 4 - This bit holds the cycle to initiate an I/O RAM read."]
    #[inline(always)]
    pub fn startrd(&self) -> STARTRD_R {
        STARTRD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit selects the transfer bit ordering."]
    #[inline(always)]
    pub fn lsb(&self) -> LSB_R {
        LSB_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    pub fn ifcsel(&self) -> IFCSEL_R {
        IFCSEL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - IOSLAVE interface enable."]
    #[inline(always)]
    pub fn ifcen(&mut self) -> IFCEN_W {
        IFCEN_W { w: self }
    }
    #[doc = "Bits 8:19 - 7-bit or 10-bit I2C device address."]
    #[inline(always)]
    pub fn i2caddr(&mut self) -> I2CADDR_W {
        I2CADDR_W { w: self }
    }
    #[doc = "Bit 4 - This bit holds the cycle to initiate an I/O RAM read."]
    #[inline(always)]
    pub fn startrd(&mut self) -> STARTRD_W {
        STARTRD_W { w: self }
    }
    #[doc = "Bit 2 - This bit selects the transfer bit ordering."]
    #[inline(always)]
    pub fn lsb(&mut self) -> LSB_W {
        LSB_W { w: self }
    }
    #[doc = "Bit 1 - This bit selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Bit 0 - This bit selects the I/O interface."]
    #[inline(always)]
    pub fn ifcsel(&mut self) -> IFCSEL_W {
        IFCSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Slave Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

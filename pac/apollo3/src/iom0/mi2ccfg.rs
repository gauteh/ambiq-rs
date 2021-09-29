#[doc = "Register `MI2CCFG` reader"]
pub struct R(crate::R<MI2CCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MI2CCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MI2CCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MI2CCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MI2CCFG` writer"]
pub struct W(crate::W<MI2CCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MI2CCFG_SPEC>;
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
impl From<crate::W<MI2CCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MI2CCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRDIS` reader - Disable detection of clock stretch events smaller than 1 cycle"]
pub struct STRDIS_R(crate::FieldReader<bool, bool>);
impl STRDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        STRDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRDIS` writer - Disable detection of clock stretch events smaller than 1 cycle"]
pub struct STRDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> STRDIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `SMPCNT` reader - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
pub struct SMPCNT_R(crate::FieldReader<u8, u8>);
impl SMPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMPCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMPCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMPCNT` writer - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
pub struct SMPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SMPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `SDAENDLY` reader - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
pub struct SDAENDLY_R(crate::FieldReader<u8, u8>);
impl SDAENDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDAENDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDAENDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDAENDLY` writer - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
pub struct SDAENDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SDAENDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `SCLENDLY` reader - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
pub struct SCLENDLY_R(crate::FieldReader<u8, u8>);
impl SCLENDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SCLENDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCLENDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCLENDLY` writer - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
pub struct SCLENDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SCLENDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `MI2CRST` reader - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub struct MI2CRST_R(crate::FieldReader<bool, bool>);
impl MI2CRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MI2CRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MI2CRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MI2CRST` writer - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub struct MI2CRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MI2CRST_W<'a> {
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
#[doc = "Field `SDADLY` reader - Delay to enable on the SDA output. Values are 0x0-0x3."]
pub struct SDADLY_R(crate::FieldReader<u8, u8>);
impl SDADLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SDADLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDADLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDADLY` writer - Delay to enable on the SDA output. Values are 0x0-0x3."]
pub struct SDADLY_W<'a> {
    w: &'a mut W,
}
impl<'a> SDADLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ARBEN_A {
    #[doc = "1: Enable multi-master bus arbitration support for this i2c master value."]
    ARBEN = 1,
    #[doc = "0: Disable multi-master bus arbitration support for this i2c master value."]
    ARBDIS = 0,
}
impl From<ARBEN_A> for bool {
    #[inline(always)]
    fn from(variant: ARBEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARBEN` reader - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
pub struct ARBEN_R(crate::FieldReader<bool, ARBEN_A>);
impl ARBEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARBEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBEN_A {
        match self.bits {
            true => ARBEN_A::ARBEN,
            false => ARBEN_A::ARBDIS,
        }
    }
    #[doc = "Checks if the value of the field is `ARBEN`"]
    #[inline(always)]
    pub fn is_arben(&self) -> bool {
        **self == ARBEN_A::ARBEN
    }
    #[doc = "Checks if the value of the field is `ARBDIS`"]
    #[inline(always)]
    pub fn is_arbdis(&self) -> bool {
        **self == ARBEN_A::ARBDIS
    }
}
impl core::ops::Deref for ARBEN_R {
    type Target = crate::FieldReader<bool, ARBEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARBEN` writer - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
pub struct ARBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ARBEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ARBEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable multi-master bus arbitration support for this i2c master value."]
    #[inline(always)]
    pub fn arben(self) -> &'a mut W {
        self.variant(ARBEN_A::ARBEN)
    }
    #[doc = "Disable multi-master bus arbitration support for this i2c master value."]
    #[inline(always)]
    pub fn arbdis(self) -> &'a mut W {
        self.variant(ARBEN_A::ARBDIS)
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
#[doc = "Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2CLSB_A {
    #[doc = "0: Byte data is transmitted MSB first onto the bus/read from the bus value."]
    MSBFIRST = 0,
    #[doc = "1: Byte data is transmitted LSB first onto the bus/read from the bus value."]
    LSBFIRST = 1,
}
impl From<I2CLSB_A> for bool {
    #[inline(always)]
    fn from(variant: I2CLSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2CLSB` reader - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
pub struct I2CLSB_R(crate::FieldReader<bool, I2CLSB_A>);
impl I2CLSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2CLSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2CLSB_A {
        match self.bits {
            false => I2CLSB_A::MSBFIRST,
            true => I2CLSB_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        **self == I2CLSB_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        **self == I2CLSB_A::LSBFIRST
    }
}
impl core::ops::Deref for I2CLSB_R {
    type Target = crate::FieldReader<bool, I2CLSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2CLSB` writer - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
pub struct I2CLSB_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CLSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2CLSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Byte data is transmitted MSB first onto the bus/read from the bus value."]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(I2CLSB_A::MSBFIRST)
    }
    #[doc = "Byte data is transmitted LSB first onto the bus/read from the bus value."]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(I2CLSB_A::LSBFIRST)
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
#[doc = "Sets the I2C master device address size to either 7b (0) or 10b (1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADDRSZ_A {
    #[doc = "0: Use 7b addressing for I2C master transactions value."]
    ADDRSZ7 = 0,
    #[doc = "1: Use 10b addressing for I2C master transactions value."]
    ADDRSZ10 = 1,
}
impl From<ADDRSZ_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRSZ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRSZ` reader - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
pub struct ADDRSZ_R(crate::FieldReader<bool, ADDRSZ_A>);
impl ADDRSZ_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADDRSZ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRSZ_A {
        match self.bits {
            false => ADDRSZ_A::ADDRSZ7,
            true => ADDRSZ_A::ADDRSZ10,
        }
    }
    #[doc = "Checks if the value of the field is `ADDRSZ7`"]
    #[inline(always)]
    pub fn is_addrsz7(&self) -> bool {
        **self == ADDRSZ_A::ADDRSZ7
    }
    #[doc = "Checks if the value of the field is `ADDRSZ10`"]
    #[inline(always)]
    pub fn is_addrsz10(&self) -> bool {
        **self == ADDRSZ_A::ADDRSZ10
    }
}
impl core::ops::Deref for ADDRSZ_R {
    type Target = crate::FieldReader<bool, ADDRSZ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRSZ` writer - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
pub struct ADDRSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRSZ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADDRSZ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use 7b addressing for I2C master transactions value."]
    #[inline(always)]
    pub fn addrsz7(self) -> &'a mut W {
        self.variant(ADDRSZ_A::ADDRSZ7)
    }
    #[doc = "Use 10b addressing for I2C master transactions value."]
    #[inline(always)]
    pub fn addrsz10(self) -> &'a mut W {
        self.variant(ADDRSZ_A::ADDRSZ10)
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
    #[doc = "Bit 24 - Disable detection of clock stretch events smaller than 1 cycle"]
    #[inline(always)]
    pub fn strdis(&self) -> STRDIS_R {
        STRDIS_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
    #[inline(always)]
    pub fn smpcnt(&self) -> SMPCNT_R {
        SMPCNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 12:15 - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
    #[inline(always)]
    pub fn sdaendly(&self) -> SDAENDLY_R {
        SDAENDLY_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[inline(always)]
    pub fn sclendly(&self) -> SCLENDLY_R {
        SCLENDLY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    pub fn mi2crst(&self) -> MI2CRST_R {
        MI2CRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[inline(always)]
    pub fn sdadly(&self) -> SDADLY_R {
        SDADLY_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 2 - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
    #[inline(always)]
    pub fn arben(&self) -> ARBEN_R {
        ARBEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
    #[inline(always)]
    pub fn i2clsb(&self) -> I2CLSB_R {
        I2CLSB_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[inline(always)]
    pub fn addrsz(&self) -> ADDRSZ_R {
        ADDRSZ_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Disable detection of clock stretch events smaller than 1 cycle"]
    #[inline(always)]
    pub fn strdis(&mut self) -> STRDIS_W {
        STRDIS_W { w: self }
    }
    #[doc = "Bits 16:23 - Number of Base clk cycles to wait before sampling the SCL clock to determine if a clock stretch event has occured"]
    #[inline(always)]
    pub fn smpcnt(&mut self) -> SMPCNT_W {
        SMPCNT_W { w: self }
    }
    #[doc = "Bits 12:15 - Number of IOCLK cycles to delay the SDA output en (all transitions affected). Used to delay data relative to clock"]
    #[inline(always)]
    pub fn sdaendly(&mut self) -> SDAENDLY_W {
        SDAENDLY_W { w: self }
    }
    #[doc = "Bits 8:11 - Number of IOCLK cycles to delay the rising edge of the SCL output en (clock will go low on this edge). Used to allow clock shaping."]
    #[inline(always)]
    pub fn sclendly(&mut self) -> SCLENDLY_W {
        SCLENDLY_W { w: self }
    }
    #[doc = "Bit 6 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    pub fn mi2crst(&mut self) -> MI2CRST_W {
        MI2CRST_W { w: self }
    }
    #[doc = "Bits 4:5 - Delay to enable on the SDA output. Values are 0x0-0x3."]
    #[inline(always)]
    pub fn sdadly(&mut self) -> SDADLY_W {
        SDADLY_W { w: self }
    }
    #[doc = "Bit 2 - Enables multi-master arbitration for the I2C master. If the bus is known to have only a single master, this function can be disabled to save clock cycles on I2C transactions"]
    #[inline(always)]
    pub fn arben(&mut self) -> ARBEN_W {
        ARBEN_W { w: self }
    }
    #[doc = "Bit 1 - Direction of data transmit and receive, MSB(0) or LSB(1) first. Default per I2C specification is MSB first. This applies to both read and write data, and read data will be bit"]
    #[inline(always)]
    pub fn i2clsb(&mut self) -> I2CLSB_W {
        I2CLSB_W { w: self }
    }
    #[doc = "Bit 0 - Sets the I2C master device address size to either 7b (0) or 10b (1)."]
    #[inline(always)]
    pub fn addrsz(&mut self) -> ADDRSZ_W {
        ADDRSZ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mi2ccfg](index.html) module"]
pub struct MI2CCFG_SPEC;
impl crate::RegisterSpec for MI2CCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mi2ccfg::R](R) reader structure"]
impl crate::Readable for MI2CCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mi2ccfg::W](W) writer structure"]
impl crate::Writable for MI2CCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MI2CCFG to value 0"]
impl crate::Resettable for MI2CCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `MSPICFG` reader"]
pub struct R(crate::R<MSPICFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSPICFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSPICFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSPICFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSPICFG` writer"]
pub struct W(crate::W<MSPICFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSPICFG_SPEC>;
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
impl From<crate::W<MSPICFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSPICFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSPIRST` reader - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub struct MSPIRST_R(crate::FieldReader<bool, bool>);
impl MSPIRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        MSPIRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MSPIRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MSPIRST` writer - Not used. To reset the module, toggle the SMOD_EN for the module"]
pub struct MSPIRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MSPIRST_W<'a> {
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
#[doc = "Field `DOUTDLY` reader - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
pub struct DOUTDLY_R(crate::FieldReader<u8, u8>);
impl DOUTDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOUTDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DOUTDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOUTDLY` writer - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
pub struct DOUTDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DOUTDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
#[doc = "Field `DINDLY` reader - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
pub struct DINDLY_R(crate::FieldReader<u8, u8>);
impl DINDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DINDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DINDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DINDLY` writer - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
pub struct DINDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DINDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPILSB_A {
    #[doc = "0: Send and receive MSB bit first value."]
    MSB = 0,
    #[doc = "1: Send and receive LSB bit first value."]
    LSB = 1,
}
impl From<SPILSB_A> for bool {
    #[inline(always)]
    fn from(variant: SPILSB_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPILSB` reader - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
pub struct SPILSB_R(crate::FieldReader<bool, SPILSB_A>);
impl SPILSB_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPILSB_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPILSB_A {
        match self.bits {
            false => SPILSB_A::MSB,
            true => SPILSB_A::LSB,
        }
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        **self == SPILSB_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        **self == SPILSB_A::LSB
    }
}
impl core::ops::Deref for SPILSB_R {
    type Target = crate::FieldReader<bool, SPILSB_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPILSB` writer - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
pub struct SPILSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SPILSB_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPILSB_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Send and receive MSB bit first value."]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(SPILSB_A::MSB)
    }
    #[doc = "Send and receive LSB bit first value."]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(SPILSB_A::LSB)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "selects the read flow control signal polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFCPOL_A {
    #[doc = "0: Flow control signal high creates flow control. value."]
    HIGH = 0,
    #[doc = "1: Flow control signal low creates flow control. value."]
    LOW = 1,
}
impl From<RDFCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: RDFCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDFCPOL` reader - selects the read flow control signal polarity."]
pub struct RDFCPOL_R(crate::FieldReader<bool, RDFCPOL_A>);
impl RDFCPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDFCPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDFCPOL_A {
        match self.bits {
            false => RDFCPOL_A::HIGH,
            true => RDFCPOL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == RDFCPOL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == RDFCPOL_A::LOW
    }
}
impl core::ops::Deref for RDFCPOL_R {
    type Target = crate::FieldReader<bool, RDFCPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDFCPOL` writer - selects the read flow control signal polarity."]
pub struct RDFCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> RDFCPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDFCPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flow control signal high creates flow control. value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(RDFCPOL_A::HIGH)
    }
    #[doc = "Flow control signal low creates flow control. value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(RDFCPOL_A::LOW)
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
#[doc = "selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers).\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFCPOL_A {
    #[doc = "0: Flow control signal high(1) creates flow control and byte transfers will stop until the flow control signal goes low. value."]
    HIGH = 0,
    #[doc = "1: Flow control signal low(0) creates flow control and byte transfers will stop until the flow control signal goes high(1). value."]
    LOW = 1,
}
impl From<WTFCPOL_A> for bool {
    #[inline(always)]
    fn from(variant: WTFCPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTFCPOL` reader - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
pub struct WTFCPOL_R(crate::FieldReader<bool, WTFCPOL_A>);
impl WTFCPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WTFCPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTFCPOL_A {
        match self.bits {
            false => WTFCPOL_A::HIGH,
            true => WTFCPOL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == WTFCPOL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        **self == WTFCPOL_A::LOW
    }
}
impl core::ops::Deref for WTFCPOL_R {
    type Target = crate::FieldReader<bool, WTFCPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTFCPOL` writer - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
pub struct WTFCPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> WTFCPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTFCPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Flow control signal high(1) creates flow control and byte transfers will stop until the flow control signal goes low. value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WTFCPOL_A::HIGH)
    }
    #[doc = "Flow control signal low(0) creates flow control and byte transfers will stop until the flow control signal goes high(1). value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WTFCPOL_A::LOW)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "selects the write mode flow control signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFCIRQ_A {
    #[doc = "0: MISO is used as the write mode flow control signal. value."]
    MISO = 0,
    #[doc = "1: IRQ is used as the write mode flow control signal. value."]
    IRQ = 1,
}
impl From<WTFCIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: WTFCIRQ_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTFCIRQ` reader - selects the write mode flow control signal."]
pub struct WTFCIRQ_R(crate::FieldReader<bool, WTFCIRQ_A>);
impl WTFCIRQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        WTFCIRQ_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTFCIRQ_A {
        match self.bits {
            false => WTFCIRQ_A::MISO,
            true => WTFCIRQ_A::IRQ,
        }
    }
    #[doc = "Checks if the value of the field is `MISO`"]
    #[inline(always)]
    pub fn is_miso(&self) -> bool {
        **self == WTFCIRQ_A::MISO
    }
    #[doc = "Checks if the value of the field is `IRQ`"]
    #[inline(always)]
    pub fn is_irq(&self) -> bool {
        **self == WTFCIRQ_A::IRQ
    }
}
impl core::ops::Deref for WTFCIRQ_R {
    type Target = crate::FieldReader<bool, WTFCIRQ_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTFCIRQ` writer - selects the write mode flow control signal."]
pub struct WTFCIRQ_W<'a> {
    w: &'a mut W,
}
impl<'a> WTFCIRQ_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTFCIRQ_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MISO is used as the write mode flow control signal. value."]
    #[inline(always)]
    pub fn miso(self) -> &'a mut W {
        self.variant(WTFCIRQ_A::MISO)
    }
    #[doc = "IRQ is used as the write mode flow control signal. value."]
    #[inline(always)]
    pub fn irq(self) -> &'a mut W {
        self.variant(WTFCIRQ_A::IRQ)
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "inverts MOSI when flow control is enabled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MOSIINV_A {
    #[doc = "0: MOSI is set to 0 in read mode and 1 in write mode. value."]
    NORMAL = 0,
    #[doc = "1: MOSI is set to 1 in read mode and 0 in write mode. value."]
    INVERT = 1,
}
impl From<MOSIINV_A> for bool {
    #[inline(always)]
    fn from(variant: MOSIINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MOSIINV` reader - inverts MOSI when flow control is enabled."]
pub struct MOSIINV_R(crate::FieldReader<bool, MOSIINV_A>);
impl MOSIINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        MOSIINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOSIINV_A {
        match self.bits {
            false => MOSIINV_A::NORMAL,
            true => MOSIINV_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == MOSIINV_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == MOSIINV_A::INVERT
    }
}
impl core::ops::Deref for MOSIINV_R {
    type Target = crate::FieldReader<bool, MOSIINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MOSIINV` writer - inverts MOSI when flow control is enabled."]
pub struct MOSIINV_W<'a> {
    w: &'a mut W,
}
impl<'a> MOSIINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MOSIINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "MOSI is set to 0 in read mode and 1 in write mode. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MOSIINV_A::NORMAL)
    }
    #[doc = "MOSI is set to 1 in read mode and 0 in write mode. value."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(MOSIINV_A::INVERT)
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "enables read mode flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDFC_A {
    #[doc = "0: Read mode flow control disabled. value."]
    DIS = 0,
    #[doc = "1: Read mode flow control enabled. value."]
    EN = 1,
}
impl From<RDFC_A> for bool {
    #[inline(always)]
    fn from(variant: RDFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDFC` reader - enables read mode flow control."]
pub struct RDFC_R(crate::FieldReader<bool, RDFC_A>);
impl RDFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        RDFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDFC_A {
        match self.bits {
            false => RDFC_A::DIS,
            true => RDFC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == RDFC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == RDFC_A::EN
    }
}
impl core::ops::Deref for RDFC_R {
    type Target = crate::FieldReader<bool, RDFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RDFC` writer - enables read mode flow control."]
pub struct RDFC_W<'a> {
    w: &'a mut W,
}
impl<'a> RDFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Read mode flow control disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(RDFC_A::DIS)
    }
    #[doc = "Read mode flow control enabled. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(RDFC_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "enables write mode flow control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WTFC_A {
    #[doc = "0: Write mode flow control disabled. value."]
    DIS = 0,
    #[doc = "1: Write mode flow control enabled. value."]
    EN = 1,
}
impl From<WTFC_A> for bool {
    #[inline(always)]
    fn from(variant: WTFC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WTFC` reader - enables write mode flow control."]
pub struct WTFC_R(crate::FieldReader<bool, WTFC_A>);
impl WTFC_R {
    pub(crate) fn new(bits: bool) -> Self {
        WTFC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTFC_A {
        match self.bits {
            false => WTFC_A::DIS,
            true => WTFC_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == WTFC_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == WTFC_A::EN
    }
}
impl core::ops::Deref for WTFC_R {
    type Target = crate::FieldReader<bool, WTFC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WTFC` writer - enables write mode flow control."]
pub struct WTFC_W<'a> {
    w: &'a mut W,
}
impl<'a> WTFC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WTFC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Write mode flow control disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(WTFC_A::DIS)
    }
    #[doc = "Write mode flow control enabled. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(WTFC_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `FULLDUP` reader - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
pub struct FULLDUP_R(crate::FieldReader<bool, bool>);
impl FULLDUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FULLDUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FULLDUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FULLDUP` writer - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
pub struct FULLDUP_W<'a> {
    w: &'a mut W,
}
impl<'a> FULLDUP_W<'a> {
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
#[doc = "selects SPI phase.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPHA_A {
    #[doc = "0: Sample on the leading (first) clock edge. value."]
    SAMPLE_LEADING_EDGE = 0,
    #[doc = "1: Sample on the trailing (second) clock edge. value."]
    SAMPLE_TRAILING_EDGE = 1,
}
impl From<SPHA_A> for bool {
    #[inline(always)]
    fn from(variant: SPHA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPHA` reader - selects SPI phase."]
pub struct SPHA_R(crate::FieldReader<bool, SPHA_A>);
impl SPHA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPHA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPHA_A {
        match self.bits {
            false => SPHA_A::SAMPLE_LEADING_EDGE,
            true => SPHA_A::SAMPLE_TRAILING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE_LEADING_EDGE`"]
    #[inline(always)]
    pub fn is_sample_leading_edge(&self) -> bool {
        **self == SPHA_A::SAMPLE_LEADING_EDGE
    }
    #[doc = "Checks if the value of the field is `SAMPLE_TRAILING_EDGE`"]
    #[inline(always)]
    pub fn is_sample_trailing_edge(&self) -> bool {
        **self == SPHA_A::SAMPLE_TRAILING_EDGE
    }
}
impl core::ops::Deref for SPHA_R {
    type Target = crate::FieldReader<bool, SPHA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPHA` writer - selects SPI phase."]
pub struct SPHA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPHA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPHA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Sample on the leading (first) clock edge. value."]
    #[inline(always)]
    pub fn sample_leading_edge(self) -> &'a mut W {
        self.variant(SPHA_A::SAMPLE_LEADING_EDGE)
    }
    #[doc = "Sample on the trailing (second) clock edge. value."]
    #[inline(always)]
    pub fn sample_trailing_edge(self) -> &'a mut W {
        self.variant(SPHA_A::SAMPLE_TRAILING_EDGE)
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
#[doc = "selects SPI polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPOL_A {
    #[doc = "0: The base value of the clock is 0. value."]
    CLK_BASE_0 = 0,
    #[doc = "1: The base value of the clock is 1. value."]
    CLK_BASE_1 = 1,
}
impl From<SPOL_A> for bool {
    #[inline(always)]
    fn from(variant: SPOL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SPOL` reader - selects SPI polarity."]
pub struct SPOL_R(crate::FieldReader<bool, SPOL_A>);
impl SPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPOL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOL_A {
        match self.bits {
            false => SPOL_A::CLK_BASE_0,
            true => SPOL_A::CLK_BASE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_0`"]
    #[inline(always)]
    pub fn is_clk_base_0(&self) -> bool {
        **self == SPOL_A::CLK_BASE_0
    }
    #[doc = "Checks if the value of the field is `CLK_BASE_1`"]
    #[inline(always)]
    pub fn is_clk_base_1(&self) -> bool {
        **self == SPOL_A::CLK_BASE_1
    }
}
impl core::ops::Deref for SPOL_R {
    type Target = crate::FieldReader<bool, SPOL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPOL` writer - selects SPI polarity."]
pub struct SPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPOL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPOL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The base value of the clock is 0. value."]
    #[inline(always)]
    pub fn clk_base_0(self) -> &'a mut W {
        self.variant(SPOL_A::CLK_BASE_0)
    }
    #[doc = "The base value of the clock is 1. value."]
    #[inline(always)]
    pub fn clk_base_1(self) -> &'a mut W {
        self.variant(SPOL_A::CLK_BASE_1)
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
    #[doc = "Bit 30 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    pub fn mspirst(&self) -> MSPIRST_R {
        MSPIRST_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 27:29 - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
    #[inline(always)]
    pub fn doutdly(&self) -> DOUTDLY_R {
        DOUTDLY_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[inline(always)]
    pub fn dindly(&self) -> DINDLY_R {
        DINDLY_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bit 23 - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline(always)]
    pub fn spilsb(&self) -> SPILSB_R {
        SPILSB_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - selects the read flow control signal polarity."]
    #[inline(always)]
    pub fn rdfcpol(&self) -> RDFCPOL_R {
        RDFCPOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
    #[inline(always)]
    pub fn wtfcpol(&self) -> WTFCPOL_R {
        WTFCPOL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - selects the write mode flow control signal."]
    #[inline(always)]
    pub fn wtfcirq(&self) -> WTFCIRQ_R {
        WTFCIRQ_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 18 - inverts MOSI when flow control is enabled."]
    #[inline(always)]
    pub fn mosiinv(&self) -> MOSIINV_R {
        MOSIINV_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - enables read mode flow control."]
    #[inline(always)]
    pub fn rdfc(&self) -> RDFC_R {
        RDFC_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - enables write mode flow control."]
    #[inline(always)]
    pub fn wtfc(&self) -> WTFC_R {
        WTFC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
    #[inline(always)]
    pub fn fulldup(&self) -> FULLDUP_R {
        FULLDUP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - selects SPI phase."]
    #[inline(always)]
    pub fn spha(&self) -> SPHA_R {
        SPHA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Not used. To reset the module, toggle the SMOD_EN for the module"]
    #[inline(always)]
    pub fn mspirst(&mut self) -> MSPIRST_W {
        MSPIRST_W { w: self }
    }
    #[doc = "Bits 27:29 - Delay tap to use for the output signal (MOSI). This give more hold time on the output data"]
    #[inline(always)]
    pub fn doutdly(&mut self) -> DOUTDLY_W {
        DOUTDLY_W { w: self }
    }
    #[doc = "Bits 24:26 - Delay tap to use for the input signal (MISO). This gives more hold time on the input data."]
    #[inline(always)]
    pub fn dindly(&mut self) -> DINDLY_W {
        DINDLY_W { w: self }
    }
    #[doc = "Bit 23 - Selects data transfer as MSB first (0) or LSB first (1) for the data portion of the SPI transaction. The offset bytes are always transmitted MSB first."]
    #[inline(always)]
    pub fn spilsb(&mut self) -> SPILSB_W {
        SPILSB_W { w: self }
    }
    #[doc = "Bit 22 - selects the read flow control signal polarity."]
    #[inline(always)]
    pub fn rdfcpol(&mut self) -> RDFCPOL_W {
        RDFCPOL_W { w: self }
    }
    #[doc = "Bit 21 - selects the write flow control signal polarity. The transfers are halted when the selected flow control signal is OPPOSITE polarity of bit. (For example: WTFCPOL = 0 will allow a IRQ=1 to pause transfers)."]
    #[inline(always)]
    pub fn wtfcpol(&mut self) -> WTFCPOL_W {
        WTFCPOL_W { w: self }
    }
    #[doc = "Bit 20 - selects the write mode flow control signal."]
    #[inline(always)]
    pub fn wtfcirq(&mut self) -> WTFCIRQ_W {
        WTFCIRQ_W { w: self }
    }
    #[doc = "Bit 18 - inverts MOSI when flow control is enabled."]
    #[inline(always)]
    pub fn mosiinv(&mut self) -> MOSIINV_W {
        MOSIINV_W { w: self }
    }
    #[doc = "Bit 17 - enables read mode flow control."]
    #[inline(always)]
    pub fn rdfc(&mut self) -> RDFC_W {
        RDFC_W { w: self }
    }
    #[doc = "Bit 16 - enables write mode flow control."]
    #[inline(always)]
    pub fn wtfc(&mut self) -> WTFC_W {
        WTFC_W { w: self }
    }
    #[doc = "Bit 2 - Enables full duplex mode for Master SPI write operations. Data will be captured simultaneously into the read fifo"]
    #[inline(always)]
    pub fn fulldup(&mut self) -> FULLDUP_W {
        FULLDUP_W { w: self }
    }
    #[doc = "Bit 1 - selects SPI phase."]
    #[inline(always)]
    pub fn spha(&mut self) -> SPHA_W {
        SPHA_W { w: self }
    }
    #[doc = "Bit 0 - selects SPI polarity."]
    #[inline(always)]
    pub fn spol(&mut self) -> SPOL_W {
        SPOL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI module master configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspicfg](index.html) module"]
pub struct MSPICFG_SPEC;
impl crate::RegisterSpec for MSPICFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mspicfg::R](R) reader structure"]
impl crate::Readable for MSPICFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mspicfg::W](W) writer structure"]
impl crate::Writable for MSPICFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSPICFG to value 0x0020_0000"]
impl crate::Resettable for MSPICFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0020_0000
    }
}

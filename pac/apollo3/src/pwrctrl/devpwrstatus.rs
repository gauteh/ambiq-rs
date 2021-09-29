#[doc = "Register `DEVPWRSTATUS` reader"]
pub struct R(crate::R<DEVPWRSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEVPWRSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEVPWRSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEVPWRSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEVPWRSTATUS` writer"]
pub struct W(crate::W<DEVPWRSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEVPWRSTATUS_SPEC>;
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
impl From<crate::W<DEVPWRSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEVPWRSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLEH` reader - This bit is 1 if power is supplied to BLEH"]
pub struct BLEH_R(crate::FieldReader<bool, bool>);
impl BLEH_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEH` writer - This bit is 1 if power is supplied to BLEH"]
pub struct BLEH_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEH_W<'a> {
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
#[doc = "Field `BLEL` reader - This bit is 1 if power is supplied to BLEL"]
pub struct BLEL_R(crate::FieldReader<bool, bool>);
impl BLEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLEL` writer - This bit is 1 if power is supplied to BLEL"]
pub struct BLEL_W<'a> {
    w: &'a mut W,
}
impl<'a> BLEL_W<'a> {
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
#[doc = "Field `PWRPDM` reader - This bit is 1 if power is supplied to PDM"]
pub struct PWRPDM_R(crate::FieldReader<bool, bool>);
impl PWRPDM_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRPDM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRPDM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRPDM` writer - This bit is 1 if power is supplied to PDM"]
pub struct PWRPDM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRPDM_W<'a> {
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
#[doc = "Field `PWRMSPI` reader - This bit is 1 if power is supplied to MSPI"]
pub struct PWRMSPI_R(crate::FieldReader<bool, bool>);
impl PWRMSPI_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRMSPI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRMSPI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRMSPI` writer - This bit is 1 if power is supplied to MSPI"]
pub struct PWRMSPI_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRMSPI_W<'a> {
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
#[doc = "Field `PWRADC` reader - This bit is 1 if power is supplied to ADC"]
pub struct PWRADC_R(crate::FieldReader<bool, bool>);
impl PWRADC_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRADC` writer - This bit is 1 if power is supplied to ADC"]
pub struct PWRADC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRADC_W<'a> {
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
#[doc = "Field `HCPC` reader - This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)"]
pub struct HCPC_R(crate::FieldReader<bool, bool>);
impl HCPC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCPC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCPC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCPC` writer - This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)"]
pub struct HCPC_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPC_W<'a> {
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
#[doc = "Field `HCPB` reader - This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)"]
pub struct HCPB_R(crate::FieldReader<bool, bool>);
impl HCPB_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCPB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCPB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCPB` writer - This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)"]
pub struct HCPB_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPB_W<'a> {
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
#[doc = "Field `HCPA` reader - This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)"]
pub struct HCPA_R(crate::FieldReader<bool, bool>);
impl HCPA_R {
    pub(crate) fn new(bits: bool) -> Self {
        HCPA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HCPA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HCPA` writer - This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)"]
pub struct HCPA_W<'a> {
    w: &'a mut W,
}
impl<'a> HCPA_W<'a> {
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
#[doc = "Field `MCUH` reader - This bit is 1 if power is supplied to MCUH"]
pub struct MCUH_R(crate::FieldReader<bool, bool>);
impl MCUH_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCUH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCUH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCUH` writer - This bit is 1 if power is supplied to MCUH"]
pub struct MCUH_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUH_W<'a> {
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
#[doc = "Field `MCUL` reader - This bit is 1 if power is supplied to MCUL"]
pub struct MCUL_R(crate::FieldReader<bool, bool>);
impl MCUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCUL` writer - This bit is 1 if power is supplied to MCUL"]
pub struct MCUL_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUL_W<'a> {
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
    #[doc = "Bit 9 - This bit is 1 if power is supplied to BLEH"]
    #[inline(always)]
    pub fn bleh(&self) -> BLEH_R {
        BLEH_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to BLEL"]
    #[inline(always)]
    pub fn blel(&self) -> BLEL_R {
        BLEL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to PDM"]
    #[inline(always)]
    pub fn pwrpdm(&self) -> PWRPDM_R {
        PWRPDM_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to MSPI"]
    #[inline(always)]
    pub fn pwrmspi(&self) -> PWRMSPI_R {
        PWRMSPI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to ADC"]
    #[inline(always)]
    pub fn pwradc(&self) -> PWRADC_R {
        PWRADC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)"]
    #[inline(always)]
    pub fn hcpc(&self) -> HCPC_R {
        HCPC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)"]
    #[inline(always)]
    pub fn hcpb(&self) -> HCPB_R {
        HCPB_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)"]
    #[inline(always)]
    pub fn hcpa(&self) -> HCPA_R {
        HCPA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to MCUH"]
    #[inline(always)]
    pub fn mcuh(&self) -> MCUH_R {
        MCUH_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to MCUL"]
    #[inline(always)]
    pub fn mcul(&self) -> MCUL_R {
        MCUL_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - This bit is 1 if power is supplied to BLEH"]
    #[inline(always)]
    pub fn bleh(&mut self) -> BLEH_W {
        BLEH_W { w: self }
    }
    #[doc = "Bit 8 - This bit is 1 if power is supplied to BLEL"]
    #[inline(always)]
    pub fn blel(&mut self) -> BLEL_W {
        BLEL_W { w: self }
    }
    #[doc = "Bit 7 - This bit is 1 if power is supplied to PDM"]
    #[inline(always)]
    pub fn pwrpdm(&mut self) -> PWRPDM_W {
        PWRPDM_W { w: self }
    }
    #[doc = "Bit 6 - This bit is 1 if power is supplied to MSPI"]
    #[inline(always)]
    pub fn pwrmspi(&mut self) -> PWRMSPI_W {
        PWRMSPI_W { w: self }
    }
    #[doc = "Bit 5 - This bit is 1 if power is supplied to ADC"]
    #[inline(always)]
    pub fn pwradc(&mut self) -> PWRADC_W {
        PWRADC_W { w: self }
    }
    #[doc = "Bit 4 - This bit is 1 if power is supplied to HCPC domain (IO MASTER4, 5, 6)"]
    #[inline(always)]
    pub fn hcpc(&mut self) -> HCPC_W {
        HCPC_W { w: self }
    }
    #[doc = "Bit 3 - This bit is 1 if power is supplied to HCPB domain (IO MASTER 0, 1, 2)"]
    #[inline(always)]
    pub fn hcpb(&mut self) -> HCPB_W {
        HCPB_W { w: self }
    }
    #[doc = "Bit 2 - This bit is 1 if power is supplied to HCPA domain (IO SLAVE, UART0, UART1, SCARD)"]
    #[inline(always)]
    pub fn hcpa(&mut self) -> HCPA_W {
        HCPA_W { w: self }
    }
    #[doc = "Bit 1 - This bit is 1 if power is supplied to MCUH"]
    #[inline(always)]
    pub fn mcuh(&mut self) -> MCUH_W {
        MCUH_W { w: self }
    }
    #[doc = "Bit 0 - This bit is 1 if power is supplied to MCUL"]
    #[inline(always)]
    pub fn mcul(&mut self) -> MCUL_W {
        MCUL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Power ON Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [devpwrstatus](index.html) module"]
pub struct DEVPWRSTATUS_SPEC;
impl crate::RegisterSpec for DEVPWRSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [devpwrstatus::R](R) reader structure"]
impl crate::Readable for DEVPWRSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [devpwrstatus::W](W) writer structure"]
impl crate::Writable for DEVPWRSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEVPWRSTATUS to value 0x03"]
impl crate::Resettable for DEVPWRSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}

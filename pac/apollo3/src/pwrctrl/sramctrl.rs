#[doc = "Register `SRAMCTRL` reader"]
pub struct R(crate::R<SRAMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMCTRL` writer"]
pub struct W(crate::W<SRAMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMCTRL_SPEC>;
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
impl From<crate::W<SRAMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum SRAMLIGHTSLEEP_A {
    #[doc = "255: Enable LIGHT SLEEP for ALL SRAMs value."]
    ALL = 255,
    #[doc = "0: Disables LIGHT SLEEP for ALL SRAMs value."]
    DIS = 0,
}
impl From<SRAMLIGHTSLEEP_A> for u16 {
    #[inline(always)]
    fn from(variant: SRAMLIGHTSLEEP_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SRAMLIGHTSLEEP` reader - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
pub struct SRAMLIGHTSLEEP_R(crate::FieldReader<u16, SRAMLIGHTSLEEP_A>);
impl SRAMLIGHTSLEEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        SRAMLIGHTSLEEP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRAMLIGHTSLEEP_A> {
        match self.bits {
            255 => Some(SRAMLIGHTSLEEP_A::ALL),
            0 => Some(SRAMLIGHTSLEEP_A::DIS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        **self == SRAMLIGHTSLEEP_A::ALL
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SRAMLIGHTSLEEP_A::DIS
    }
}
impl core::ops::Deref for SRAMLIGHTSLEEP_R {
    type Target = crate::FieldReader<u16, SRAMLIGHTSLEEP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMLIGHTSLEEP` writer - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
pub struct SRAMLIGHTSLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMLIGHTSLEEP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMLIGHTSLEEP_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Enable LIGHT SLEEP for ALL SRAMs value."]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(SRAMLIGHTSLEEP_A::ALL)
    }
    #[doc = "Disables LIGHT SLEEP for ALL SRAMs value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAMLIGHTSLEEP_A::DIS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
#[doc = "This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMMASTERCLKGATE_A {
    #[doc = "1: Enable Master SRAM Clock Gate value."]
    EN = 1,
    #[doc = "0: Disables Master SRAM Clock Gating value."]
    DIS = 0,
}
impl From<SRAMMASTERCLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMMASTERCLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMMASTERCLKGATE` reader - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
pub struct SRAMMASTERCLKGATE_R(crate::FieldReader<bool, SRAMMASTERCLKGATE_A>);
impl SRAMMASTERCLKGATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAMMASTERCLKGATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMMASTERCLKGATE_A {
        match self.bits {
            true => SRAMMASTERCLKGATE_A::EN,
            false => SRAMMASTERCLKGATE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SRAMMASTERCLKGATE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SRAMMASTERCLKGATE_A::DIS
    }
}
impl core::ops::Deref for SRAMMASTERCLKGATE_R {
    type Target = crate::FieldReader<bool, SRAMMASTERCLKGATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMMASTERCLKGATE` writer - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
pub struct SRAMMASTERCLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMMASTERCLKGATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMMASTERCLKGATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Master SRAM Clock Gate value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SRAMMASTERCLKGATE_A::EN)
    }
    #[doc = "Disables Master SRAM Clock Gating value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAMMASTERCLKGATE_A::DIS)
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
#[doc = "This bit is 1 if clock gating is allowed for individual system SRAMs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRAMCLKGATE_A {
    #[doc = "1: Enable Individual SRAM Clock Gating value."]
    EN = 1,
    #[doc = "0: Disables Individual SRAM Clock Gating value."]
    DIS = 0,
}
impl From<SRAMCLKGATE_A> for bool {
    #[inline(always)]
    fn from(variant: SRAMCLKGATE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRAMCLKGATE` reader - This bit is 1 if clock gating is allowed for individual system SRAMs"]
pub struct SRAMCLKGATE_R(crate::FieldReader<bool, SRAMCLKGATE_A>);
impl SRAMCLKGATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRAMCLKGATE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRAMCLKGATE_A {
        match self.bits {
            true => SRAMCLKGATE_A::EN,
            false => SRAMCLKGATE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SRAMCLKGATE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SRAMCLKGATE_A::DIS
    }
}
impl core::ops::Deref for SRAMCLKGATE_R {
    type Target = crate::FieldReader<bool, SRAMCLKGATE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAMCLKGATE` writer - This bit is 1 if clock gating is allowed for individual system SRAMs"]
pub struct SRAMCLKGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAMCLKGATE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRAMCLKGATE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Individual SRAM Clock Gating value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SRAMCLKGATE_A::EN)
    }
    #[doc = "Disables Individual SRAM Clock Gating value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SRAMCLKGATE_A::DIS)
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
impl R {
    #[doc = "Bits 8:19 - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[inline(always)]
    pub fn sramlightsleep(&self) -> SRAMLIGHTSLEEP_R {
        SRAMLIGHTSLEEP_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bit 2 - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
    #[inline(always)]
    pub fn srammasterclkgate(&self) -> SRAMMASTERCLKGATE_R {
        SRAMMASTERCLKGATE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if clock gating is allowed for individual system SRAMs"]
    #[inline(always)]
    pub fn sramclkgate(&self) -> SRAMCLKGATE_R {
        SRAMCLKGATE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:19 - Light Sleep enable for each TCM/SRAM bank. When 1, corresponding bank will be put into light sleep. For optimal power, banks should be put into light sleep while the system is active but the bank has minimal or no accesses."]
    #[inline(always)]
    pub fn sramlightsleep(&mut self) -> SRAMLIGHTSLEEP_W {
        SRAMLIGHTSLEEP_W { w: self }
    }
    #[doc = "Bit 2 - This bit is 1 when the master clock gate is enabled (top-level clock gate for entire SRAM block)"]
    #[inline(always)]
    pub fn srammasterclkgate(&mut self) -> SRAMMASTERCLKGATE_W {
        SRAMMASTERCLKGATE_W { w: self }
    }
    #[doc = "Bit 1 - This bit is 1 if clock gating is allowed for individual system SRAMs"]
    #[inline(always)]
    pub fn sramclkgate(&mut self) -> SRAMCLKGATE_W {
        SRAMCLKGATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramctrl](index.html) module"]
pub struct SRAMCTRL_SPEC;
impl crate::RegisterSpec for SRAMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramctrl::R](R) reader structure"]
impl crate::Readable for SRAMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramctrl::W](W) writer structure"]
impl crate::Writable for SRAMCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRAMCTRL to value 0"]
impl crate::Resettable for SRAMCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `CMDSTAT` reader"]
pub struct R(crate::R<CMDSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMDSTAT` writer"]
pub struct W(crate::W<CMDSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMDSTAT_SPEC>;
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
impl From<crate::W<CMDSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMDSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTSIZE` reader - The current number of bytes still to be transferred with this command. This field will count down to zero."]
pub struct CTSIZE_R(crate::FieldReader<u16, u16>);
impl CTSIZE_R {
    pub(crate) fn new(bits: u16) -> Self {
        CTSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSIZE_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSIZE` writer - The current number of bytes still to be transferred with this command. This field will count down to zero."]
pub struct CTSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
#[doc = "The current status of the command execution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CMDSTAT_A {
    #[doc = "1: Error encountered with command value."]
    ERR = 1,
    #[doc = "2: Actively processing command value."]
    ACTIVE = 2,
    #[doc = "4: Idle state, no active command, no error value."]
    IDLE = 4,
    #[doc = "6: Command in progress, but waiting on data from host value."]
    WAIT = 6,
}
impl From<CMDSTAT_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDSTAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CMDSTAT` reader - The current status of the command execution."]
pub struct CMDSTAT_R(crate::FieldReader<u8, CMDSTAT_A>);
impl CMDSTAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CMDSTAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMDSTAT_A> {
        match self.bits {
            1 => Some(CMDSTAT_A::ERR),
            2 => Some(CMDSTAT_A::ACTIVE),
            4 => Some(CMDSTAT_A::IDLE),
            6 => Some(CMDSTAT_A::WAIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ERR`"]
    #[inline(always)]
    pub fn is_err(&self) -> bool {
        **self == CMDSTAT_A::ERR
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        **self == CMDSTAT_A::ACTIVE
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        **self == CMDSTAT_A::IDLE
    }
    #[doc = "Checks if the value of the field is `WAIT`"]
    #[inline(always)]
    pub fn is_wait(&self) -> bool {
        **self == CMDSTAT_A::WAIT
    }
}
impl core::ops::Deref for CMDSTAT_R {
    type Target = crate::FieldReader<u8, CMDSTAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMDSTAT` writer - The current status of the command execution."]
pub struct CMDSTAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CMDSTAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CMDSTAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Error encountered with command value."]
    #[inline(always)]
    pub fn err(self) -> &'a mut W {
        self.variant(CMDSTAT_A::ERR)
    }
    #[doc = "Actively processing command value."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(CMDSTAT_A::ACTIVE)
    }
    #[doc = "Idle state, no active command, no error value."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(CMDSTAT_A::IDLE)
    }
    #[doc = "Command in progress, but waiting on data from host value."]
    #[inline(always)]
    pub fn wait(self) -> &'a mut W {
        self.variant(CMDSTAT_A::WAIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
#[doc = "Field `CCMD` reader - current command that is being executed"]
pub struct CCMD_R(crate::FieldReader<u8, u8>);
impl CCMD_R {
    pub(crate) fn new(bits: u8) -> Self {
        CCMD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCMD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCMD` writer - current command that is being executed"]
pub struct CCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CCMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline(always)]
    pub fn ctsize(&self) -> CTSIZE_R {
        CTSIZE_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline(always)]
    pub fn cmdstat(&self) -> CMDSTAT_R {
        CMDSTAT_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline(always)]
    pub fn ccmd(&self) -> CCMD_R {
        CCMD_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:19 - The current number of bytes still to be transferred with this command. This field will count down to zero."]
    #[inline(always)]
    pub fn ctsize(&mut self) -> CTSIZE_W {
        CTSIZE_W { w: self }
    }
    #[doc = "Bits 5:7 - The current status of the command execution."]
    #[inline(always)]
    pub fn cmdstat(&mut self) -> CMDSTAT_W {
        CMDSTAT_W { w: self }
    }
    #[doc = "Bits 0:4 - current command that is being executed"]
    #[inline(always)]
    pub fn ccmd(&mut self) -> CCMD_W {
        CCMD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdstat](index.html) module"]
pub struct CMDSTAT_SPEC;
impl crate::RegisterSpec for CMDSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdstat::R](R) reader structure"]
impl crate::Readable for CMDSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmdstat::W](W) writer structure"]
impl crate::Writable for CMDSTAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMDSTAT to value 0"]
impl crate::Resettable for CMDSTAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `FAULTSTATUS` reader"]
pub struct R(crate::R<FAULTSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FAULTSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FAULTSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FAULTSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FAULTSTATUS` writer"]
pub struct W(crate::W<FAULTSTATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FAULTSTATUS_SPEC>;
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
impl From<crate::W<FAULTSTATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FAULTSTATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSFAULT_A {
    #[doc = "0: No bus fault has been detected. value."]
    NOFAULT = 0,
    #[doc = "1: Bus fault detected. value."]
    FAULT = 1,
}
impl From<SYSFAULT_A> for bool {
    #[inline(always)]
    fn from(variant: SYSFAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSFAULT` reader - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
pub struct SYSFAULT_R(crate::FieldReader<bool, SYSFAULT_A>);
impl SYSFAULT_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSFAULT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYSFAULT_A {
        match self.bits {
            false => SYSFAULT_A::NOFAULT,
            true => SYSFAULT_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        **self == SYSFAULT_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        **self == SYSFAULT_A::FAULT
    }
}
impl core::ops::Deref for SYSFAULT_R {
    type Target = crate::FieldReader<bool, SYSFAULT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSFAULT` writer - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
pub struct SYSFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSFAULT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No bus fault has been detected. value."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(SYSFAULT_A::NOFAULT)
    }
    #[doc = "Bus fault detected. value."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(SYSFAULT_A::FAULT)
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
#[doc = "DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DCODEFAULT_A {
    #[doc = "0: No DCODE fault has been detected. value."]
    NOFAULT = 0,
    #[doc = "1: DCODE fault detected. value."]
    FAULT = 1,
}
impl From<DCODEFAULT_A> for bool {
    #[inline(always)]
    fn from(variant: DCODEFAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCODEFAULT` reader - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
pub struct DCODEFAULT_R(crate::FieldReader<bool, DCODEFAULT_A>);
impl DCODEFAULT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCODEFAULT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCODEFAULT_A {
        match self.bits {
            false => DCODEFAULT_A::NOFAULT,
            true => DCODEFAULT_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        **self == DCODEFAULT_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        **self == DCODEFAULT_A::FAULT
    }
}
impl core::ops::Deref for DCODEFAULT_R {
    type Target = crate::FieldReader<bool, DCODEFAULT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCODEFAULT` writer - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
pub struct DCODEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> DCODEFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DCODEFAULT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No DCODE fault has been detected. value."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(DCODEFAULT_A::NOFAULT)
    }
    #[doc = "DCODE fault detected. value."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(DCODEFAULT_A::FAULT)
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
#[doc = "The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ICODEFAULT_A {
    #[doc = "0: No ICODE fault has been detected. value."]
    NOFAULT = 0,
    #[doc = "1: ICODE fault detected. value."]
    FAULT = 1,
}
impl From<ICODEFAULT_A> for bool {
    #[inline(always)]
    fn from(variant: ICODEFAULT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICODEFAULT` reader - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
pub struct ICODEFAULT_R(crate::FieldReader<bool, ICODEFAULT_A>);
impl ICODEFAULT_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICODEFAULT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICODEFAULT_A {
        match self.bits {
            false => ICODEFAULT_A::NOFAULT,
            true => ICODEFAULT_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NOFAULT`"]
    #[inline(always)]
    pub fn is_nofault(&self) -> bool {
        **self == ICODEFAULT_A::NOFAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        **self == ICODEFAULT_A::FAULT
    }
}
impl core::ops::Deref for ICODEFAULT_R {
    type Target = crate::FieldReader<bool, ICODEFAULT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICODEFAULT` writer - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
pub struct ICODEFAULT_W<'a> {
    w: &'a mut W,
}
impl<'a> ICODEFAULT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ICODEFAULT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No ICODE fault has been detected. value."]
    #[inline(always)]
    pub fn nofault(self) -> &'a mut W {
        self.variant(ICODEFAULT_A::NOFAULT)
    }
    #[doc = "ICODE fault detected. value."]
    #[inline(always)]
    pub fn fault(self) -> &'a mut W {
        self.variant(ICODEFAULT_A::FAULT)
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
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn sysfault(&self) -> SYSFAULT_R {
        SYSFAULT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn dcodefault(&self) -> DCODEFAULT_R {
        DCODEFAULT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn icodefault(&self) -> ICODEFAULT_R {
        ICODEFAULT_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SYS Bus Decoder Fault Detected bit. When set, a fault has been detected, and the SYSFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn sysfault(&mut self) -> SYSFAULT_W {
        SYSFAULT_W { w: self }
    }
    #[doc = "Bit 1 - DCODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the DCODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn dcodefault(&mut self) -> DCODEFAULT_W {
        DCODEFAULT_W { w: self }
    }
    #[doc = "Bit 0 - The ICODE Bus Decoder Fault Detected bit. When set, a fault has been detected, and the ICODEFAULTADDR register will contain the bus address which generated the fault."]
    #[inline(always)]
    pub fn icodefault(&mut self) -> ICODEFAULT_W {
        ICODEFAULT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Reflects the status of the bus decoders' fault detection. Any write to this register will clear all of the status bits within the register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [faultstatus](index.html) module"]
pub struct FAULTSTATUS_SPEC;
impl crate::RegisterSpec for FAULTSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [faultstatus::R](R) reader structure"]
impl crate::Readable for FAULTSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [faultstatus::W](W) writer structure"]
impl crate::Writable for FAULTSTATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FAULTSTATUS to value 0"]
impl crate::Resettable for FAULTSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

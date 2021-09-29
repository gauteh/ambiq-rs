#[doc = "Register `FIFOCTRL` reader"]
pub struct R(crate::R<FIFOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFOCTRL` writer"]
pub struct W(crate::W<FIFOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFOCTRL_SPEC>;
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
impl From<crate::W<FIFOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIFORSTN` reader - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
pub struct FIFORSTN_R(crate::FieldReader<bool, bool>);
impl FIFORSTN_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIFORSTN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFORSTN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFORSTN` writer - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
pub struct FIFORSTN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFORSTN_W<'a> {
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
#[doc = "Field `POPWR` reader - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
pub struct POPWR_R(crate::FieldReader<bool, bool>);
impl POPWR_R {
    pub(crate) fn new(bits: bool) -> Self {
        POPWR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POPWR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POPWR` writer - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
pub struct POPWR_W<'a> {
    w: &'a mut W,
}
impl<'a> POPWR_W<'a> {
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
    #[doc = "Bit 1 - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[inline(always)]
    pub fn fiforstn(&self) -> FIFORSTN_R {
        FIFORSTN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[inline(always)]
    pub fn popwr(&self) -> POPWR_R {
        POPWR_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Active low manual reset of the fifo. Write to 0 to reset fifo, and then write to 1 to remove the reset."]
    #[inline(always)]
    pub fn fiforstn(&mut self) -> FIFORSTN_W {
        FIFORSTN_W { w: self }
    }
    #[doc = "Bit 0 - Selects the mode in which 'pop' events are done for the fifo read operations. A value of '1' will prevent a pop event on a read operation, and will require a write to the FIFOPOP register to create a pop event. A value of '0' in this register will allow a pop event to occur on the read of the FIFOPOP register, and may cause inadvertant fifo pops when used in a debugging mode."]
    #[inline(always)]
    pub fn popwr(&mut self) -> POPWR_W {
        POPWR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifoctrl](index.html) module"]
pub struct FIFOCTRL_SPEC;
impl crate::RegisterSpec for FIFOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifoctrl::R](R) reader structure"]
impl crate::Readable for FIFOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifoctrl::W](W) writer structure"]
impl crate::Writable for FIFOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFOCTRL to value 0x02"]
impl crate::Resettable for FIFOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}

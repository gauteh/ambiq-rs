#[doc = "Register `MISCCTRL` reader"]
pub struct R(crate::R<MISCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISCCTRL` writer"]
pub struct W(crate::W<MISCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISCCTRL_SPEC>;
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
impl From<crate::W<MISCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLE_RESETN` reader - BLE reset signal."]
pub struct BLE_RESETN_R(crate::FieldReader<bool, bool>);
impl BLE_RESETN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BLE_RESETN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BLE_RESETN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BLE_RESETN` writer - BLE reset signal."]
pub struct BLE_RESETN_W<'a> {
    w: &'a mut W,
}
impl<'a> BLE_RESETN_W<'a> {
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
#[doc = "Field `RESERVED_RW_0` reader - Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
pub struct RESERVED_RW_0_R(crate::FieldReader<u8, u8>);
impl RESERVED_RW_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESERVED_RW_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESERVED_RW_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESERVED_RW_0` writer - Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
pub struct RESERVED_RW_0_W<'a> {
    w: &'a mut W,
}
impl<'a> RESERVED_RW_0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bit 5 - BLE reset signal."]
    #[inline(always)]
    pub fn ble_resetn(&self) -> BLE_RESETN_R {
        BLE_RESETN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 0:4 - Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_0(&self) -> RESERVED_RW_0_R {
        RESERVED_RW_0_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 5 - BLE reset signal."]
    #[inline(always)]
    pub fn ble_resetn(&mut self) -> BLE_RESETN_W {
        BLE_RESETN_W { w: self }
    }
    #[doc = "Bits 0:4 - Reserved bits, always leave unchanged. The MISCCTRL register must be modified via atomic RMW, leaving this bitfield completely unmodified. Failure to do so will result in unpredictable behavior."]
    #[inline(always)]
    pub fn reserved_rw_0(&mut self) -> RESERVED_RW_0_W {
        RESERVED_RW_0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Miscellaneous control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [miscctrl](index.html) module"]
pub struct MISCCTRL_SPEC;
impl crate::RegisterSpec for MISCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [miscctrl::R](R) reader structure"]
impl crate::Readable for MISCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [miscctrl::W](W) writer structure"]
impl crate::Writable for MISCCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISCCTRL to value 0"]
impl crate::Resettable for MISCCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

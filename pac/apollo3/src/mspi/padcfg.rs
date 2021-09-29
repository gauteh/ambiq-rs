#[doc = "Register `PADCFG` reader"]
pub struct R(crate::R<PADCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PADCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PADCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PADCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PADCFG` writer"]
pub struct W(crate::W<PADCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PADCFG_SPEC>;
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
impl From<crate::W<PADCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PADCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REVCS` reader - Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines"]
pub struct REVCS_R(crate::FieldReader<bool, bool>);
impl REVCS_R {
    pub(crate) fn new(bits: bool) -> Self {
        REVCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REVCS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REVCS` writer - Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines"]
pub struct REVCS_W<'a> {
    w: &'a mut W,
}
impl<'a> REVCS_W<'a> {
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
#[doc = "Field `IN3` reader - Data Input pad 3 pin muxing: 0=pad\\[3\\]
1=pad\\[7\\]"]
pub struct IN3_R(crate::FieldReader<bool, bool>);
impl IN3_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN3` writer - Data Input pad 3 pin muxing: 0=pad\\[3\\]
1=pad\\[7\\]"]
pub struct IN3_W<'a> {
    w: &'a mut W,
}
impl<'a> IN3_W<'a> {
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
#[doc = "Field `IN2` reader - Data Input pad 2 pin muxing: 0=pad\\[2\\]
1=pad\\[6\\]"]
pub struct IN2_R(crate::FieldReader<bool, bool>);
impl IN2_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN2` writer - Data Input pad 2 pin muxing: 0=pad\\[2\\]
1=pad\\[6\\]"]
pub struct IN2_W<'a> {
    w: &'a mut W,
}
impl<'a> IN2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `IN1` reader - Data Input pad 1 pin muxing: 0=pad\\[1\\]
1=pad\\[5\\]"]
pub struct IN1_R(crate::FieldReader<bool, bool>);
impl IN1_R {
    pub(crate) fn new(bits: bool) -> Self {
        IN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN1` writer - Data Input pad 1 pin muxing: 0=pad\\[1\\]
1=pad\\[5\\]"]
pub struct IN1_W<'a> {
    w: &'a mut W,
}
impl<'a> IN1_W<'a> {
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
#[doc = "Field `IN0` reader - Data Input pad 0 pin muxing: 0=pad\\[0\\]
1=pad\\[4\\]
2=pad\\[1\\]
3=pad\\[5\\]"]
pub struct IN0_R(crate::FieldReader<u8, u8>);
impl IN0_R {
    pub(crate) fn new(bits: u8) -> Self {
        IN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IN0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IN0` writer - Data Input pad 0 pin muxing: 0=pad\\[0\\]
1=pad\\[4\\]
2=pad\\[1\\]
3=pad\\[5\\]"]
pub struct IN0_W<'a> {
    w: &'a mut W,
}
impl<'a> IN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `OUT7` reader - Output pad 7 configuration. 0=data\\[7\\]
1=data\\[3\\]"]
pub struct OUT7_R(crate::FieldReader<bool, bool>);
impl OUT7_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT7` writer - Output pad 7 configuration. 0=data\\[7\\]
1=data\\[3\\]"]
pub struct OUT7_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT7_W<'a> {
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
#[doc = "Field `OUT6` reader - Output pad 6 configuration. 0=data\\[6\\]
1=data\\[2\\]"]
pub struct OUT6_R(crate::FieldReader<bool, bool>);
impl OUT6_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT6` writer - Output pad 6 configuration. 0=data\\[6\\]
1=data\\[2\\]"]
pub struct OUT6_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT6_W<'a> {
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
#[doc = "Field `OUT5` reader - Output pad 5 configuration. 0=data\\[5\\]
1=data\\[1\\]"]
pub struct OUT5_R(crate::FieldReader<bool, bool>);
impl OUT5_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT5` writer - Output pad 5 configuration. 0=data\\[5\\]
1=data\\[1\\]"]
pub struct OUT5_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT5_W<'a> {
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
#[doc = "Field `OUT4` reader - Output pad 4 configuration. 0=data\\[4\\]
1=data\\[0\\]"]
pub struct OUT4_R(crate::FieldReader<bool, bool>);
impl OUT4_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT4` writer - Output pad 4 configuration. 0=data\\[4\\]
1=data\\[0\\]"]
pub struct OUT4_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT4_W<'a> {
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
#[doc = "Field `OUT3` reader - Output pad 3 configuration. 0=data\\[3\\]
1=CLK"]
pub struct OUT3_R(crate::FieldReader<bool, bool>);
impl OUT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUT3` writer - Output pad 3 configuration. 0=data\\[3\\]
1=CLK"]
pub struct OUT3_W<'a> {
    w: &'a mut W,
}
impl<'a> OUT3_W<'a> {
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
    #[doc = "Bit 21 - Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines"]
    #[inline(always)]
    pub fn revcs(&self) -> REVCS_R {
        REVCS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Data Input pad 3 pin muxing: 0=pad\\[3\\]
1=pad\\[7\\]"]
    #[inline(always)]
    pub fn in3(&self) -> IN3_R {
        IN3_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Data Input pad 2 pin muxing: 0=pad\\[2\\]
1=pad\\[6\\]"]
    #[inline(always)]
    pub fn in2(&self) -> IN2_R {
        IN2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data Input pad 1 pin muxing: 0=pad\\[1\\]
1=pad\\[5\\]"]
    #[inline(always)]
    pub fn in1(&self) -> IN1_R {
        IN1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Data Input pad 0 pin muxing: 0=pad\\[0\\]
1=pad\\[4\\]
2=pad\\[1\\]
3=pad\\[5\\]"]
    #[inline(always)]
    pub fn in0(&self) -> IN0_R {
        IN0_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Output pad 7 configuration. 0=data\\[7\\]
1=data\\[3\\]"]
    #[inline(always)]
    pub fn out7(&self) -> OUT7_R {
        OUT7_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output pad 6 configuration. 0=data\\[6\\]
1=data\\[2\\]"]
    #[inline(always)]
    pub fn out6(&self) -> OUT6_R {
        OUT6_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Output pad 5 configuration. 0=data\\[5\\]
1=data\\[1\\]"]
    #[inline(always)]
    pub fn out5(&self) -> OUT5_R {
        OUT5_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Output pad 4 configuration. 0=data\\[4\\]
1=data\\[0\\]"]
    #[inline(always)]
    pub fn out4(&self) -> OUT4_R {
        OUT4_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Output pad 3 configuration. 0=data\\[3\\]
1=CLK"]
    #[inline(always)]
    pub fn out3(&self) -> OUT3_R {
        OUT3_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 21 - Reverse CS connections. Allows CS1 to be associated with lower data lanes and CS0 to be associated with upper data lines"]
    #[inline(always)]
    pub fn revcs(&mut self) -> REVCS_W {
        REVCS_W { w: self }
    }
    #[doc = "Bit 20 - Data Input pad 3 pin muxing: 0=pad\\[3\\]
1=pad\\[7\\]"]
    #[inline(always)]
    pub fn in3(&mut self) -> IN3_W {
        IN3_W { w: self }
    }
    #[doc = "Bit 19 - Data Input pad 2 pin muxing: 0=pad\\[2\\]
1=pad\\[6\\]"]
    #[inline(always)]
    pub fn in2(&mut self) -> IN2_W {
        IN2_W { w: self }
    }
    #[doc = "Bit 18 - Data Input pad 1 pin muxing: 0=pad\\[1\\]
1=pad\\[5\\]"]
    #[inline(always)]
    pub fn in1(&mut self) -> IN1_W {
        IN1_W { w: self }
    }
    #[doc = "Bits 16:17 - Data Input pad 0 pin muxing: 0=pad\\[0\\]
1=pad\\[4\\]
2=pad\\[1\\]
3=pad\\[5\\]"]
    #[inline(always)]
    pub fn in0(&mut self) -> IN0_W {
        IN0_W { w: self }
    }
    #[doc = "Bit 4 - Output pad 7 configuration. 0=data\\[7\\]
1=data\\[3\\]"]
    #[inline(always)]
    pub fn out7(&mut self) -> OUT7_W {
        OUT7_W { w: self }
    }
    #[doc = "Bit 3 - Output pad 6 configuration. 0=data\\[6\\]
1=data\\[2\\]"]
    #[inline(always)]
    pub fn out6(&mut self) -> OUT6_W {
        OUT6_W { w: self }
    }
    #[doc = "Bit 2 - Output pad 5 configuration. 0=data\\[5\\]
1=data\\[1\\]"]
    #[inline(always)]
    pub fn out5(&mut self) -> OUT5_W {
        OUT5_W { w: self }
    }
    #[doc = "Bit 1 - Output pad 4 configuration. 0=data\\[4\\]
1=data\\[0\\]"]
    #[inline(always)]
    pub fn out4(&mut self) -> OUT4_W {
        OUT4_W { w: self }
    }
    #[doc = "Bit 0 - Output pad 3 configuration. 0=data\\[3\\]
1=CLK"]
    #[inline(always)]
    pub fn out3(&mut self) -> OUT3_W {
        OUT3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI Output Pad Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [padcfg](index.html) module"]
pub struct PADCFG_SPEC;
impl crate::RegisterSpec for PADCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [padcfg::R](R) reader structure"]
impl crate::Readable for PADCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [padcfg::W](W) writer structure"]
impl crate::Writable for PADCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PADCFG to value 0"]
impl crate::Resettable for PADCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `HFADJ` reader"]
pub struct R(crate::R<HFADJ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFADJ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFADJ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFADJ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFADJ` writer"]
pub struct W(crate::W<HFADJ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFADJ_SPEC>;
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
impl From<crate::W<HFADJ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFADJ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Gain control for HFRC adjustment\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFADJGAIN_A {
    #[doc = "0: HF Adjust with Gain of 1 value."]
    GAIN_OF_1 = 0,
    #[doc = "1: HF Adjust with Gain of 0.5 value."]
    GAIN_OF_1_IN_2 = 1,
    #[doc = "2: HF Adjust with Gain of 0.25 value."]
    GAIN_OF_1_IN_4 = 2,
    #[doc = "3: HF Adjust with Gain of 0.125 value."]
    GAIN_OF_1_IN_8 = 3,
    #[doc = "4: HF Adjust with Gain of 0.0625 value."]
    GAIN_OF_1_IN_16 = 4,
    #[doc = "5: HF Adjust with Gain of 0.03125 value."]
    GAIN_OF_1_IN_32 = 5,
}
impl From<HFADJGAIN_A> for u8 {
    #[inline(always)]
    fn from(variant: HFADJGAIN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFADJGAIN` reader - Gain control for HFRC adjustment"]
pub struct HFADJGAIN_R(crate::FieldReader<u8, HFADJGAIN_A>);
impl HFADJGAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        HFADJGAIN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HFADJGAIN_A> {
        match self.bits {
            0 => Some(HFADJGAIN_A::GAIN_OF_1),
            1 => Some(HFADJGAIN_A::GAIN_OF_1_IN_2),
            2 => Some(HFADJGAIN_A::GAIN_OF_1_IN_4),
            3 => Some(HFADJGAIN_A::GAIN_OF_1_IN_8),
            4 => Some(HFADJGAIN_A::GAIN_OF_1_IN_16),
            5 => Some(HFADJGAIN_A::GAIN_OF_1_IN_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1`"]
    #[inline(always)]
    pub fn is_gain_of_1(&self) -> bool {
        **self == HFADJGAIN_A::GAIN_OF_1
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_2`"]
    #[inline(always)]
    pub fn is_gain_of_1_in_2(&self) -> bool {
        **self == HFADJGAIN_A::GAIN_OF_1_IN_2
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_4`"]
    #[inline(always)]
    pub fn is_gain_of_1_in_4(&self) -> bool {
        **self == HFADJGAIN_A::GAIN_OF_1_IN_4
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_8`"]
    #[inline(always)]
    pub fn is_gain_of_1_in_8(&self) -> bool {
        **self == HFADJGAIN_A::GAIN_OF_1_IN_8
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_16`"]
    #[inline(always)]
    pub fn is_gain_of_1_in_16(&self) -> bool {
        **self == HFADJGAIN_A::GAIN_OF_1_IN_16
    }
    #[doc = "Checks if the value of the field is `GAIN_OF_1_IN_32`"]
    #[inline(always)]
    pub fn is_gain_of_1_in_32(&self) -> bool {
        **self == HFADJGAIN_A::GAIN_OF_1_IN_32
    }
}
impl core::ops::Deref for HFADJGAIN_R {
    type Target = crate::FieldReader<u8, HFADJGAIN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFADJGAIN` writer - Gain control for HFRC adjustment"]
pub struct HFADJGAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFADJGAIN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFADJGAIN_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HF Adjust with Gain of 1 value."]
    #[inline(always)]
    pub fn gain_of_1(self) -> &'a mut W {
        self.variant(HFADJGAIN_A::GAIN_OF_1)
    }
    #[doc = "HF Adjust with Gain of 0.5 value."]
    #[inline(always)]
    pub fn gain_of_1_in_2(self) -> &'a mut W {
        self.variant(HFADJGAIN_A::GAIN_OF_1_IN_2)
    }
    #[doc = "HF Adjust with Gain of 0.25 value."]
    #[inline(always)]
    pub fn gain_of_1_in_4(self) -> &'a mut W {
        self.variant(HFADJGAIN_A::GAIN_OF_1_IN_4)
    }
    #[doc = "HF Adjust with Gain of 0.125 value."]
    #[inline(always)]
    pub fn gain_of_1_in_8(self) -> &'a mut W {
        self.variant(HFADJGAIN_A::GAIN_OF_1_IN_8)
    }
    #[doc = "HF Adjust with Gain of 0.0625 value."]
    #[inline(always)]
    pub fn gain_of_1_in_16(self) -> &'a mut W {
        self.variant(HFADJGAIN_A::GAIN_OF_1_IN_16)
    }
    #[doc = "HF Adjust with Gain of 0.03125 value."]
    #[inline(always)]
    pub fn gain_of_1_in_32(self) -> &'a mut W {
        self.variant(HFADJGAIN_A::GAIN_OF_1_IN_32)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "XT warmup period for HFRC adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFWARMUP_A {
    #[doc = "0: Autoadjust XT warmup period = 1-2 seconds value."]
    _1SEC = 0,
    #[doc = "1: Autoadjust XT warmup period = 2-4 seconds value."]
    _2SEC = 1,
}
impl From<HFWARMUP_A> for bool {
    #[inline(always)]
    fn from(variant: HFWARMUP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFWARMUP` reader - XT warmup period for HFRC adjustment"]
pub struct HFWARMUP_R(crate::FieldReader<bool, HFWARMUP_A>);
impl HFWARMUP_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFWARMUP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFWARMUP_A {
        match self.bits {
            false => HFWARMUP_A::_1SEC,
            true => HFWARMUP_A::_2SEC,
        }
    }
    #[doc = "Checks if the value of the field is `_1SEC`"]
    #[inline(always)]
    pub fn is_1sec(&self) -> bool {
        **self == HFWARMUP_A::_1SEC
    }
    #[doc = "Checks if the value of the field is `_2SEC`"]
    #[inline(always)]
    pub fn is_2sec(&self) -> bool {
        **self == HFWARMUP_A::_2SEC
    }
}
impl core::ops::Deref for HFWARMUP_R {
    type Target = crate::FieldReader<bool, HFWARMUP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFWARMUP` writer - XT warmup period for HFRC adjustment"]
pub struct HFWARMUP_W<'a> {
    w: &'a mut W,
}
impl<'a> HFWARMUP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFWARMUP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Autoadjust XT warmup period = 1-2 seconds value."]
    #[inline(always)]
    pub fn _1sec(self) -> &'a mut W {
        self.variant(HFWARMUP_A::_1SEC)
    }
    #[doc = "Autoadjust XT warmup period = 2-4 seconds value."]
    #[inline(always)]
    pub fn _2sec(self) -> &'a mut W {
        self.variant(HFWARMUP_A::_2SEC)
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
#[doc = "Field `HFXTADJ` reader - Target HFRC adjustment value."]
pub struct HFXTADJ_R(crate::FieldReader<u16, u16>);
impl HFXTADJ_R {
    pub(crate) fn new(bits: u16) -> Self {
        HFXTADJ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXTADJ_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXTADJ` writer - Target HFRC adjustment value."]
pub struct HFXTADJ_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXTADJ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 8)) | ((value as u32 & 0x0fff) << 8);
        self.w
    }
}
#[doc = "Repeat period for HFRC adjustment\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFADJCK_A {
    #[doc = "0: Autoadjust repeat period = 4 seconds value."]
    _4SEC = 0,
    #[doc = "1: Autoadjust repeat period = 16 seconds value."]
    _16SEC = 1,
    #[doc = "2: Autoadjust repeat period = 32 seconds value."]
    _32SEC = 2,
    #[doc = "3: Autoadjust repeat period = 64 seconds value."]
    _64SEC = 3,
    #[doc = "4: Autoadjust repeat period = 128 seconds value."]
    _128SEC = 4,
    #[doc = "5: Autoadjust repeat period = 256 seconds value."]
    _256SEC = 5,
    #[doc = "6: Autoadjust repeat period = 512 seconds value."]
    _512SEC = 6,
    #[doc = "7: Autoadjust repeat period = 1024 seconds value."]
    _1024SEC = 7,
}
impl From<HFADJCK_A> for u8 {
    #[inline(always)]
    fn from(variant: HFADJCK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HFADJCK` reader - Repeat period for HFRC adjustment"]
pub struct HFADJCK_R(crate::FieldReader<u8, HFADJCK_A>);
impl HFADJCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        HFADJCK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFADJCK_A {
        match self.bits {
            0 => HFADJCK_A::_4SEC,
            1 => HFADJCK_A::_16SEC,
            2 => HFADJCK_A::_32SEC,
            3 => HFADJCK_A::_64SEC,
            4 => HFADJCK_A::_128SEC,
            5 => HFADJCK_A::_256SEC,
            6 => HFADJCK_A::_512SEC,
            7 => HFADJCK_A::_1024SEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4SEC`"]
    #[inline(always)]
    pub fn is_4sec(&self) -> bool {
        **self == HFADJCK_A::_4SEC
    }
    #[doc = "Checks if the value of the field is `_16SEC`"]
    #[inline(always)]
    pub fn is_16sec(&self) -> bool {
        **self == HFADJCK_A::_16SEC
    }
    #[doc = "Checks if the value of the field is `_32SEC`"]
    #[inline(always)]
    pub fn is_32sec(&self) -> bool {
        **self == HFADJCK_A::_32SEC
    }
    #[doc = "Checks if the value of the field is `_64SEC`"]
    #[inline(always)]
    pub fn is_64sec(&self) -> bool {
        **self == HFADJCK_A::_64SEC
    }
    #[doc = "Checks if the value of the field is `_128SEC`"]
    #[inline(always)]
    pub fn is_128sec(&self) -> bool {
        **self == HFADJCK_A::_128SEC
    }
    #[doc = "Checks if the value of the field is `_256SEC`"]
    #[inline(always)]
    pub fn is_256sec(&self) -> bool {
        **self == HFADJCK_A::_256SEC
    }
    #[doc = "Checks if the value of the field is `_512SEC`"]
    #[inline(always)]
    pub fn is_512sec(&self) -> bool {
        **self == HFADJCK_A::_512SEC
    }
    #[doc = "Checks if the value of the field is `_1024SEC`"]
    #[inline(always)]
    pub fn is_1024sec(&self) -> bool {
        **self == HFADJCK_A::_1024SEC
    }
}
impl core::ops::Deref for HFADJCK_R {
    type Target = crate::FieldReader<u8, HFADJCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFADJCK` writer - Repeat period for HFRC adjustment"]
pub struct HFADJCK_W<'a> {
    w: &'a mut W,
}
impl<'a> HFADJCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFADJCK_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Autoadjust repeat period = 4 seconds value."]
    #[inline(always)]
    pub fn _4sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_4SEC)
    }
    #[doc = "Autoadjust repeat period = 16 seconds value."]
    #[inline(always)]
    pub fn _16sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_16SEC)
    }
    #[doc = "Autoadjust repeat period = 32 seconds value."]
    #[inline(always)]
    pub fn _32sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_32SEC)
    }
    #[doc = "Autoadjust repeat period = 64 seconds value."]
    #[inline(always)]
    pub fn _64sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_64SEC)
    }
    #[doc = "Autoadjust repeat period = 128 seconds value."]
    #[inline(always)]
    pub fn _128sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_128SEC)
    }
    #[doc = "Autoadjust repeat period = 256 seconds value."]
    #[inline(always)]
    pub fn _256sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_256SEC)
    }
    #[doc = "Autoadjust repeat period = 512 seconds value."]
    #[inline(always)]
    pub fn _512sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_512SEC)
    }
    #[doc = "Autoadjust repeat period = 1024 seconds value."]
    #[inline(always)]
    pub fn _1024sec(self) -> &'a mut W {
        self.variant(HFADJCK_A::_1024SEC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 1)) | ((value as u32 & 0x07) << 1);
        self.w
    }
}
#[doc = "HFRC adjustment control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HFADJEN_A {
    #[doc = "0: Disable the HFRC adjustment value."]
    DIS = 0,
    #[doc = "1: Enable the HFRC adjustment value."]
    EN = 1,
}
impl From<HFADJEN_A> for bool {
    #[inline(always)]
    fn from(variant: HFADJEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HFADJEN` reader - HFRC adjustment control"]
pub struct HFADJEN_R(crate::FieldReader<bool, HFADJEN_A>);
impl HFADJEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFADJEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HFADJEN_A {
        match self.bits {
            false => HFADJEN_A::DIS,
            true => HFADJEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == HFADJEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == HFADJEN_A::EN
    }
}
impl core::ops::Deref for HFADJEN_R {
    type Target = crate::FieldReader<bool, HFADJEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFADJEN` writer - HFRC adjustment control"]
pub struct HFADJEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFADJEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFADJEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the HFRC adjustment value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(HFADJEN_A::DIS)
    }
    #[doc = "Enable the HFRC adjustment value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(HFADJEN_A::EN)
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
    #[doc = "Bits 21:23 - Gain control for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjgain(&self) -> HFADJGAIN_R {
        HFADJGAIN_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 20 - XT warmup period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfwarmup(&self) -> HFWARMUP_R {
        HFWARMUP_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 8:19 - Target HFRC adjustment value."]
    #[inline(always)]
    pub fn hfxtadj(&self) -> HFXTADJ_R {
        HFXTADJ_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjck(&self) -> HFADJCK_R {
        HFADJCK_R::new(((self.bits >> 1) & 0x07) as u8)
    }
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline(always)]
    pub fn hfadjen(&self) -> HFADJEN_R {
        HFADJEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 21:23 - Gain control for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjgain(&mut self) -> HFADJGAIN_W {
        HFADJGAIN_W { w: self }
    }
    #[doc = "Bit 20 - XT warmup period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfwarmup(&mut self) -> HFWARMUP_W {
        HFWARMUP_W { w: self }
    }
    #[doc = "Bits 8:19 - Target HFRC adjustment value."]
    #[inline(always)]
    pub fn hfxtadj(&mut self) -> HFXTADJ_W {
        HFXTADJ_W { w: self }
    }
    #[doc = "Bits 1:3 - Repeat period for HFRC adjustment"]
    #[inline(always)]
    pub fn hfadjck(&mut self) -> HFADJCK_W {
        HFADJCK_W { w: self }
    }
    #[doc = "Bit 0 - HFRC adjustment control"]
    #[inline(always)]
    pub fn hfadjen(&mut self) -> HFADJEN_W {
        HFADJEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFRC Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfadj](index.html) module"]
pub struct HFADJ_SPEC;
impl crate::RegisterSpec for HFADJ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfadj::R](R) reader structure"]
impl crate::Readable for HFADJ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfadj::W](W) writer structure"]
impl crate::Writable for HFADJ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFADJ to value 0x0025_b800"]
impl crate::Resettable for HFADJ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0025_b800
    }
}

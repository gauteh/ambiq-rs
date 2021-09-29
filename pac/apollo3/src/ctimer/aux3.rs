#[doc = "Register `AUX3` reader"]
pub struct R(crate::R<AUX3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX3` writer"]
pub struct W(crate::W<AUX3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX3_SPEC>;
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
impl From<crate::W<AUX3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer B3 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRB3EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3EN23` reader - Counter/Timer B3 Upper compare enable."]
pub struct TMRB3EN23_R(crate::FieldReader<bool, TMRB3EN23_A>);
impl TMRB3EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3EN23_A {
        match self.bits {
            true => TMRB3EN23_A::DIS,
            false => TMRB3EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB3EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB3EN23_A::EN
    }
}
impl core::ops::Deref for TMRB3EN23_R {
    type Target = crate::FieldReader<bool, TMRB3EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3EN23` writer - Counter/Timer B3 Upper compare enable."]
pub struct TMRB3EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3EN23_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRB3POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3POL23` reader - Upper output polarity"]
pub struct TMRB3POL23_R(crate::FieldReader<bool, TMRB3POL23_A>);
impl TMRB3POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3POL23_A {
        match self.bits {
            false => TMRB3POL23_A::NORM,
            true => TMRB3POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRB3POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRB3POL23_A::INV
    }
}
impl core::ops::Deref for TMRB3POL23_R {
    type Target = crate::FieldReader<bool, TMRB3POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3POL23` writer - Upper output polarity"]
pub struct TMRB3POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB3POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB3POL23_A::INV)
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Counter/Timer B3 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRB3TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3TINV` reader - Counter/Timer B3 Invert on trigger."]
pub struct TMRB3TINV_R(crate::FieldReader<bool, TMRB3TINV_A>);
impl TMRB3TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3TINV_A {
        match self.bits {
            false => TMRB3TINV_A::DIS,
            true => TMRB3TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB3TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB3TINV_A::EN
    }
}
impl core::ops::Deref for TMRB3TINV_R {
    type Target = crate::FieldReader<bool, TMRB3TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3TINV` writer - Counter/Timer B3 Invert on trigger."]
pub struct TMRB3TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3TINV_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRB3NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3NOSYNC` reader - Source clock synchronization control."]
pub struct TMRB3NOSYNC_R(crate::FieldReader<bool, TMRB3NOSYNC_A>);
impl TMRB3NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3NOSYNC_A {
        match self.bits {
            false => TMRB3NOSYNC_A::DIS,
            true => TMRB3NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB3NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRB3NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRB3NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRB3NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3NOSYNC` writer - Source clock synchronization control."]
pub struct TMRB3NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB3NOSYNC_A::NOSYNC)
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Counter/Timer B3 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB3TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 1,
    #[doc = "2: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 2,
    #[doc = "3: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 3,
    #[doc = "4: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 4,
    #[doc = "5: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 5,
    #[doc = "6: Trigger source is CTIMERA6 OUT. value."]
    A6OUT = 6,
    #[doc = "7: Trigger source is CTIMERB6 OUT. value."]
    B6OUT = 7,
    #[doc = "8: Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL = 15,
}
impl From<TMRB3TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB3TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB3TRIG` reader - Counter/Timer B3 Trigger Select."]
pub struct TMRB3TRIG_R(crate::FieldReader<u8, TMRB3TRIG_A>);
impl TMRB3TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB3TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3TRIG_A {
        match self.bits {
            0 => TMRB3TRIG_A::DIS,
            1 => TMRB3TRIG_A::A3OUT,
            2 => TMRB3TRIG_A::B2OUT,
            3 => TMRB3TRIG_A::A2OUT,
            4 => TMRB3TRIG_A::A4OUT,
            5 => TMRB3TRIG_A::B4OUT,
            6 => TMRB3TRIG_A::A6OUT,
            7 => TMRB3TRIG_A::B6OUT,
            8 => TMRB3TRIG_A::B5OUT2,
            9 => TMRB3TRIG_A::A5OUT2,
            10 => TMRB3TRIG_A::A1OUT2,
            11 => TMRB3TRIG_A::B1OUT2,
            12 => TMRB3TRIG_A::A6OUT2DUAL,
            13 => TMRB3TRIG_A::A7OUT2DUAL,
            14 => TMRB3TRIG_A::B2OUT2DUAL,
            15 => TMRB3TRIG_A::A2OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB3TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == TMRB3TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        **self == TMRB3TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        **self == TMRB3TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == TMRB3TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        **self == TMRB3TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == TMRB3TRIG_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        **self == TMRB3TRIG_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        **self == TMRB3TRIG_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        **self == TMRB3TRIG_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        **self == TMRB3TRIG_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        **self == TMRB3TRIG_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        **self == TMRB3TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        **self == TMRB3TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b2out2dual(&self) -> bool {
        **self == TMRB3TRIG_A::B2OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a2out2dual(&self) -> bool {
        **self == TMRB3TRIG_A::A2OUT2DUAL
    }
}
impl core::ops::Deref for TMRB3TRIG_R {
    type Target = crate::FieldReader<u8, TMRB3TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3TRIG` writer - Counter/Timer B3 Trigger Select."]
pub struct TMRB3TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B6OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b2out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::B2OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a2out2dual(self) -> &'a mut W {
        self.variant(TMRB3TRIG_A::A2OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | ((value as u32 & 0x0f) << 23);
        self.w
    }
}
#[doc = "Field `TMRB3LMT` reader - Counter/Timer B3 Pattern Limit Count."]
pub struct TMRB3LMT_R(crate::FieldReader<u8, u8>);
impl TMRB3LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB3LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRB3LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3LMT` writer - Counter/Timer B3 Pattern Limit Count."]
pub struct TMRB3LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A3 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRA3EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3EN23` reader - Counter/Timer A3 Upper compare enable."]
pub struct TMRA3EN23_R(crate::FieldReader<bool, TMRA3EN23_A>);
impl TMRA3EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3EN23_A {
        match self.bits {
            true => TMRA3EN23_A::DIS,
            false => TMRA3EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA3EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA3EN23_A::EN
    }
}
impl core::ops::Deref for TMRA3EN23_R {
    type Target = crate::FieldReader<bool, TMRA3EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3EN23` writer - Counter/Timer A3 Upper compare enable."]
pub struct TMRA3EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3EN23_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Counter/Timer A3 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRA3POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3POL23` reader - Counter/Timer A3 Upper output polarity"]
pub struct TMRA3POL23_R(crate::FieldReader<bool, TMRA3POL23_A>);
impl TMRA3POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3POL23_A {
        match self.bits {
            false => TMRA3POL23_A::NORM,
            true => TMRA3POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRA3POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRA3POL23_A::INV
    }
}
impl core::ops::Deref for TMRA3POL23_R {
    type Target = crate::FieldReader<bool, TMRA3POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3POL23` writer - Counter/Timer A3 Upper output polarity"]
pub struct TMRA3POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA3POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA3POL23_A::INV)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Counter/Timer A3 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRA3TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3TINV` reader - Counter/Timer A3 Invert on trigger."]
pub struct TMRA3TINV_R(crate::FieldReader<bool, TMRA3TINV_A>);
impl TMRA3TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3TINV_A {
        match self.bits {
            false => TMRA3TINV_A::DIS,
            true => TMRA3TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA3TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA3TINV_A::EN
    }
}
impl core::ops::Deref for TMRA3TINV_R {
    type Target = crate::FieldReader<bool, TMRA3TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3TINV` writer - Counter/Timer A3 Invert on trigger."]
pub struct TMRA3TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3TINV_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Source clock synchronization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRA3NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3NOSYNC` reader - Source clock synchronization control."]
pub struct TMRA3NOSYNC_R(crate::FieldReader<bool, TMRA3NOSYNC_A>);
impl TMRA3NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3NOSYNC_A {
        match self.bits {
            false => TMRA3NOSYNC_A::DIS,
            true => TMRA3NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA3NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRA3NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRA3NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRA3NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3NOSYNC` writer - Source clock synchronization control."]
pub struct TMRA3NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA3NOSYNC_A::NOSYNC)
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Counter/Timer A3 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA3TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 1,
    #[doc = "2: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 2,
    #[doc = "3: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 3,
    #[doc = "4: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 4,
    #[doc = "5: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 5,
    #[doc = "6: Trigger source is CTIMERA7 OUT. value."]
    A7OUT = 6,
    #[doc = "7: Trigger source is CTIMERB7 OUT. value."]
    B7OUT = 7,
    #[doc = "8: Trigger source is CTIMERB5 OUT2. value."]
    B5OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA5 OUT2. value."]
    A5OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB2 OUT2, dual edge. value."]
    B2OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA2 OUT2, dual edge. value."]
    A2OUT2DUAL = 15,
}
impl From<TMRA3TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA3TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA3TRIG` reader - Counter/Timer A3 Trigger Select."]
pub struct TMRA3TRIG_R(crate::FieldReader<u8, TMRA3TRIG_A>);
impl TMRA3TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA3TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3TRIG_A {
        match self.bits {
            0 => TMRA3TRIG_A::DIS,
            1 => TMRA3TRIG_A::B3OUT,
            2 => TMRA3TRIG_A::B2OUT,
            3 => TMRA3TRIG_A::A2OUT,
            4 => TMRA3TRIG_A::A4OUT,
            5 => TMRA3TRIG_A::B4OUT,
            6 => TMRA3TRIG_A::A7OUT,
            7 => TMRA3TRIG_A::B7OUT,
            8 => TMRA3TRIG_A::B5OUT2,
            9 => TMRA3TRIG_A::A5OUT2,
            10 => TMRA3TRIG_A::A1OUT2,
            11 => TMRA3TRIG_A::B1OUT2,
            12 => TMRA3TRIG_A::A6OUT2DUAL,
            13 => TMRA3TRIG_A::A7OUT2DUAL,
            14 => TMRA3TRIG_A::B2OUT2DUAL,
            15 => TMRA3TRIG_A::A2OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA3TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == TMRA3TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        **self == TMRA3TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        **self == TMRA3TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == TMRA3TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        **self == TMRA3TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == TMRA3TRIG_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        **self == TMRA3TRIG_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT2`"]
    #[inline(always)]
    pub fn is_b5out2(&self) -> bool {
        **self == TMRA3TRIG_A::B5OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2`"]
    #[inline(always)]
    pub fn is_a5out2(&self) -> bool {
        **self == TMRA3TRIG_A::A5OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        **self == TMRA3TRIG_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        **self == TMRA3TRIG_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        **self == TMRA3TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        **self == TMRA3TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b2out2dual(&self) -> bool {
        **self == TMRA3TRIG_A::B2OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A2OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a2out2dual(&self) -> bool {
        **self == TMRA3TRIG_A::A2OUT2DUAL
    }
}
impl core::ops::Deref for TMRA3TRIG_R {
    type Target = crate::FieldReader<u8, TMRA3TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3TRIG` writer - Counter/Timer A3 Trigger Select."]
pub struct TMRA3TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A7OUT)
    }
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B7OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2. value."]
    #[inline(always)]
    pub fn b5out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B5OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2. value."]
    #[inline(always)]
    pub fn a5out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A5OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b2out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::B2OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a2out2dual(self) -> &'a mut W {
        self.variant(TMRA3TRIG_A::A2OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `TMRA3LMT` reader - Counter/Timer A3 Pattern Limit Count."]
pub struct TMRA3LMT_R(crate::FieldReader<u8, u8>);
impl TMRA3LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA3LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRA3LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3LMT` writer - Counter/Timer A3 Pattern Limit Count."]
pub struct TMRA3LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B3 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb3en23(&self) -> TMRB3EN23_R {
        TMRB3EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb3pol23(&self) -> TMRB3POL23_R {
        TMRB3POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B3 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb3tinv(&self) -> TMRB3TINV_R {
        TMRB3TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb3nosync(&self) -> TMRB3NOSYNC_R {
        TMRB3NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B3 Trigger Select."]
    #[inline(always)]
    pub fn tmrb3trig(&self) -> TMRB3TRIG_R {
        TMRB3TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb3lmt(&self) -> TMRB3LMT_R {
        TMRB3LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A3 Upper compare enable."]
    #[inline(always)]
    pub fn tmra3en23(&self) -> TMRA3EN23_R {
        TMRA3EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A3 Upper output polarity"]
    #[inline(always)]
    pub fn tmra3pol23(&self) -> TMRA3POL23_R {
        TMRA3POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A3 Invert on trigger."]
    #[inline(always)]
    pub fn tmra3tinv(&self) -> TMRA3TINV_R {
        TMRA3TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra3nosync(&self) -> TMRA3NOSYNC_R {
        TMRA3NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A3 Trigger Select."]
    #[inline(always)]
    pub fn tmra3trig(&self) -> TMRA3TRIG_R {
        TMRA3TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra3lmt(&self) -> TMRA3LMT_R {
        TMRA3LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B3 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb3en23(&mut self) -> TMRB3EN23_W {
        TMRB3EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb3pol23(&mut self) -> TMRB3POL23_W {
        TMRB3POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B3 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb3tinv(&mut self) -> TMRB3TINV_W {
        TMRB3TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb3nosync(&mut self) -> TMRB3NOSYNC_W {
        TMRB3NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B3 Trigger Select."]
    #[inline(always)]
    pub fn tmrb3trig(&mut self) -> TMRB3TRIG_W {
        TMRB3TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb3lmt(&mut self) -> TMRB3LMT_W {
        TMRB3LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A3 Upper compare enable."]
    #[inline(always)]
    pub fn tmra3en23(&mut self) -> TMRA3EN23_W {
        TMRA3EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A3 Upper output polarity"]
    #[inline(always)]
    pub fn tmra3pol23(&mut self) -> TMRA3POL23_W {
        TMRA3POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A3 Invert on trigger."]
    #[inline(always)]
    pub fn tmra3tinv(&mut self) -> TMRA3TINV_W {
        TMRA3TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra3nosync(&mut self) -> TMRA3NOSYNC_W {
        TMRA3NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A3 Trigger Select."]
    #[inline(always)]
    pub fn tmra3trig(&mut self) -> TMRA3TRIG_W {
        TMRA3TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A3 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra3lmt(&mut self) -> TMRA3LMT_W {
        TMRA3LMT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux3](index.html) module"]
pub struct AUX3_SPEC;
impl crate::RegisterSpec for AUX3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux3::R](R) reader structure"]
impl crate::Readable for AUX3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux3::W](W) writer structure"]
impl crate::Writable for AUX3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUX3 to value 0"]
impl crate::Resettable for AUX3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

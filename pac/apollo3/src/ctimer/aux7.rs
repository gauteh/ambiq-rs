#[doc = "Register `AUX7` reader"]
pub struct R(crate::R<AUX7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX7` writer"]
pub struct W(crate::W<AUX7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX7_SPEC>;
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
impl From<crate::W<AUX7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer B7 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRB7EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB7EN23` reader - Counter/Timer B7 Upper compare enable."]
pub struct TMRB7EN23_R(crate::FieldReader<bool, TMRB7EN23_A>);
impl TMRB7EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB7EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7EN23_A {
        match self.bits {
            true => TMRB7EN23_A::DIS,
            false => TMRB7EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB7EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB7EN23_A::EN
    }
}
impl core::ops::Deref for TMRB7EN23_R {
    type Target = crate::FieldReader<bool, TMRB7EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB7EN23` writer - Counter/Timer B7 Upper compare enable."]
pub struct TMRB7EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7EN23_A::EN)
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
pub enum TMRB7POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRB7POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB7POL23` reader - Upper output polarity"]
pub struct TMRB7POL23_R(crate::FieldReader<bool, TMRB7POL23_A>);
impl TMRB7POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB7POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7POL23_A {
        match self.bits {
            false => TMRB7POL23_A::NORM,
            true => TMRB7POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRB7POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRB7POL23_A::INV
    }
}
impl core::ops::Deref for TMRB7POL23_R {
    type Target = crate::FieldReader<bool, TMRB7POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB7POL23` writer - Upper output polarity"]
pub struct TMRB7POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB7POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB7POL23_A::INV)
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
#[doc = "Counter/Timer B7 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB7TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRB7TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB7TINV` reader - Counter/Timer B7 Invert on trigger."]
pub struct TMRB7TINV_R(crate::FieldReader<bool, TMRB7TINV_A>);
impl TMRB7TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB7TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7TINV_A {
        match self.bits {
            false => TMRB7TINV_A::DIS,
            true => TMRB7TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB7TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB7TINV_A::EN
    }
}
impl core::ops::Deref for TMRB7TINV_R {
    type Target = crate::FieldReader<bool, TMRB7TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB7TINV` writer - Counter/Timer B7 Invert on trigger."]
pub struct TMRB7TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB7TINV_A::EN)
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
pub enum TMRB7NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRB7NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB7NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB7NOSYNC` reader - Source clock synchronization control."]
pub struct TMRB7NOSYNC_R(crate::FieldReader<bool, TMRB7NOSYNC_A>);
impl TMRB7NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB7NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7NOSYNC_A {
        match self.bits {
            false => TMRB7NOSYNC_A::DIS,
            true => TMRB7NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB7NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRB7NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRB7NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRB7NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB7NOSYNC` writer - Source clock synchronization control."]
pub struct TMRB7NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB7NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer B7 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB7TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA7 OUT. value."]
    A7OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA5 OUT. value."]
    A5OUT = 4,
    #[doc = "5: Trigger source is CTIMERB5 OUT. value."]
    B5OUT = 5,
    #[doc = "6: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 6,
    #[doc = "7: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB2 OUT2. value."]
    B2OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB1 OUT2, dual edge. value."]
    B1OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA1 OUT2, dual edge. value."]
    A1OUT2DUAL = 15,
}
impl From<TMRB7TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB7TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB7TRIG` reader - Counter/Timer B7 Trigger Select."]
pub struct TMRB7TRIG_R(crate::FieldReader<u8, TMRB7TRIG_A>);
impl TMRB7TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB7TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB7TRIG_A {
        match self.bits {
            0 => TMRB7TRIG_A::DIS,
            1 => TMRB7TRIG_A::A7OUT,
            2 => TMRB7TRIG_A::B3OUT,
            3 => TMRB7TRIG_A::A3OUT,
            4 => TMRB7TRIG_A::A5OUT,
            5 => TMRB7TRIG_A::B5OUT,
            6 => TMRB7TRIG_A::A2OUT,
            7 => TMRB7TRIG_A::B2OUT,
            8 => TMRB7TRIG_A::B3OUT2,
            9 => TMRB7TRIG_A::A3OUT2,
            10 => TMRB7TRIG_A::A2OUT2,
            11 => TMRB7TRIG_A::B2OUT2,
            12 => TMRB7TRIG_A::A6OUT2DUAL,
            13 => TMRB7TRIG_A::A7OUT2DUAL,
            14 => TMRB7TRIG_A::B1OUT2DUAL,
            15 => TMRB7TRIG_A::A1OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB7TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == TMRB7TRIG_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == TMRB7TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == TMRB7TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        **self == TMRB7TRIG_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        **self == TMRB7TRIG_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        **self == TMRB7TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        **self == TMRB7TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == TMRB7TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == TMRB7TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        **self == TMRB7TRIG_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        **self == TMRB7TRIG_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        **self == TMRB7TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        **self == TMRB7TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B1OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b1out2dual(&self) -> bool {
        **self == TMRB7TRIG_A::B1OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A1OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a1out2dual(&self) -> bool {
        **self == TMRB7TRIG_A::A1OUT2DUAL
    }
}
impl core::ops::Deref for TMRB7TRIG_R {
    type Target = crate::FieldReader<u8, TMRB7TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB7TRIG` writer - Counter/Timer B7 Trigger Select."]
pub struct TMRB7TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB7TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A7OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::B5OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b1out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::B1OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a1out2dual(self) -> &'a mut W {
        self.variant(TMRB7TRIG_A::A1OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | ((value as u32 & 0x0f) << 23);
        self.w
    }
}
#[doc = "Field `TMRB7LMT` reader - Counter/Timer B7 Pattern Limit Count."]
pub struct TMRB7LMT_R(crate::FieldReader<u8, u8>);
impl TMRB7LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB7LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRB7LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB7LMT` writer - Counter/Timer B7 Pattern Limit Count."]
pub struct TMRB7LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB7LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A7 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRA7EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA7EN23` reader - Counter/Timer A7 Upper compare enable."]
pub struct TMRA7EN23_R(crate::FieldReader<bool, TMRA7EN23_A>);
impl TMRA7EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA7EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7EN23_A {
        match self.bits {
            true => TMRA7EN23_A::DIS,
            false => TMRA7EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA7EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA7EN23_A::EN
    }
}
impl core::ops::Deref for TMRA7EN23_R {
    type Target = crate::FieldReader<bool, TMRA7EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA7EN23` writer - Counter/Timer A7 Upper compare enable."]
pub struct TMRA7EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7EN23_A::EN)
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
#[doc = "Counter/Timer A7 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRA7POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA7POL23` reader - Counter/Timer A7 Upper output polarity"]
pub struct TMRA7POL23_R(crate::FieldReader<bool, TMRA7POL23_A>);
impl TMRA7POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA7POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7POL23_A {
        match self.bits {
            false => TMRA7POL23_A::NORM,
            true => TMRA7POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRA7POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRA7POL23_A::INV
    }
}
impl core::ops::Deref for TMRA7POL23_R {
    type Target = crate::FieldReader<bool, TMRA7POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA7POL23` writer - Counter/Timer A7 Upper output polarity"]
pub struct TMRA7POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA7POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA7POL23_A::INV)
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
#[doc = "Counter/Timer A7 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA7TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRA7TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA7TINV` reader - Counter/Timer A7 Invert on trigger."]
pub struct TMRA7TINV_R(crate::FieldReader<bool, TMRA7TINV_A>);
impl TMRA7TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA7TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7TINV_A {
        match self.bits {
            false => TMRA7TINV_A::DIS,
            true => TMRA7TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA7TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA7TINV_A::EN
    }
}
impl core::ops::Deref for TMRA7TINV_R {
    type Target = crate::FieldReader<bool, TMRA7TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA7TINV` writer - Counter/Timer A7 Invert on trigger."]
pub struct TMRA7TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA7TINV_A::EN)
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
pub enum TMRA7NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRA7NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA7NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA7NOSYNC` reader - Source clock synchronization control."]
pub struct TMRA7NOSYNC_R(crate::FieldReader<bool, TMRA7NOSYNC_A>);
impl TMRA7NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA7NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7NOSYNC_A {
        match self.bits {
            false => TMRA7NOSYNC_A::DIS,
            true => TMRA7NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA7NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRA7NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRA7NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRA7NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA7NOSYNC` writer - Source clock synchronization control."]
pub struct TMRA7NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA7NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer A7 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA7TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB7 OUT. value."]
    B7OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA1 OUT. value."]
    A1OUT = 4,
    #[doc = "5: Trigger source is CTIMERB1 OUT. value."]
    B1OUT = 5,
    #[doc = "6: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 6,
    #[doc = "7: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB2 OUT2. value."]
    B2OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB4 OUT2, dual edge. value."]
    B4OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA4 OUT2, dual edge. value."]
    A4OUT2DUAL = 15,
}
impl From<TMRA7TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA7TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA7TRIG` reader - Counter/Timer A7 Trigger Select."]
pub struct TMRA7TRIG_R(crate::FieldReader<u8, TMRA7TRIG_A>);
impl TMRA7TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA7TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA7TRIG_A {
        match self.bits {
            0 => TMRA7TRIG_A::DIS,
            1 => TMRA7TRIG_A::B7OUT,
            2 => TMRA7TRIG_A::B3OUT,
            3 => TMRA7TRIG_A::A3OUT,
            4 => TMRA7TRIG_A::A1OUT,
            5 => TMRA7TRIG_A::B1OUT,
            6 => TMRA7TRIG_A::A4OUT,
            7 => TMRA7TRIG_A::B4OUT,
            8 => TMRA7TRIG_A::B3OUT2,
            9 => TMRA7TRIG_A::A3OUT2,
            10 => TMRA7TRIG_A::A2OUT2,
            11 => TMRA7TRIG_A::B2OUT2,
            12 => TMRA7TRIG_A::A6OUT2DUAL,
            13 => TMRA7TRIG_A::A5OUT2DUAL,
            14 => TMRA7TRIG_A::B4OUT2DUAL,
            15 => TMRA7TRIG_A::A4OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA7TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        **self == TMRA7TRIG_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == TMRA7TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == TMRA7TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == TMRA7TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == TMRA7TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == TMRA7TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        **self == TMRA7TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == TMRA7TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == TMRA7TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        **self == TMRA7TRIG_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        **self == TMRA7TRIG_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        **self == TMRA7TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a5out2dual(&self) -> bool {
        **self == TMRA7TRIG_A::A5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b4out2dual(&self) -> bool {
        **self == TMRA7TRIG_A::B4OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A4OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a4out2dual(&self) -> bool {
        **self == TMRA7TRIG_A::A4OUT2DUAL
    }
}
impl core::ops::Deref for TMRA7TRIG_R {
    type Target = crate::FieldReader<u8, TMRA7TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA7TRIG` writer - Counter/Timer A7 Trigger Select."]
pub struct TMRA7TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA7TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::B7OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b4out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::B4OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA4 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a4out2dual(self) -> &'a mut W {
        self.variant(TMRA7TRIG_A::A4OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `TMRA7LMT` reader - Counter/Timer A7 Pattern Limit Count."]
pub struct TMRA7LMT_R(crate::FieldReader<u8, u8>);
impl TMRA7LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA7LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRA7LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA7LMT` writer - Counter/Timer A7 Pattern Limit Count."]
pub struct TMRA7LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA7LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B7 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb7en23(&self) -> TMRB7EN23_R {
        TMRB7EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb7pol23(&self) -> TMRB7POL23_R {
        TMRB7POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B7 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb7tinv(&self) -> TMRB7TINV_R {
        TMRB7TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb7nosync(&self) -> TMRB7NOSYNC_R {
        TMRB7NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B7 Trigger Select."]
    #[inline(always)]
    pub fn tmrb7trig(&self) -> TMRB7TRIG_R {
        TMRB7TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B7 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb7lmt(&self) -> TMRB7LMT_R {
        TMRB7LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A7 Upper compare enable."]
    #[inline(always)]
    pub fn tmra7en23(&self) -> TMRA7EN23_R {
        TMRA7EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A7 Upper output polarity"]
    #[inline(always)]
    pub fn tmra7pol23(&self) -> TMRA7POL23_R {
        TMRA7POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A7 Invert on trigger."]
    #[inline(always)]
    pub fn tmra7tinv(&self) -> TMRA7TINV_R {
        TMRA7TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra7nosync(&self) -> TMRA7NOSYNC_R {
        TMRA7NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A7 Trigger Select."]
    #[inline(always)]
    pub fn tmra7trig(&self) -> TMRA7TRIG_R {
        TMRA7TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A7 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra7lmt(&self) -> TMRA7LMT_R {
        TMRA7LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B7 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb7en23(&mut self) -> TMRB7EN23_W {
        TMRB7EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb7pol23(&mut self) -> TMRB7POL23_W {
        TMRB7POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B7 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb7tinv(&mut self) -> TMRB7TINV_W {
        TMRB7TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb7nosync(&mut self) -> TMRB7NOSYNC_W {
        TMRB7NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B7 Trigger Select."]
    #[inline(always)]
    pub fn tmrb7trig(&mut self) -> TMRB7TRIG_W {
        TMRB7TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B7 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb7lmt(&mut self) -> TMRB7LMT_W {
        TMRB7LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A7 Upper compare enable."]
    #[inline(always)]
    pub fn tmra7en23(&mut self) -> TMRA7EN23_W {
        TMRA7EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A7 Upper output polarity"]
    #[inline(always)]
    pub fn tmra7pol23(&mut self) -> TMRA7POL23_W {
        TMRA7POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A7 Invert on trigger."]
    #[inline(always)]
    pub fn tmra7tinv(&mut self) -> TMRA7TINV_W {
        TMRA7TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra7nosync(&mut self) -> TMRA7NOSYNC_W {
        TMRA7NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A7 Trigger Select."]
    #[inline(always)]
    pub fn tmra7trig(&mut self) -> TMRA7TRIG_W {
        TMRA7TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A7 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra7lmt(&mut self) -> TMRA7LMT_W {
        TMRA7LMT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux7](index.html) module"]
pub struct AUX7_SPEC;
impl crate::RegisterSpec for AUX7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux7::R](R) reader structure"]
impl crate::Readable for AUX7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux7::W](W) writer structure"]
impl crate::Writable for AUX7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUX7 to value 0"]
impl crate::Resettable for AUX7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

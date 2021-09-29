#[doc = "Register `AUX6` reader"]
pub struct R(crate::R<AUX6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX6` writer"]
pub struct W(crate::W<AUX6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX6_SPEC>;
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
impl From<crate::W<AUX6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer B6 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRB6EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6EN23` reader - Counter/Timer B6 Upper compare enable."]
pub struct TMRB6EN23_R(crate::FieldReader<bool, TMRB6EN23_A>);
impl TMRB6EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6EN23_A {
        match self.bits {
            true => TMRB6EN23_A::DIS,
            false => TMRB6EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB6EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB6EN23_A::EN
    }
}
impl core::ops::Deref for TMRB6EN23_R {
    type Target = crate::FieldReader<bool, TMRB6EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6EN23` writer - Counter/Timer B6 Upper compare enable."]
pub struct TMRB6EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6EN23_A::EN)
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
pub enum TMRB6POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRB6POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6POL23` reader - Upper output polarity"]
pub struct TMRB6POL23_R(crate::FieldReader<bool, TMRB6POL23_A>);
impl TMRB6POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6POL23_A {
        match self.bits {
            false => TMRB6POL23_A::NORM,
            true => TMRB6POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRB6POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRB6POL23_A::INV
    }
}
impl core::ops::Deref for TMRB6POL23_R {
    type Target = crate::FieldReader<bool, TMRB6POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6POL23` writer - Upper output polarity"]
pub struct TMRB6POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB6POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB6POL23_A::INV)
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
#[doc = "Counter/Timer B6 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRB6TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6TINV` reader - Counter/Timer B6 Invert on trigger."]
pub struct TMRB6TINV_R(crate::FieldReader<bool, TMRB6TINV_A>);
impl TMRB6TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6TINV_A {
        match self.bits {
            false => TMRB6TINV_A::DIS,
            true => TMRB6TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB6TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB6TINV_A::EN
    }
}
impl core::ops::Deref for TMRB6TINV_R {
    type Target = crate::FieldReader<bool, TMRB6TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6TINV` writer - Counter/Timer B6 Invert on trigger."]
pub struct TMRB6TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6TINV_A::EN)
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
pub enum TMRB6NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRB6NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6NOSYNC` reader - Source clock synchronization control."]
pub struct TMRB6NOSYNC_R(crate::FieldReader<bool, TMRB6NOSYNC_A>);
impl TMRB6NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6NOSYNC_A {
        match self.bits {
            false => TMRB6NOSYNC_A::DIS,
            true => TMRB6NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB6NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRB6NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRB6NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRB6NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6NOSYNC` writer - Source clock synchronization control."]
pub struct TMRB6NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB6NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer B6 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB6TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA6 OUT. value."]
    A6OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 4,
    #[doc = "5: Trigger source is CTIMERB4 OUT. value."]
    B4OUT = 5,
    #[doc = "6: Trigger source is CTIMERA1 OUT. value."]
    A1OUT = 6,
    #[doc = "7: Trigger source is CTIMERB1 OUT. value."]
    B1OUT = 7,
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
    #[doc = "14: Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0OUT2DUAL = 15,
}
impl From<TMRB6TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB6TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB6TRIG` reader - Counter/Timer B6 Trigger Select."]
pub struct TMRB6TRIG_R(crate::FieldReader<u8, TMRB6TRIG_A>);
impl TMRB6TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB6TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6TRIG_A {
        match self.bits {
            0 => TMRB6TRIG_A::DIS,
            1 => TMRB6TRIG_A::A6OUT,
            2 => TMRB6TRIG_A::B3OUT,
            3 => TMRB6TRIG_A::A3OUT,
            4 => TMRB6TRIG_A::A4OUT,
            5 => TMRB6TRIG_A::B4OUT,
            6 => TMRB6TRIG_A::A1OUT,
            7 => TMRB6TRIG_A::B1OUT,
            8 => TMRB6TRIG_A::B3OUT2,
            9 => TMRB6TRIG_A::A3OUT2,
            10 => TMRB6TRIG_A::A2OUT2,
            11 => TMRB6TRIG_A::B2OUT2,
            12 => TMRB6TRIG_A::A6OUT2DUAL,
            13 => TMRB6TRIG_A::A7OUT2DUAL,
            14 => TMRB6TRIG_A::B0OUT2DUAL,
            15 => TMRB6TRIG_A::A0OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB6TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == TMRB6TRIG_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == TMRB6TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == TMRB6TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == TMRB6TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B4OUT`"]
    #[inline(always)]
    pub fn is_b4out(&self) -> bool {
        **self == TMRB6TRIG_A::B4OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == TMRB6TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == TMRB6TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == TMRB6TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == TMRB6TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        **self == TMRB6TRIG_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        **self == TMRB6TRIG_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        **self == TMRB6TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        **self == TMRB6TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B0OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b0out2dual(&self) -> bool {
        **self == TMRB6TRIG_A::B0OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A0OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a0out2dual(&self) -> bool {
        **self == TMRB6TRIG_A::A0OUT2DUAL
    }
}
impl core::ops::Deref for TMRB6TRIG_R {
    type Target = crate::FieldReader<u8, TMRB6TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6TRIG` writer - Counter/Timer B6 Trigger Select."]
pub struct TMRB6TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn b4out(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::B4OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERB2 OUT2. value."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b0out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::B0OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a0out2dual(self) -> &'a mut W {
        self.variant(TMRB6TRIG_A::A0OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | ((value as u32 & 0x0f) << 23);
        self.w
    }
}
#[doc = "Field `TMRB6LMT` reader - Counter/Timer B6 Pattern Limit Count."]
pub struct TMRB6LMT_R(crate::FieldReader<u8, u8>);
impl TMRB6LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB6LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRB6LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6LMT` writer - Counter/Timer B6 Pattern Limit Count."]
pub struct TMRB6LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A6 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRA6EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6EN23` reader - Counter/Timer A6 Upper compare enable."]
pub struct TMRA6EN23_R(crate::FieldReader<bool, TMRA6EN23_A>);
impl TMRA6EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6EN23_A {
        match self.bits {
            true => TMRA6EN23_A::DIS,
            false => TMRA6EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA6EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA6EN23_A::EN
    }
}
impl core::ops::Deref for TMRA6EN23_R {
    type Target = crate::FieldReader<bool, TMRA6EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6EN23` writer - Counter/Timer A6 Upper compare enable."]
pub struct TMRA6EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6EN23_A::EN)
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
#[doc = "Counter/Timer A6 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRA6POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6POL23` reader - Counter/Timer A6 Upper output polarity"]
pub struct TMRA6POL23_R(crate::FieldReader<bool, TMRA6POL23_A>);
impl TMRA6POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6POL23_A {
        match self.bits {
            false => TMRA6POL23_A::NORM,
            true => TMRA6POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRA6POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRA6POL23_A::INV
    }
}
impl core::ops::Deref for TMRA6POL23_R {
    type Target = crate::FieldReader<bool, TMRA6POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6POL23` writer - Counter/Timer A6 Upper output polarity"]
pub struct TMRA6POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA6POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA6POL23_A::INV)
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
#[doc = "Counter/Timer A6 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRA6TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6TINV` reader - Counter/Timer A6 Invert on trigger."]
pub struct TMRA6TINV_R(crate::FieldReader<bool, TMRA6TINV_A>);
impl TMRA6TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6TINV_A {
        match self.bits {
            false => TMRA6TINV_A::DIS,
            true => TMRA6TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA6TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA6TINV_A::EN
    }
}
impl core::ops::Deref for TMRA6TINV_R {
    type Target = crate::FieldReader<bool, TMRA6TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6TINV` writer - Counter/Timer A6 Invert on trigger."]
pub struct TMRA6TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6TINV_A::EN)
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
pub enum TMRA6NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRA6NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6NOSYNC` reader - Source clock synchronization control."]
pub struct TMRA6NOSYNC_R(crate::FieldReader<bool, TMRA6NOSYNC_A>);
impl TMRA6NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6NOSYNC_A {
        match self.bits {
            false => TMRA6NOSYNC_A::DIS,
            true => TMRA6NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA6NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRA6NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRA6NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRA6NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6NOSYNC` writer - Source clock synchronization control."]
pub struct TMRA6NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA6NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer A6 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA6TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERB6 OUT. value."]
    B6OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA5 OUT. value."]
    A5OUT = 4,
    #[doc = "5: Trigger source is CTIMERB5 OUT. value."]
    B5OUT = 5,
    #[doc = "6: Trigger source is CTIMERA1 OUT. value."]
    A1OUT = 6,
    #[doc = "7: Trigger source is CTIMERB1 OUT. value."]
    B1OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA2 OUT2. value."]
    A2OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERBb OUT2. value."]
    B2OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB0 OUT2, dual edge. value."]
    B0OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA0 OUT2, dual edge. value."]
    A0OUT2DUAL = 15,
}
impl From<TMRA6TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA6TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA6TRIG` reader - Counter/Timer A6 Trigger Select."]
pub struct TMRA6TRIG_R(crate::FieldReader<u8, TMRA6TRIG_A>);
impl TMRA6TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA6TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6TRIG_A {
        match self.bits {
            0 => TMRA6TRIG_A::DIS,
            1 => TMRA6TRIG_A::B6OUT,
            2 => TMRA6TRIG_A::B3OUT,
            3 => TMRA6TRIG_A::A3OUT,
            4 => TMRA6TRIG_A::A5OUT,
            5 => TMRA6TRIG_A::B5OUT,
            6 => TMRA6TRIG_A::A1OUT,
            7 => TMRA6TRIG_A::B1OUT,
            8 => TMRA6TRIG_A::B3OUT2,
            9 => TMRA6TRIG_A::A3OUT2,
            10 => TMRA6TRIG_A::A2OUT2,
            11 => TMRA6TRIG_A::B2OUT2,
            12 => TMRA6TRIG_A::A5OUT2DUAL,
            13 => TMRA6TRIG_A::A7OUT2DUAL,
            14 => TMRA6TRIG_A::B0OUT2DUAL,
            15 => TMRA6TRIG_A::A0OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA6TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        **self == TMRA6TRIG_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == TMRA6TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == TMRA6TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A5OUT`"]
    #[inline(always)]
    pub fn is_a5out(&self) -> bool {
        **self == TMRA6TRIG_A::A5OUT
    }
    #[doc = "Checks if the value of the field is `B5OUT`"]
    #[inline(always)]
    pub fn is_b5out(&self) -> bool {
        **self == TMRA6TRIG_A::B5OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == TMRA6TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == TMRA6TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == TMRA6TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == TMRA6TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A2OUT2`"]
    #[inline(always)]
    pub fn is_a2out2(&self) -> bool {
        **self == TMRA6TRIG_A::A2OUT2
    }
    #[doc = "Checks if the value of the field is `B2OUT2`"]
    #[inline(always)]
    pub fn is_b2out2(&self) -> bool {
        **self == TMRA6TRIG_A::B2OUT2
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a5out2dual(&self) -> bool {
        **self == TMRA6TRIG_A::A5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        **self == TMRA6TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B0OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b0out2dual(&self) -> bool {
        **self == TMRA6TRIG_A::B0OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A0OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a0out2dual(&self) -> bool {
        **self == TMRA6TRIG_A::A0OUT2DUAL
    }
}
impl core::ops::Deref for TMRA6TRIG_R {
    type Target = crate::FieldReader<u8, TMRA6TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6TRIG` writer - Counter/Timer A6 Trigger Select."]
pub struct TMRA6TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::B6OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA5 OUT. value."]
    #[inline(always)]
    pub fn a5out(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A5OUT)
    }
    #[doc = "Trigger source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn b5out(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::B5OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA2 OUT2. value."]
    #[inline(always)]
    pub fn a2out2(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A2OUT2)
    }
    #[doc = "Trigger source is CTIMERBb OUT2. value."]
    #[inline(always)]
    pub fn b2out2(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::B2OUT2)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB0 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b0out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::B0OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA0 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a0out2dual(self) -> &'a mut W {
        self.variant(TMRA6TRIG_A::A0OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `TMRA6LMT` reader - Counter/Timer A6 Pattern Limit Count."]
pub struct TMRA6LMT_R(crate::FieldReader<u8, u8>);
impl TMRA6LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA6LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRA6LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6LMT` writer - Counter/Timer A6 Pattern Limit Count."]
pub struct TMRA6LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B6 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb6en23(&self) -> TMRB6EN23_R {
        TMRB6EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb6pol23(&self) -> TMRB6POL23_R {
        TMRB6POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B6 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb6tinv(&self) -> TMRB6TINV_R {
        TMRB6TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb6nosync(&self) -> TMRB6NOSYNC_R {
        TMRB6NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B6 Trigger Select."]
    #[inline(always)]
    pub fn tmrb6trig(&self) -> TMRB6TRIG_R {
        TMRB6TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B6 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb6lmt(&self) -> TMRB6LMT_R {
        TMRB6LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A6 Upper compare enable."]
    #[inline(always)]
    pub fn tmra6en23(&self) -> TMRA6EN23_R {
        TMRA6EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A6 Upper output polarity"]
    #[inline(always)]
    pub fn tmra6pol23(&self) -> TMRA6POL23_R {
        TMRA6POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A6 Invert on trigger."]
    #[inline(always)]
    pub fn tmra6tinv(&self) -> TMRA6TINV_R {
        TMRA6TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra6nosync(&self) -> TMRA6NOSYNC_R {
        TMRA6NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A6 Trigger Select."]
    #[inline(always)]
    pub fn tmra6trig(&self) -> TMRA6TRIG_R {
        TMRA6TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A6 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra6lmt(&self) -> TMRA6LMT_R {
        TMRA6LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B6 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb6en23(&mut self) -> TMRB6EN23_W {
        TMRB6EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb6pol23(&mut self) -> TMRB6POL23_W {
        TMRB6POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B6 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb6tinv(&mut self) -> TMRB6TINV_W {
        TMRB6TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb6nosync(&mut self) -> TMRB6NOSYNC_W {
        TMRB6NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B6 Trigger Select."]
    #[inline(always)]
    pub fn tmrb6trig(&mut self) -> TMRB6TRIG_W {
        TMRB6TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B6 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb6lmt(&mut self) -> TMRB6LMT_W {
        TMRB6LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A6 Upper compare enable."]
    #[inline(always)]
    pub fn tmra6en23(&mut self) -> TMRA6EN23_W {
        TMRA6EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A6 Upper output polarity"]
    #[inline(always)]
    pub fn tmra6pol23(&mut self) -> TMRA6POL23_W {
        TMRA6POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A6 Invert on trigger."]
    #[inline(always)]
    pub fn tmra6tinv(&mut self) -> TMRA6TINV_W {
        TMRA6TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra6nosync(&mut self) -> TMRA6NOSYNC_W {
        TMRA6NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A6 Trigger Select."]
    #[inline(always)]
    pub fn tmra6trig(&mut self) -> TMRA6TRIG_W {
        TMRA6TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A6 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra6lmt(&mut self) -> TMRA6LMT_W {
        TMRA6LMT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux6](index.html) module"]
pub struct AUX6_SPEC;
impl crate::RegisterSpec for AUX6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux6::R](R) reader structure"]
impl crate::Readable for AUX6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux6::W](W) writer structure"]
impl crate::Writable for AUX6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUX6 to value 0"]
impl crate::Resettable for AUX6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `AUX4` reader"]
pub struct R(crate::R<AUX4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUX4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUX4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUX4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUX4` writer"]
pub struct W(crate::W<AUX4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUX4_SPEC>;
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
impl From<crate::W<AUX4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUX4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer B4 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRB4EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB4EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB4EN23` reader - Counter/Timer B4 Upper compare enable."]
pub struct TMRB4EN23_R(crate::FieldReader<bool, TMRB4EN23_A>);
impl TMRB4EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB4EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB4EN23_A {
        match self.bits {
            true => TMRB4EN23_A::DIS,
            false => TMRB4EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB4EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB4EN23_A::EN
    }
}
impl core::ops::Deref for TMRB4EN23_R {
    type Target = crate::FieldReader<bool, TMRB4EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB4EN23` writer - Counter/Timer B4 Upper compare enable."]
pub struct TMRB4EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB4EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB4EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB4EN23_A::EN)
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
pub enum TMRB4POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRB4POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB4POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB4POL23` reader - Upper output polarity"]
pub struct TMRB4POL23_R(crate::FieldReader<bool, TMRB4POL23_A>);
impl TMRB4POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB4POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB4POL23_A {
        match self.bits {
            false => TMRB4POL23_A::NORM,
            true => TMRB4POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRB4POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRB4POL23_A::INV
    }
}
impl core::ops::Deref for TMRB4POL23_R {
    type Target = crate::FieldReader<bool, TMRB4POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB4POL23` writer - Upper output polarity"]
pub struct TMRB4POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB4POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB4POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRB4POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRB4POL23_A::INV)
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
#[doc = "Counter/Timer B4 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB4TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRB4TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB4TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB4TINV` reader - Counter/Timer B4 Invert on trigger."]
pub struct TMRB4TINV_R(crate::FieldReader<bool, TMRB4TINV_A>);
impl TMRB4TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB4TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB4TINV_A {
        match self.bits {
            false => TMRB4TINV_A::DIS,
            true => TMRB4TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB4TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB4TINV_A::EN
    }
}
impl core::ops::Deref for TMRB4TINV_R {
    type Target = crate::FieldReader<bool, TMRB4TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB4TINV` writer - Counter/Timer B4 Invert on trigger."]
pub struct TMRB4TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB4TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB4TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB4TINV_A::EN)
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
pub enum TMRB4NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRB4NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB4NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB4NOSYNC` reader - Source clock synchronization control."]
pub struct TMRB4NOSYNC_R(crate::FieldReader<bool, TMRB4NOSYNC_A>);
impl TMRB4NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB4NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB4NOSYNC_A {
        match self.bits {
            false => TMRB4NOSYNC_A::DIS,
            true => TMRB4NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB4NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRB4NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRB4NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRB4NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB4NOSYNC` writer - Source clock synchronization control."]
pub struct TMRB4NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB4NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB4NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRB4NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer B4 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB4TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is CTIMERA4 OUT. value."]
    A4OUT = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA7 OUT. value."]
    A7OUT = 4,
    #[doc = "5: Trigger source is CTIMERB7 OUT. value."]
    B7OUT = 5,
    #[doc = "6: Trigger source is CTIMERA1 OUT. value."]
    A1OUT = 6,
    #[doc = "7: Trigger source is CTIMERB1 OUT. value."]
    B1OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL = 15,
}
impl From<TMRB4TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB4TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB4TRIG` reader - Counter/Timer B4 Trigger Select."]
pub struct TMRB4TRIG_R(crate::FieldReader<u8, TMRB4TRIG_A>);
impl TMRB4TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB4TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB4TRIG_A {
        match self.bits {
            0 => TMRB4TRIG_A::DIS,
            1 => TMRB4TRIG_A::A4OUT,
            2 => TMRB4TRIG_A::B3OUT,
            3 => TMRB4TRIG_A::A3OUT,
            4 => TMRB4TRIG_A::A7OUT,
            5 => TMRB4TRIG_A::B7OUT,
            6 => TMRB4TRIG_A::A1OUT,
            7 => TMRB4TRIG_A::B1OUT,
            8 => TMRB4TRIG_A::B3OUT2,
            9 => TMRB4TRIG_A::A3OUT2,
            10 => TMRB4TRIG_A::A1OUT2,
            11 => TMRB4TRIG_A::B1OUT2,
            12 => TMRB4TRIG_A::A6OUT2DUAL,
            13 => TMRB4TRIG_A::A7OUT2DUAL,
            14 => TMRB4TRIG_A::B5OUT2DUAL,
            15 => TMRB4TRIG_A::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB4TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `A4OUT`"]
    #[inline(always)]
    pub fn is_a4out(&self) -> bool {
        **self == TMRB4TRIG_A::A4OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == TMRB4TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == TMRB4TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A7OUT`"]
    #[inline(always)]
    pub fn is_a7out(&self) -> bool {
        **self == TMRB4TRIG_A::A7OUT
    }
    #[doc = "Checks if the value of the field is `B7OUT`"]
    #[inline(always)]
    pub fn is_b7out(&self) -> bool {
        **self == TMRB4TRIG_A::B7OUT
    }
    #[doc = "Checks if the value of the field is `A1OUT`"]
    #[inline(always)]
    pub fn is_a1out(&self) -> bool {
        **self == TMRB4TRIG_A::A1OUT
    }
    #[doc = "Checks if the value of the field is `B1OUT`"]
    #[inline(always)]
    pub fn is_b1out(&self) -> bool {
        **self == TMRB4TRIG_A::B1OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == TMRB4TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == TMRB4TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        **self == TMRB4TRIG_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        **self == TMRB4TRIG_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        **self == TMRB4TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        **self == TMRB4TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b5out2dual(&self) -> bool {
        **self == TMRB4TRIG_A::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a5out2dual(&self) -> bool {
        **self == TMRB4TRIG_A::A5OUT2DUAL
    }
}
impl core::ops::Deref for TMRB4TRIG_R {
    type Target = crate::FieldReader<u8, TMRB4TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB4TRIG` writer - Counter/Timer B4 Trigger Select."]
pub struct TMRB4TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB4TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB4TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::DIS)
    }
    #[doc = "Trigger source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn a4out(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A4OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA7 OUT. value."]
    #[inline(always)]
    pub fn a7out(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A7OUT)
    }
    #[doc = "Trigger source is CTIMERB7 OUT. value."]
    #[inline(always)]
    pub fn b7out(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::B7OUT)
    }
    #[doc = "Trigger source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn a1out(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A1OUT)
    }
    #[doc = "Trigger source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn b1out(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::B1OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRB4TRIG_A::A5OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 23)) | ((value as u32 & 0x0f) << 23);
        self.w
    }
}
#[doc = "Field `TMRB4LMT` reader - Counter/Timer B4 Pattern Limit Count."]
pub struct TMRB4LMT_R(crate::FieldReader<u8, u8>);
impl TMRB4LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB4LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRB4LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB4LMT` writer - Counter/Timer B4 Pattern Limit Count."]
pub struct TMRB4LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB4LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A4 Upper compare enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4EN23_A {
    #[doc = "1: Disable enhanced functions. value."]
    DIS = 1,
    #[doc = "0: Enable enhanced functions. value."]
    EN = 0,
}
impl From<TMRA4EN23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA4EN23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA4EN23` reader - Counter/Timer A4 Upper compare enable."]
pub struct TMRA4EN23_R(crate::FieldReader<bool, TMRA4EN23_A>);
impl TMRA4EN23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA4EN23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA4EN23_A {
        match self.bits {
            true => TMRA4EN23_A::DIS,
            false => TMRA4EN23_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA4EN23_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA4EN23_A::EN
    }
}
impl core::ops::Deref for TMRA4EN23_R {
    type Target = crate::FieldReader<bool, TMRA4EN23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA4EN23` writer - Counter/Timer A4 Upper compare enable."]
pub struct TMRA4EN23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA4EN23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA4EN23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable enhanced functions. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4EN23_A::DIS)
    }
    #[doc = "Enable enhanced functions. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA4EN23_A::EN)
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
#[doc = "Counter/Timer A4 Upper output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4POL23_A {
    #[doc = "0: Upper output normal polarity value."]
    NORM = 0,
    #[doc = "1: Upper output inverted polarity. value."]
    INV = 1,
}
impl From<TMRA4POL23_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA4POL23_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA4POL23` reader - Counter/Timer A4 Upper output polarity"]
pub struct TMRA4POL23_R(crate::FieldReader<bool, TMRA4POL23_A>);
impl TMRA4POL23_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA4POL23_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA4POL23_A {
        match self.bits {
            false => TMRA4POL23_A::NORM,
            true => TMRA4POL23_A::INV,
        }
    }
    #[doc = "Checks if the value of the field is `NORM`"]
    #[inline(always)]
    pub fn is_norm(&self) -> bool {
        **self == TMRA4POL23_A::NORM
    }
    #[doc = "Checks if the value of the field is `INV`"]
    #[inline(always)]
    pub fn is_inv(&self) -> bool {
        **self == TMRA4POL23_A::INV
    }
}
impl core::ops::Deref for TMRA4POL23_R {
    type Target = crate::FieldReader<bool, TMRA4POL23_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA4POL23` writer - Counter/Timer A4 Upper output polarity"]
pub struct TMRA4POL23_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA4POL23_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA4POL23_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Upper output normal polarity value."]
    #[inline(always)]
    pub fn norm(self) -> &'a mut W {
        self.variant(TMRA4POL23_A::NORM)
    }
    #[doc = "Upper output inverted polarity. value."]
    #[inline(always)]
    pub fn inv(self) -> &'a mut W {
        self.variant(TMRA4POL23_A::INV)
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
#[doc = "Counter/Timer A4 Invert on trigger.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA4TINV_A {
    #[doc = "0: Disable invert on trigger value."]
    DIS = 0,
    #[doc = "1: Enable invert on trigger value."]
    EN = 1,
}
impl From<TMRA4TINV_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA4TINV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA4TINV` reader - Counter/Timer A4 Invert on trigger."]
pub struct TMRA4TINV_R(crate::FieldReader<bool, TMRA4TINV_A>);
impl TMRA4TINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA4TINV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA4TINV_A {
        match self.bits {
            false => TMRA4TINV_A::DIS,
            true => TMRA4TINV_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA4TINV_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA4TINV_A::EN
    }
}
impl core::ops::Deref for TMRA4TINV_R {
    type Target = crate::FieldReader<bool, TMRA4TINV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA4TINV` writer - Counter/Timer A4 Invert on trigger."]
pub struct TMRA4TINV_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA4TINV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA4TINV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable invert on trigger value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4TINV_A::DIS)
    }
    #[doc = "Enable invert on trigger value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA4TINV_A::EN)
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
pub enum TMRA4NOSYNC_A {
    #[doc = "0: Synchronization on source clock value."]
    DIS = 0,
    #[doc = "1: No synchronization on source clock value."]
    NOSYNC = 1,
}
impl From<TMRA4NOSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA4NOSYNC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA4NOSYNC` reader - Source clock synchronization control."]
pub struct TMRA4NOSYNC_R(crate::FieldReader<bool, TMRA4NOSYNC_A>);
impl TMRA4NOSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA4NOSYNC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA4NOSYNC_A {
        match self.bits {
            false => TMRA4NOSYNC_A::DIS,
            true => TMRA4NOSYNC_A::NOSYNC,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA4NOSYNC_A::DIS
    }
    #[doc = "Checks if the value of the field is `NOSYNC`"]
    #[inline(always)]
    pub fn is_nosync(&self) -> bool {
        **self == TMRA4NOSYNC_A::NOSYNC
    }
}
impl core::ops::Deref for TMRA4NOSYNC_R {
    type Target = crate::FieldReader<bool, TMRA4NOSYNC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA4NOSYNC` writer - Source clock synchronization control."]
pub struct TMRA4NOSYNC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA4NOSYNC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA4NOSYNC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Synchronization on source clock value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4NOSYNC_A::DIS)
    }
    #[doc = "No synchronization on source clock value."]
    #[inline(always)]
    pub fn nosync(self) -> &'a mut W {
        self.variant(TMRA4NOSYNC_A::NOSYNC)
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
#[doc = "Counter/Timer A4 Trigger Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA4TRIG_A {
    #[doc = "0: Trigger source is disabled. value."]
    DIS = 0,
    #[doc = "1: Trigger source is STimer Interrupt.  Only Active When CTLINK==1 and TMRB4TRIG!=0.  TMRB4TRIG selects an STIMER interrupt value."]
    STIMER = 1,
    #[doc = "2: Trigger source is CTIMERB3 OUT. value."]
    B3OUT = 2,
    #[doc = "3: Trigger source is CTIMERA3 OUT. value."]
    A3OUT = 3,
    #[doc = "4: Trigger source is CTIMERA6 OUT. value."]
    A6OUT = 4,
    #[doc = "5: Trigger source is CTIMERB6 OUT. value."]
    B6OUT = 5,
    #[doc = "6: Trigger source is CTIMERA2 OUT. value."]
    A2OUT = 6,
    #[doc = "7: Trigger source is CTIMERB2 OUT. value."]
    B2OUT = 7,
    #[doc = "8: Trigger source is CTIMERB3 OUT2. value."]
    B3OUT2 = 8,
    #[doc = "9: Trigger source is CTIMERA3 OUT2. value."]
    A3OUT2 = 9,
    #[doc = "10: Trigger source is CTIMERA1 OUT2. value."]
    A1OUT2 = 10,
    #[doc = "11: Trigger source is CTIMERB1 OUT2. value."]
    B1OUT2 = 11,
    #[doc = "12: Trigger source is CTIMERA6 OUT2, dual edge. value."]
    A6OUT2DUAL = 12,
    #[doc = "13: Trigger source is CTIMERA7 OUT2, dual edge. value."]
    A7OUT2DUAL = 13,
    #[doc = "14: Trigger source is CTIMERB5 OUT2, dual edge. value."]
    B5OUT2DUAL = 14,
    #[doc = "15: Trigger source is CTIMERA5 OUT2, dual edge. value."]
    A5OUT2DUAL = 15,
}
impl From<TMRA4TRIG_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA4TRIG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA4TRIG` reader - Counter/Timer A4 Trigger Select."]
pub struct TMRA4TRIG_R(crate::FieldReader<u8, TMRA4TRIG_A>);
impl TMRA4TRIG_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA4TRIG_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA4TRIG_A {
        match self.bits {
            0 => TMRA4TRIG_A::DIS,
            1 => TMRA4TRIG_A::STIMER,
            2 => TMRA4TRIG_A::B3OUT,
            3 => TMRA4TRIG_A::A3OUT,
            4 => TMRA4TRIG_A::A6OUT,
            5 => TMRA4TRIG_A::B6OUT,
            6 => TMRA4TRIG_A::A2OUT,
            7 => TMRA4TRIG_A::B2OUT,
            8 => TMRA4TRIG_A::B3OUT2,
            9 => TMRA4TRIG_A::A3OUT2,
            10 => TMRA4TRIG_A::A1OUT2,
            11 => TMRA4TRIG_A::B1OUT2,
            12 => TMRA4TRIG_A::A6OUT2DUAL,
            13 => TMRA4TRIG_A::A7OUT2DUAL,
            14 => TMRA4TRIG_A::B5OUT2DUAL,
            15 => TMRA4TRIG_A::A5OUT2DUAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA4TRIG_A::DIS
    }
    #[doc = "Checks if the value of the field is `STIMER`"]
    #[inline(always)]
    pub fn is_stimer(&self) -> bool {
        **self == TMRA4TRIG_A::STIMER
    }
    #[doc = "Checks if the value of the field is `B3OUT`"]
    #[inline(always)]
    pub fn is_b3out(&self) -> bool {
        **self == TMRA4TRIG_A::B3OUT
    }
    #[doc = "Checks if the value of the field is `A3OUT`"]
    #[inline(always)]
    pub fn is_a3out(&self) -> bool {
        **self == TMRA4TRIG_A::A3OUT
    }
    #[doc = "Checks if the value of the field is `A6OUT`"]
    #[inline(always)]
    pub fn is_a6out(&self) -> bool {
        **self == TMRA4TRIG_A::A6OUT
    }
    #[doc = "Checks if the value of the field is `B6OUT`"]
    #[inline(always)]
    pub fn is_b6out(&self) -> bool {
        **self == TMRA4TRIG_A::B6OUT
    }
    #[doc = "Checks if the value of the field is `A2OUT`"]
    #[inline(always)]
    pub fn is_a2out(&self) -> bool {
        **self == TMRA4TRIG_A::A2OUT
    }
    #[doc = "Checks if the value of the field is `B2OUT`"]
    #[inline(always)]
    pub fn is_b2out(&self) -> bool {
        **self == TMRA4TRIG_A::B2OUT
    }
    #[doc = "Checks if the value of the field is `B3OUT2`"]
    #[inline(always)]
    pub fn is_b3out2(&self) -> bool {
        **self == TMRA4TRIG_A::B3OUT2
    }
    #[doc = "Checks if the value of the field is `A3OUT2`"]
    #[inline(always)]
    pub fn is_a3out2(&self) -> bool {
        **self == TMRA4TRIG_A::A3OUT2
    }
    #[doc = "Checks if the value of the field is `A1OUT2`"]
    #[inline(always)]
    pub fn is_a1out2(&self) -> bool {
        **self == TMRA4TRIG_A::A1OUT2
    }
    #[doc = "Checks if the value of the field is `B1OUT2`"]
    #[inline(always)]
    pub fn is_b1out2(&self) -> bool {
        **self == TMRA4TRIG_A::B1OUT2
    }
    #[doc = "Checks if the value of the field is `A6OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a6out2dual(&self) -> bool {
        **self == TMRA4TRIG_A::A6OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A7OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a7out2dual(&self) -> bool {
        **self == TMRA4TRIG_A::A7OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `B5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_b5out2dual(&self) -> bool {
        **self == TMRA4TRIG_A::B5OUT2DUAL
    }
    #[doc = "Checks if the value of the field is `A5OUT2DUAL`"]
    #[inline(always)]
    pub fn is_a5out2dual(&self) -> bool {
        **self == TMRA4TRIG_A::A5OUT2DUAL
    }
}
impl core::ops::Deref for TMRA4TRIG_R {
    type Target = crate::FieldReader<u8, TMRA4TRIG_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA4TRIG` writer - Counter/Timer A4 Trigger Select."]
pub struct TMRA4TRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA4TRIG_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA4TRIG_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Trigger source is disabled. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::DIS)
    }
    #[doc = "Trigger source is STimer Interrupt. Only Active When CTLINK==1 and TMRB4TRIG!=0. TMRB4TRIG selects an STIMER interrupt value."]
    #[inline(always)]
    pub fn stimer(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::STIMER)
    }
    #[doc = "Trigger source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn b3out(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::B3OUT)
    }
    #[doc = "Trigger source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn a3out(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A3OUT)
    }
    #[doc = "Trigger source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn a6out(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A6OUT)
    }
    #[doc = "Trigger source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn b6out(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::B6OUT)
    }
    #[doc = "Trigger source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn a2out(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A2OUT)
    }
    #[doc = "Trigger source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn b2out(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::B2OUT)
    }
    #[doc = "Trigger source is CTIMERB3 OUT2. value."]
    #[inline(always)]
    pub fn b3out2(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::B3OUT2)
    }
    #[doc = "Trigger source is CTIMERA3 OUT2. value."]
    #[inline(always)]
    pub fn a3out2(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A3OUT2)
    }
    #[doc = "Trigger source is CTIMERA1 OUT2. value."]
    #[inline(always)]
    pub fn a1out2(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A1OUT2)
    }
    #[doc = "Trigger source is CTIMERB1 OUT2. value."]
    #[inline(always)]
    pub fn b1out2(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::B1OUT2)
    }
    #[doc = "Trigger source is CTIMERA6 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a6out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A6OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA7 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a7out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A7OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERB5 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn b5out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::B5OUT2DUAL)
    }
    #[doc = "Trigger source is CTIMERA5 OUT2, dual edge. value."]
    #[inline(always)]
    pub fn a5out2dual(self) -> &'a mut W {
        self.variant(TMRA4TRIG_A::A5OUT2DUAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 7)) | ((value as u32 & 0x0f) << 7);
        self.w
    }
}
#[doc = "Field `TMRA4LMT` reader - Counter/Timer A4 Pattern Limit Count."]
pub struct TMRA4LMT_R(crate::FieldReader<u8, u8>);
impl TMRA4LMT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA4LMT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMRA4LMT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA4LMT` writer - Counter/Timer A4 Pattern Limit Count."]
pub struct TMRA4LMT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA4LMT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bit 30 - Counter/Timer B4 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb4en23(&self) -> TMRB4EN23_R {
        TMRB4EN23_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb4pol23(&self) -> TMRB4POL23_R {
        TMRB4POL23_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B4 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb4tinv(&self) -> TMRB4TINV_R {
        TMRB4TINV_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb4nosync(&self) -> TMRB4NOSYNC_R {
        TMRB4NOSYNC_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 23:26 - Counter/Timer B4 Trigger Select."]
    #[inline(always)]
    pub fn tmrb4trig(&self) -> TMRB4TRIG_R {
        TMRB4TRIG_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Counter/Timer B4 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb4lmt(&self) -> TMRB4LMT_R {
        TMRB4LMT_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - Counter/Timer A4 Upper compare enable."]
    #[inline(always)]
    pub fn tmra4en23(&self) -> TMRA4EN23_R {
        TMRA4EN23_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Counter/Timer A4 Upper output polarity"]
    #[inline(always)]
    pub fn tmra4pol23(&self) -> TMRA4POL23_R {
        TMRA4POL23_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A4 Invert on trigger."]
    #[inline(always)]
    pub fn tmra4tinv(&self) -> TMRA4TINV_R {
        TMRA4TINV_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra4nosync(&self) -> TMRA4NOSYNC_R {
        TMRA4NOSYNC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 7:10 - Counter/Timer A4 Trigger Select."]
    #[inline(always)]
    pub fn tmra4trig(&self) -> TMRA4TRIG_R {
        TMRA4TRIG_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bits 0:6 - Counter/Timer A4 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra4lmt(&self) -> TMRA4LMT_R {
        TMRA4LMT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 30 - Counter/Timer B4 Upper compare enable."]
    #[inline(always)]
    pub fn tmrb4en23(&mut self) -> TMRB4EN23_W {
        TMRB4EN23_W { w: self }
    }
    #[doc = "Bit 29 - Upper output polarity"]
    #[inline(always)]
    pub fn tmrb4pol23(&mut self) -> TMRB4POL23_W {
        TMRB4POL23_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B4 Invert on trigger."]
    #[inline(always)]
    pub fn tmrb4tinv(&mut self) -> TMRB4TINV_W {
        TMRB4TINV_W { w: self }
    }
    #[doc = "Bit 27 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmrb4nosync(&mut self) -> TMRB4NOSYNC_W {
        TMRB4NOSYNC_W { w: self }
    }
    #[doc = "Bits 23:26 - Counter/Timer B4 Trigger Select."]
    #[inline(always)]
    pub fn tmrb4trig(&mut self) -> TMRB4TRIG_W {
        TMRB4TRIG_W { w: self }
    }
    #[doc = "Bits 16:21 - Counter/Timer B4 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmrb4lmt(&mut self) -> TMRB4LMT_W {
        TMRB4LMT_W { w: self }
    }
    #[doc = "Bit 14 - Counter/Timer A4 Upper compare enable."]
    #[inline(always)]
    pub fn tmra4en23(&mut self) -> TMRA4EN23_W {
        TMRA4EN23_W { w: self }
    }
    #[doc = "Bit 13 - Counter/Timer A4 Upper output polarity"]
    #[inline(always)]
    pub fn tmra4pol23(&mut self) -> TMRA4POL23_W {
        TMRA4POL23_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A4 Invert on trigger."]
    #[inline(always)]
    pub fn tmra4tinv(&mut self) -> TMRA4TINV_W {
        TMRA4TINV_W { w: self }
    }
    #[doc = "Bit 11 - Source clock synchronization control."]
    #[inline(always)]
    pub fn tmra4nosync(&mut self) -> TMRA4NOSYNC_W {
        TMRA4NOSYNC_W { w: self }
    }
    #[doc = "Bits 7:10 - Counter/Timer A4 Trigger Select."]
    #[inline(always)]
    pub fn tmra4trig(&mut self) -> TMRA4TRIG_W {
        TMRA4TRIG_W { w: self }
    }
    #[doc = "Bits 0:6 - Counter/Timer A4 Pattern Limit Count."]
    #[inline(always)]
    pub fn tmra4lmt(&mut self) -> TMRA4LMT_W {
        TMRA4LMT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Auxiliary\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aux4](index.html) module"]
pub struct AUX4_SPEC;
impl crate::RegisterSpec for AUX4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aux4::R](R) reader structure"]
impl crate::Readable for AUX4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aux4::W](W) writer structure"]
impl crate::Writable for AUX4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUX4 to value 0"]
impl crate::Resettable for AUX4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

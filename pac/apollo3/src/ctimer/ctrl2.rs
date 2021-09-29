#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer A2/B2 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK2_A {
    #[doc = "0: Use A2/B2 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A2/B2 timers into a single 32-bit timer. value."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK2_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLINK2` reader - Counter/Timer A2/B2 Link bit."]
pub struct CTLINK2_R(crate::FieldReader<bool, CTLINK2_A>);
impl CTLINK2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTLINK2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK2_A {
        match self.bits {
            false => CTLINK2_A::TWO_16BIT_TIMERS,
            true => CTLINK2_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        **self == CTLINK2_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        **self == CTLINK2_A::_32BIT_TIMER
    }
}
impl core::ops::Deref for CTLINK2_R {
    type Target = crate::FieldReader<bool, CTLINK2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTLINK2` writer - Counter/Timer A2/B2 Link bit."]
pub struct CTLINK2_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use A2/B2 timers as two independent 16-bit timers (default). value."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK2_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A2/B2 timers into a single 32-bit timer. value."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK2_A::_32BIT_TIMER)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
#[doc = "Counter/Timer B2 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2POL_A {
    #[doc = "0: The polarity of the TMRPINB2 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB2 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRB2POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB2POL` reader - Counter/Timer B2 output polarity."]
pub struct TMRB2POL_R(crate::FieldReader<bool, TMRB2POL_A>);
impl TMRB2POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB2POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2POL_A {
        match self.bits {
            false => TMRB2POL_A::NORMAL,
            true => TMRB2POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRB2POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRB2POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRB2POL_R {
    type Target = crate::FieldReader<bool, TMRB2POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB2POL` writer - Counter/Timer B2 output polarity."]
pub struct TMRB2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINB2 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB2POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB2 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB2POL_A::INVERTED)
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
#[doc = "Counter/Timer B2 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2CLR_A {
    #[doc = "0: Allow counter/timer B2 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer B2 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRB2CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB2CLR` reader - Counter/Timer B2 Clear bit."]
pub struct TMRB2CLR_R(crate::FieldReader<bool, TMRB2CLR_A>);
impl TMRB2CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB2CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2CLR_A {
        match self.bits {
            false => TMRB2CLR_A::RUN,
            true => TMRB2CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRB2CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRB2CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRB2CLR_R {
    type Target = crate::FieldReader<bool, TMRB2CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB2CLR` writer - Counter/Timer B2 Clear bit."]
pub struct TMRB2CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer B2 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB2CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B2 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB2CLR_A::CLEAR)
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
#[doc = "Counter/Timer B2 Interrupt Enable bit for COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2IE1_A {
    #[doc = "0: Disable counter/timer B2 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B2 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRB2IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB2IE1` reader - Counter/Timer B2 Interrupt Enable bit for COMPR1."]
pub struct TMRB2IE1_R(crate::FieldReader<bool, TMRB2IE1_A>);
impl TMRB2IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB2IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2IE1_A {
        match self.bits {
            false => TMRB2IE1_A::DIS,
            true => TMRB2IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB2IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB2IE1_A::EN
    }
}
impl core::ops::Deref for TMRB2IE1_R {
    type Target = crate::FieldReader<bool, TMRB2IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB2IE1` writer - Counter/Timer B2 Interrupt Enable bit for COMPR1."]
pub struct TMRB2IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2IE1_A::DIS)
    }
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2IE1_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Counter/Timer B2 Interrupt Enable bit for COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2IE0_A {
    #[doc = "0: Disable counter/timer B2 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B2 to generate an interrupt based on COMPR0 value."]
    EN = 1,
}
impl From<TMRB2IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB2IE0` reader - Counter/Timer B2 Interrupt Enable bit for COMPR0."]
pub struct TMRB2IE0_R(crate::FieldReader<bool, TMRB2IE0_A>);
impl TMRB2IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB2IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2IE0_A {
        match self.bits {
            false => TMRB2IE0_A::DIS,
            true => TMRB2IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB2IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB2IE0_A::EN
    }
}
impl core::ops::Deref for TMRB2IE0_R {
    type Target = crate::FieldReader<bool, TMRB2IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB2IE0` writer - Counter/Timer B2 Interrupt Enable bit for COMPR0."]
pub struct TMRB2IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B2 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2IE0_A::DIS)
    }
    #[doc = "Enable counter/timer B2 to generate an interrupt based on COMPR0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2IE0_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Counter/Timer B2 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB2FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B2, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B2, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B2, assert, count to CMPR1B2, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B2, assert, count to CMPR1B2, deassert, restart. value."]
    PULSE_CONT = 3,
    #[doc = "4: Single pattern. value."]
    SINGLEPATTERN = 4,
    #[doc = "5: Repeated pattern. value."]
    REPEATPATTERN = 5,
    #[doc = "6: Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS = 6,
    #[doc = "7: Alternate PWM value."]
    ALTPWN = 7,
}
impl From<TMRB2FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB2FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB2FN` reader - Counter/Timer B2 Function Select."]
pub struct TMRB2FN_R(crate::FieldReader<u8, TMRB2FN_A>);
impl TMRB2FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB2FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2FN_A {
        match self.bits {
            0 => TMRB2FN_A::SINGLECOUNT,
            1 => TMRB2FN_A::REPEATEDCOUNT,
            2 => TMRB2FN_A::PULSE_ONCE,
            3 => TMRB2FN_A::PULSE_CONT,
            4 => TMRB2FN_A::SINGLEPATTERN,
            5 => TMRB2FN_A::REPEATPATTERN,
            6 => TMRB2FN_A::CONTINUOUS,
            7 => TMRB2FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRB2FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRB2FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRB2FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRB2FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRB2FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRB2FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRB2FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRB2FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRB2FN_R {
    type Target = crate::FieldReader<u8, TMRB2FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB2FN` writer - Counter/Timer B2 Function Select."]
pub struct TMRB2FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B2, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB2FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B2, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB2FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B2, assert, count to CMPR1B2, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB2FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B2, assert, count to CMPR1B2, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB2FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB2FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB2FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB2FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB2FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B2 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB2CLK_A {
    #[doc = "0: Clock source is TMRPINB. value."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC / 4 value."]
    HFRC_DIV4 = 1,
    #[doc = "2: Clock source is HFRC / 16 value."]
    HFRC_DIV16 = 2,
    #[doc = "3: Clock source is HFRC / 256 value."]
    HFRC_DIV256 = 3,
    #[doc = "4: Clock source is HFRC / 1024 value."]
    HFRC_DIV1024 = 4,
    #[doc = "5: Clock source is HFRC / 4096 value."]
    HFRC_DIV4K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated). value."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2 value."]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16 value."]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 128 value."]
    XT_DIV128 = 9,
    #[doc = "10: Clock source is LFRC / 2 value."]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32 value."]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024 value."]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC value."]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4 = 15,
    #[doc = "16: Clock source is XT / 4 value."]
    XT_DIV4 = 16,
    #[doc = "17: Clock source is XT / 8 value."]
    XT_DIV8 = 17,
    #[doc = "18: Clock source is XT / 32 value."]
    XT_DIV32 = 18,
    #[doc = "20: Clock source is CTIMERA2 OUT. value."]
    CTMRA2 = 20,
    #[doc = "21: Clock source is CTIMERA3 OUT. value."]
    CTMRB3 = 21,
    #[doc = "22: Clock source is CTIMERB3 OUT. value."]
    CTMRA3 = 22,
    #[doc = "23: Clock source is CTIMERA4 OUT. value."]
    CTMRA4 = 23,
    #[doc = "24: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 24,
    #[doc = "25: Clock source is CTIMERB0 OUT. value."]
    CTMRB0 = 25,
    #[doc = "26: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 26,
    #[doc = "27: Clock source is CTIMERB5 OUT. value."]
    CTMRB5 = 27,
    #[doc = "28: Clock source is CTIMERB6 OUT. value."]
    CTMRB6 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses. value."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses. value."]
    BUCKA = 31,
}
impl From<TMRB2CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB2CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB2CLK` reader - Counter/Timer B2 Clock Select."]
pub struct TMRB2CLK_R(crate::FieldReader<u8, TMRB2CLK_A>);
impl TMRB2CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB2CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRB2CLK_A> {
        match self.bits {
            0 => Some(TMRB2CLK_A::TMRPIN),
            1 => Some(TMRB2CLK_A::HFRC_DIV4),
            2 => Some(TMRB2CLK_A::HFRC_DIV16),
            3 => Some(TMRB2CLK_A::HFRC_DIV256),
            4 => Some(TMRB2CLK_A::HFRC_DIV1024),
            5 => Some(TMRB2CLK_A::HFRC_DIV4K),
            6 => Some(TMRB2CLK_A::XT),
            7 => Some(TMRB2CLK_A::XT_DIV2),
            8 => Some(TMRB2CLK_A::XT_DIV16),
            9 => Some(TMRB2CLK_A::XT_DIV128),
            10 => Some(TMRB2CLK_A::LFRC_DIV2),
            11 => Some(TMRB2CLK_A::LFRC_DIV32),
            12 => Some(TMRB2CLK_A::LFRC_DIV1K),
            13 => Some(TMRB2CLK_A::LFRC),
            14 => Some(TMRB2CLK_A::RTC_100HZ),
            15 => Some(TMRB2CLK_A::HCLK_DIV4),
            16 => Some(TMRB2CLK_A::XT_DIV4),
            17 => Some(TMRB2CLK_A::XT_DIV8),
            18 => Some(TMRB2CLK_A::XT_DIV32),
            20 => Some(TMRB2CLK_A::CTMRA2),
            21 => Some(TMRB2CLK_A::CTMRB3),
            22 => Some(TMRB2CLK_A::CTMRA3),
            23 => Some(TMRB2CLK_A::CTMRA4),
            24 => Some(TMRB2CLK_A::CTMRB4),
            25 => Some(TMRB2CLK_A::CTMRB0),
            26 => Some(TMRB2CLK_A::CTMRB1),
            27 => Some(TMRB2CLK_A::CTMRB5),
            28 => Some(TMRB2CLK_A::CTMRB6),
            29 => Some(TMRB2CLK_A::BUCKBLE),
            30 => Some(TMRB2CLK_A::BUCKB),
            31 => Some(TMRB2CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRB2CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRB2CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRB2CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRB2CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRB2CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRB2CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRB2CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRB2CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRB2CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRB2CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRB2CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRB2CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRB2CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRB2CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRB2CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRB2CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRB2CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRB2CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRB2CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline(always)]
    pub fn is_ctmra2(&self) -> bool {
        **self == TMRB2CLK_A::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        **self == TMRB2CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline(always)]
    pub fn is_ctmra3(&self) -> bool {
        **self == TMRB2CLK_A::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline(always)]
    pub fn is_ctmra4(&self) -> bool {
        **self == TMRB2CLK_A::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRB2CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        **self == TMRB2CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRB2CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        **self == TMRB2CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        **self == TMRB2CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRB2CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRB2CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRB2CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRB2CLK_R {
    type Target = crate::FieldReader<u8, TMRB2CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB2CLK` writer - Counter/Timer B2 Clock Select."]
pub struct TMRB2CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRA2)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRA3)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB2CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B2 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB2EN_A {
    #[doc = "0: Counter/Timer B2 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer B2 Enable. value."]
    EN = 1,
}
impl From<TMRB2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB2EN` reader - Counter/Timer B2 Enable bit."]
pub struct TMRB2EN_R(crate::FieldReader<bool, TMRB2EN_A>);
impl TMRB2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB2EN_A {
        match self.bits {
            false => TMRB2EN_A::DIS,
            true => TMRB2EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB2EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB2EN_A::EN
    }
}
impl core::ops::Deref for TMRB2EN_R {
    type Target = crate::FieldReader<bool, TMRB2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB2EN` writer - Counter/Timer B2 Enable bit."]
pub struct TMRB2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer B2 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB2EN_A::DIS)
    }
    #[doc = "Counter/Timer B2 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB2EN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Counter/Timer A2 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2POL_A {
    #[doc = "0: The polarity of the TMRPINA2 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA2 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRA2POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA2POL` reader - Counter/Timer A2 output polarity."]
pub struct TMRA2POL_R(crate::FieldReader<bool, TMRA2POL_A>);
impl TMRA2POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA2POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2POL_A {
        match self.bits {
            false => TMRA2POL_A::NORMAL,
            true => TMRA2POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRA2POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRA2POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRA2POL_R {
    type Target = crate::FieldReader<bool, TMRA2POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA2POL` writer - Counter/Timer A2 output polarity."]
pub struct TMRA2POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINA2 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA2POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA2 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA2POL_A::INVERTED)
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
#[doc = "Counter/Timer A2 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2CLR_A {
    #[doc = "0: Allow counter/timer A2 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer A2 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRA2CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA2CLR` reader - Counter/Timer A2 Clear bit."]
pub struct TMRA2CLR_R(crate::FieldReader<bool, TMRA2CLR_A>);
impl TMRA2CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA2CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2CLR_A {
        match self.bits {
            false => TMRA2CLR_A::RUN,
            true => TMRA2CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRA2CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRA2CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRA2CLR_R {
    type Target = crate::FieldReader<bool, TMRA2CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA2CLR` writer - Counter/Timer A2 Clear bit."]
pub struct TMRA2CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer A2 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA2CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A2 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA2CLR_A::CLEAR)
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
#[doc = "Counter/Timer A2 Interrupt Enable bit based on COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2IE1_A {
    #[doc = "0: Disable counter/timer A2 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A2 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRA2IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA2IE1` reader - Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
pub struct TMRA2IE1_R(crate::FieldReader<bool, TMRA2IE1_A>);
impl TMRA2IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA2IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2IE1_A {
        match self.bits {
            false => TMRA2IE1_A::DIS,
            true => TMRA2IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA2IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA2IE1_A::EN
    }
}
impl core::ops::Deref for TMRA2IE1_R {
    type Target = crate::FieldReader<bool, TMRA2IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA2IE1` writer - Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
pub struct TMRA2IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2IE1_A::DIS)
    }
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2IE1_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Counter/Timer A2 Interrupt Enable bit based on COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2IE0_A {
    #[doc = "0: Disable counter/timer A2 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A2 to generate an interrupt based on COMPR0. value."]
    EN = 1,
}
impl From<TMRA2IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA2IE0` reader - Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
pub struct TMRA2IE0_R(crate::FieldReader<bool, TMRA2IE0_A>);
impl TMRA2IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA2IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2IE0_A {
        match self.bits {
            false => TMRA2IE0_A::DIS,
            true => TMRA2IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA2IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA2IE0_A::EN
    }
}
impl core::ops::Deref for TMRA2IE0_R {
    type Target = crate::FieldReader<bool, TMRA2IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA2IE0` writer - Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
pub struct TMRA2IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A2 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2IE0_A::DIS)
    }
    #[doc = "Enable counter/timer A2 to generate an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2IE0_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Counter/Timer A2 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA2FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A2, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A2, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A2, assert, count to CMPR1A2, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A2, assert, count to CMPR1A2, deassert, restart. value."]
    PULSE_CONT = 3,
    #[doc = "4: Single pattern. value."]
    SINGLEPATTERN = 4,
    #[doc = "5: Repeated pattern. value."]
    REPEATPATTERN = 5,
    #[doc = "6: Continuous run (aka Free Run).  Count continuously. value."]
    CONTINUOUS = 6,
    #[doc = "7: Alternate PWM value."]
    ALTPWN = 7,
}
impl From<TMRA2FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA2FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA2FN` reader - Counter/Timer A2 Function Select."]
pub struct TMRA2FN_R(crate::FieldReader<u8, TMRA2FN_A>);
impl TMRA2FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA2FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2FN_A {
        match self.bits {
            0 => TMRA2FN_A::SINGLECOUNT,
            1 => TMRA2FN_A::REPEATEDCOUNT,
            2 => TMRA2FN_A::PULSE_ONCE,
            3 => TMRA2FN_A::PULSE_CONT,
            4 => TMRA2FN_A::SINGLEPATTERN,
            5 => TMRA2FN_A::REPEATPATTERN,
            6 => TMRA2FN_A::CONTINUOUS,
            7 => TMRA2FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRA2FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRA2FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRA2FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRA2FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRA2FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRA2FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRA2FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRA2FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRA2FN_R {
    type Target = crate::FieldReader<u8, TMRA2FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA2FN` writer - Counter/Timer A2 Function Select."]
pub struct TMRA2FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A2, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA2FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A2, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA2FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A2, assert, count to CMPR1A2, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA2FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A2, assert, count to CMPR1A2, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA2FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA2FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA2FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA2FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA2FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A2 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA2CLK_A {
    #[doc = "0: Clock source is TMRPINA. value."]
    TMRPIN = 0,
    #[doc = "1: Clock source is the HFRC / 4 value."]
    HFRC_DIV4 = 1,
    #[doc = "2: Clock source is HFRC / 16 value."]
    HFRC_DIV16 = 2,
    #[doc = "3: Clock source is HFRC / 256 value."]
    HFRC_DIV256 = 3,
    #[doc = "4: Clock source is HFRC / 1024 value."]
    HFRC_DIV1024 = 4,
    #[doc = "5: Clock source is HFRC / 4096 value."]
    HFRC_DIV4K = 5,
    #[doc = "6: Clock source is the XT (uncalibrated). value."]
    XT = 6,
    #[doc = "7: Clock source is XT / 2 value."]
    XT_DIV2 = 7,
    #[doc = "8: Clock source is XT / 16 value."]
    XT_DIV16 = 8,
    #[doc = "9: Clock source is XT / 128 value."]
    XT_DIV128 = 9,
    #[doc = "10: Clock source is LFRC / 2 value."]
    LFRC_DIV2 = 10,
    #[doc = "11: Clock source is LFRC / 32 value."]
    LFRC_DIV32 = 11,
    #[doc = "12: Clock source is LFRC / 1024 value."]
    LFRC_DIV1K = 12,
    #[doc = "13: Clock source is LFRC value."]
    LFRC = 13,
    #[doc = "14: Clock source is 100 Hz from the current RTC oscillator. value."]
    RTC_100HZ = 14,
    #[doc = "15: Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    HCLK_DIV4 = 15,
    #[doc = "16: Clock source is XT / 4 value."]
    XT_DIV4 = 16,
    #[doc = "17: Clock source is XT / 8 value."]
    XT_DIV8 = 17,
    #[doc = "18: Clock source is XT / 32 value."]
    XT_DIV32 = 18,
    #[doc = "20: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 20,
    #[doc = "21: Clock source is CTIMERA3 OUT. value."]
    CTMRB3 = 21,
    #[doc = "22: Clock source is CTIMERB3 OUT. value."]
    CTMRA3 = 22,
    #[doc = "23: Clock source is CTIMERA4 OUT. value."]
    CTMRA4 = 23,
    #[doc = "24: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 24,
    #[doc = "25: Clock source is CTIMERB0 OUT. value."]
    CTMRB0 = 25,
    #[doc = "26: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 26,
    #[doc = "27: Clock source is CTIMERB5 OUT. value."]
    CTMRB5 = 27,
    #[doc = "28: Clock source is CTIMERB6 OUT. value."]
    CTMRB6 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses. value."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses. value."]
    BUCKA = 31,
}
impl From<TMRA2CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA2CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA2CLK` reader - Counter/Timer A2 Clock Select."]
pub struct TMRA2CLK_R(crate::FieldReader<u8, TMRA2CLK_A>);
impl TMRA2CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA2CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRA2CLK_A> {
        match self.bits {
            0 => Some(TMRA2CLK_A::TMRPIN),
            1 => Some(TMRA2CLK_A::HFRC_DIV4),
            2 => Some(TMRA2CLK_A::HFRC_DIV16),
            3 => Some(TMRA2CLK_A::HFRC_DIV256),
            4 => Some(TMRA2CLK_A::HFRC_DIV1024),
            5 => Some(TMRA2CLK_A::HFRC_DIV4K),
            6 => Some(TMRA2CLK_A::XT),
            7 => Some(TMRA2CLK_A::XT_DIV2),
            8 => Some(TMRA2CLK_A::XT_DIV16),
            9 => Some(TMRA2CLK_A::XT_DIV128),
            10 => Some(TMRA2CLK_A::LFRC_DIV2),
            11 => Some(TMRA2CLK_A::LFRC_DIV32),
            12 => Some(TMRA2CLK_A::LFRC_DIV1K),
            13 => Some(TMRA2CLK_A::LFRC),
            14 => Some(TMRA2CLK_A::RTC_100HZ),
            15 => Some(TMRA2CLK_A::HCLK_DIV4),
            16 => Some(TMRA2CLK_A::XT_DIV4),
            17 => Some(TMRA2CLK_A::XT_DIV8),
            18 => Some(TMRA2CLK_A::XT_DIV32),
            20 => Some(TMRA2CLK_A::CTMRB2),
            21 => Some(TMRA2CLK_A::CTMRB3),
            22 => Some(TMRA2CLK_A::CTMRA3),
            23 => Some(TMRA2CLK_A::CTMRA4),
            24 => Some(TMRA2CLK_A::CTMRB4),
            25 => Some(TMRA2CLK_A::CTMRB0),
            26 => Some(TMRA2CLK_A::CTMRB1),
            27 => Some(TMRA2CLK_A::CTMRB5),
            28 => Some(TMRA2CLK_A::CTMRB6),
            29 => Some(TMRA2CLK_A::BUCKBLE),
            30 => Some(TMRA2CLK_A::BUCKB),
            31 => Some(TMRA2CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRA2CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRA2CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRA2CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRA2CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRA2CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRA2CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRA2CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRA2CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRA2CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRA2CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRA2CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRA2CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRA2CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRA2CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRA2CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRA2CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRA2CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRA2CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRA2CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        **self == TMRA2CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        **self == TMRA2CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline(always)]
    pub fn is_ctmra3(&self) -> bool {
        **self == TMRA2CLK_A::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline(always)]
    pub fn is_ctmra4(&self) -> bool {
        **self == TMRA2CLK_A::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRA2CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        **self == TMRA2CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRA2CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        **self == TMRA2CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        **self == TMRA2CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRA2CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRA2CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRA2CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRA2CLK_R {
    type Target = crate::FieldReader<u8, TMRA2CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA2CLK` writer - Counter/Timer A2 Clock Select."]
pub struct TMRA2CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRA3)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA2CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A2 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA2EN_A {
    #[doc = "0: Counter/Timer A2 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer A2 Enable. value."]
    EN = 1,
}
impl From<TMRA2EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA2EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA2EN` reader - Counter/Timer A2 Enable bit."]
pub struct TMRA2EN_R(crate::FieldReader<bool, TMRA2EN_A>);
impl TMRA2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA2EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA2EN_A {
        match self.bits {
            false => TMRA2EN_A::DIS,
            true => TMRA2EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA2EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA2EN_A::EN
    }
}
impl core::ops::Deref for TMRA2EN_R {
    type Target = crate::FieldReader<bool, TMRA2EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA2EN` writer - Counter/Timer A2 Enable bit."]
pub struct TMRA2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA2EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA2EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer A2 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA2EN_A::DIS)
    }
    #[doc = "Counter/Timer A2 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA2EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A2/B2 Link bit."]
    #[inline(always)]
    pub fn ctlink2(&self) -> CTLINK2_R {
        CTLINK2_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B2 output polarity."]
    #[inline(always)]
    pub fn tmrb2pol(&self) -> TMRB2POL_R {
        TMRB2POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B2 Clear bit."]
    #[inline(always)]
    pub fn tmrb2clr(&self) -> TMRB2CLR_R {
        TMRB2CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B2 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb2ie1(&self) -> TMRB2IE1_R {
        TMRB2IE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B2 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb2ie0(&self) -> TMRB2IE0_R {
        TMRB2IE0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B2 Function Select."]
    #[inline(always)]
    pub fn tmrb2fn(&self) -> TMRB2FN_R {
        TMRB2FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B2 Clock Select."]
    #[inline(always)]
    pub fn tmrb2clk(&self) -> TMRB2CLK_R {
        TMRB2CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B2 Enable bit."]
    #[inline(always)]
    pub fn tmrb2en(&self) -> TMRB2EN_R {
        TMRB2EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A2 output polarity."]
    #[inline(always)]
    pub fn tmra2pol(&self) -> TMRA2POL_R {
        TMRA2POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A2 Clear bit."]
    #[inline(always)]
    pub fn tmra2clr(&self) -> TMRA2CLR_R {
        TMRA2CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra2ie1(&self) -> TMRA2IE1_R {
        TMRA2IE1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra2ie0(&self) -> TMRA2IE0_R {
        TMRA2IE0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A2 Function Select."]
    #[inline(always)]
    pub fn tmra2fn(&self) -> TMRA2FN_R {
        TMRA2FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A2 Clock Select."]
    #[inline(always)]
    pub fn tmra2clk(&self) -> TMRA2CLK_R {
        TMRA2CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A2 Enable bit."]
    #[inline(always)]
    pub fn tmra2en(&self) -> TMRA2EN_R {
        TMRA2EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A2/B2 Link bit."]
    #[inline(always)]
    pub fn ctlink2(&mut self) -> CTLINK2_W {
        CTLINK2_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B2 output polarity."]
    #[inline(always)]
    pub fn tmrb2pol(&mut self) -> TMRB2POL_W {
        TMRB2POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B2 Clear bit."]
    #[inline(always)]
    pub fn tmrb2clr(&mut self) -> TMRB2CLR_W {
        TMRB2CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B2 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb2ie1(&mut self) -> TMRB2IE1_W {
        TMRB2IE1_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B2 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb2ie0(&mut self) -> TMRB2IE0_W {
        TMRB2IE0_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B2 Function Select."]
    #[inline(always)]
    pub fn tmrb2fn(&mut self) -> TMRB2FN_W {
        TMRB2FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B2 Clock Select."]
    #[inline(always)]
    pub fn tmrb2clk(&mut self) -> TMRB2CLK_W {
        TMRB2CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B2 Enable bit."]
    #[inline(always)]
    pub fn tmrb2en(&mut self) -> TMRB2EN_W {
        TMRB2EN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A2 output polarity."]
    #[inline(always)]
    pub fn tmra2pol(&mut self) -> TMRA2POL_W {
        TMRA2POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A2 Clear bit."]
    #[inline(always)]
    pub fn tmra2clr(&mut self) -> TMRA2CLR_W {
        TMRA2CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A2 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra2ie1(&mut self) -> TMRA2IE1_W {
        TMRA2IE1_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A2 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra2ie0(&mut self) -> TMRA2IE0_W {
        TMRA2IE0_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A2 Function Select."]
    #[inline(always)]
    pub fn tmra2fn(&mut self) -> TMRA2FN_W {
        TMRA2FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A2 Clock Select."]
    #[inline(always)]
    pub fn tmra2clk(&mut self) -> TMRA2CLK_W {
        TMRA2CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A2 Enable bit."]
    #[inline(always)]
    pub fn tmra2en(&mut self) -> TMRA2EN_W {
        TMRA2EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `CTRL3` reader"]
pub struct R(crate::R<CTRL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL3` writer"]
pub struct W(crate::W<CTRL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL3_SPEC>;
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
impl From<crate::W<CTRL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer A3/B3 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK3_A {
    #[doc = "0: Use A3/B3 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A3/B3 timers into a single 32-bit timer. value."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK3_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLINK3` reader - Counter/Timer A3/B3 Link bit."]
pub struct CTLINK3_R(crate::FieldReader<bool, CTLINK3_A>);
impl CTLINK3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTLINK3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK3_A {
        match self.bits {
            false => CTLINK3_A::TWO_16BIT_TIMERS,
            true => CTLINK3_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        **self == CTLINK3_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        **self == CTLINK3_A::_32BIT_TIMER
    }
}
impl core::ops::Deref for CTLINK3_R {
    type Target = crate::FieldReader<bool, CTLINK3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTLINK3` writer - Counter/Timer A3/B3 Link bit."]
pub struct CTLINK3_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use A3/B3 timers as two independent 16-bit timers (default). value."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK3_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A3/B3 timers into a single 32-bit timer. value."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK3_A::_32BIT_TIMER)
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
#[doc = "Counter/Timer B3 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3POL_A {
    #[doc = "0: The polarity of the TMRPINB3 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB3 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRB3POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3POL` reader - Counter/Timer B3 output polarity."]
pub struct TMRB3POL_R(crate::FieldReader<bool, TMRB3POL_A>);
impl TMRB3POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3POL_A {
        match self.bits {
            false => TMRB3POL_A::NORMAL,
            true => TMRB3POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRB3POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRB3POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRB3POL_R {
    type Target = crate::FieldReader<bool, TMRB3POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3POL` writer - Counter/Timer B3 output polarity."]
pub struct TMRB3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINB3 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB3POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB3 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB3POL_A::INVERTED)
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
#[doc = "Counter/Timer B3 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3CLR_A {
    #[doc = "0: Allow counter/timer B3 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer B3 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRB3CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3CLR` reader - Counter/Timer B3 Clear bit."]
pub struct TMRB3CLR_R(crate::FieldReader<bool, TMRB3CLR_A>);
impl TMRB3CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3CLR_A {
        match self.bits {
            false => TMRB3CLR_A::RUN,
            true => TMRB3CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRB3CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRB3CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRB3CLR_R {
    type Target = crate::FieldReader<bool, TMRB3CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3CLR` writer - Counter/Timer B3 Clear bit."]
pub struct TMRB3CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer B3 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB3CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B3 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB3CLR_A::CLEAR)
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
#[doc = "Counter/Timer B3 Interrupt Enable bit for COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3IE1_A {
    #[doc = "0: Disable counter/timer B3 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B3 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRB3IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3IE1` reader - Counter/Timer B3 Interrupt Enable bit for COMPR1."]
pub struct TMRB3IE1_R(crate::FieldReader<bool, TMRB3IE1_A>);
impl TMRB3IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3IE1_A {
        match self.bits {
            false => TMRB3IE1_A::DIS,
            true => TMRB3IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB3IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB3IE1_A::EN
    }
}
impl core::ops::Deref for TMRB3IE1_R {
    type Target = crate::FieldReader<bool, TMRB3IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3IE1` writer - Counter/Timer B3 Interrupt Enable bit for COMPR1."]
pub struct TMRB3IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3IE1_A::DIS)
    }
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3IE1_A::EN)
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
#[doc = "Counter/Timer B3 Interrupt Enable bit for COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3IE0_A {
    #[doc = "0: Disable counter/timer B3 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B3 to generate an interrupt based on COMPR0 value."]
    EN = 1,
}
impl From<TMRB3IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3IE0` reader - Counter/Timer B3 Interrupt Enable bit for COMPR0."]
pub struct TMRB3IE0_R(crate::FieldReader<bool, TMRB3IE0_A>);
impl TMRB3IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3IE0_A {
        match self.bits {
            false => TMRB3IE0_A::DIS,
            true => TMRB3IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB3IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB3IE0_A::EN
    }
}
impl core::ops::Deref for TMRB3IE0_R {
    type Target = crate::FieldReader<bool, TMRB3IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3IE0` writer - Counter/Timer B3 Interrupt Enable bit for COMPR0."]
pub struct TMRB3IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B3 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3IE0_A::DIS)
    }
    #[doc = "Enable counter/timer B3 to generate an interrupt based on COMPR0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3IE0_A::EN)
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
#[doc = "Counter/Timer B3 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB3FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B3, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B3, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B3, assert, count to CMPR1B3, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart. value."]
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
impl From<TMRB3FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB3FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB3FN` reader - Counter/Timer B3 Function Select."]
pub struct TMRB3FN_R(crate::FieldReader<u8, TMRB3FN_A>);
impl TMRB3FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB3FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3FN_A {
        match self.bits {
            0 => TMRB3FN_A::SINGLECOUNT,
            1 => TMRB3FN_A::REPEATEDCOUNT,
            2 => TMRB3FN_A::PULSE_ONCE,
            3 => TMRB3FN_A::PULSE_CONT,
            4 => TMRB3FN_A::SINGLEPATTERN,
            5 => TMRB3FN_A::REPEATPATTERN,
            6 => TMRB3FN_A::CONTINUOUS,
            7 => TMRB3FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRB3FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRB3FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRB3FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRB3FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRB3FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRB3FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRB3FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRB3FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRB3FN_R {
    type Target = crate::FieldReader<u8, TMRB3FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3FN` writer - Counter/Timer B3 Function Select."]
pub struct TMRB3FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B3, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB3FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B3, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB3FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B3, assert, count to CMPR1B3, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB3FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B3, assert, count to CMPR1B3, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB3FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB3FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB3FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB3FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB3FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B3 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB3CLK_A {
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
    #[doc = "20: Clock source is CTIMERA3 OUT. value."]
    CTMRA3 = 20,
    #[doc = "21: Clock source is CTIMERA2 OUT. value."]
    CTMRA2 = 21,
    #[doc = "22: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 22,
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
impl From<TMRB3CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB3CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB3CLK` reader - Counter/Timer B3 Clock Select."]
pub struct TMRB3CLK_R(crate::FieldReader<u8, TMRB3CLK_A>);
impl TMRB3CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB3CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRB3CLK_A> {
        match self.bits {
            0 => Some(TMRB3CLK_A::TMRPIN),
            1 => Some(TMRB3CLK_A::HFRC_DIV4),
            2 => Some(TMRB3CLK_A::HFRC_DIV16),
            3 => Some(TMRB3CLK_A::HFRC_DIV256),
            4 => Some(TMRB3CLK_A::HFRC_DIV1024),
            5 => Some(TMRB3CLK_A::HFRC_DIV4K),
            6 => Some(TMRB3CLK_A::XT),
            7 => Some(TMRB3CLK_A::XT_DIV2),
            8 => Some(TMRB3CLK_A::XT_DIV16),
            9 => Some(TMRB3CLK_A::XT_DIV128),
            10 => Some(TMRB3CLK_A::LFRC_DIV2),
            11 => Some(TMRB3CLK_A::LFRC_DIV32),
            12 => Some(TMRB3CLK_A::LFRC_DIV1K),
            13 => Some(TMRB3CLK_A::LFRC),
            14 => Some(TMRB3CLK_A::RTC_100HZ),
            15 => Some(TMRB3CLK_A::HCLK_DIV4),
            16 => Some(TMRB3CLK_A::XT_DIV4),
            17 => Some(TMRB3CLK_A::XT_DIV8),
            18 => Some(TMRB3CLK_A::XT_DIV32),
            20 => Some(TMRB3CLK_A::CTMRA3),
            21 => Some(TMRB3CLK_A::CTMRA2),
            22 => Some(TMRB3CLK_A::CTMRB2),
            23 => Some(TMRB3CLK_A::CTMRA4),
            24 => Some(TMRB3CLK_A::CTMRB4),
            25 => Some(TMRB3CLK_A::CTMRB0),
            26 => Some(TMRB3CLK_A::CTMRB1),
            27 => Some(TMRB3CLK_A::CTMRB5),
            28 => Some(TMRB3CLK_A::CTMRB6),
            29 => Some(TMRB3CLK_A::BUCKBLE),
            30 => Some(TMRB3CLK_A::BUCKB),
            31 => Some(TMRB3CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRB3CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRB3CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRB3CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRB3CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRB3CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRB3CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRB3CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRB3CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRB3CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRB3CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRB3CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRB3CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRB3CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRB3CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRB3CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRB3CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRB3CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRB3CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRB3CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline(always)]
    pub fn is_ctmra3(&self) -> bool {
        **self == TMRB3CLK_A::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline(always)]
    pub fn is_ctmra2(&self) -> bool {
        **self == TMRB3CLK_A::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        **self == TMRB3CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline(always)]
    pub fn is_ctmra4(&self) -> bool {
        **self == TMRB3CLK_A::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRB3CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        **self == TMRB3CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRB3CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        **self == TMRB3CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        **self == TMRB3CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRB3CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRB3CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRB3CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRB3CLK_R {
    type Target = crate::FieldReader<u8, TMRB3CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3CLK` writer - Counter/Timer B3 Clock Select."]
pub struct TMRB3CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRA3)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB3CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B3 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB3EN_A {
    #[doc = "0: Counter/Timer B3 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer B3 Enable. value."]
    EN = 1,
}
impl From<TMRB3EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB3EN` reader - Counter/Timer B3 Enable bit."]
pub struct TMRB3EN_R(crate::FieldReader<bool, TMRB3EN_A>);
impl TMRB3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB3EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB3EN_A {
        match self.bits {
            false => TMRB3EN_A::DIS,
            true => TMRB3EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB3EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB3EN_A::EN
    }
}
impl core::ops::Deref for TMRB3EN_R {
    type Target = crate::FieldReader<bool, TMRB3EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB3EN` writer - Counter/Timer B3 Enable bit."]
pub struct TMRB3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer B3 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB3EN_A::DIS)
    }
    #[doc = "Counter/Timer B3 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB3EN_A::EN)
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
#[doc = "Field `ADCEN` reader - Special Timer A3 enable for ADC function."]
pub struct ADCEN_R(crate::FieldReader<bool, bool>);
impl ADCEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCEN` writer - Special Timer A3 enable for ADC function."]
pub struct ADCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Counter/Timer A3 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3POL_A {
    #[doc = "0: The polarity of the TMRPINA3 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA3 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRA3POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3POL` reader - Counter/Timer A3 output polarity."]
pub struct TMRA3POL_R(crate::FieldReader<bool, TMRA3POL_A>);
impl TMRA3POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3POL_A {
        match self.bits {
            false => TMRA3POL_A::NORMAL,
            true => TMRA3POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRA3POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRA3POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRA3POL_R {
    type Target = crate::FieldReader<bool, TMRA3POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3POL` writer - Counter/Timer A3 output polarity."]
pub struct TMRA3POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINA3 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA3POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA3 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA3POL_A::INVERTED)
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
#[doc = "Counter/Timer A3 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3CLR_A {
    #[doc = "0: Allow counter/timer A3 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer A3 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRA3CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3CLR` reader - Counter/Timer A3 Clear bit."]
pub struct TMRA3CLR_R(crate::FieldReader<bool, TMRA3CLR_A>);
impl TMRA3CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3CLR_A {
        match self.bits {
            false => TMRA3CLR_A::RUN,
            true => TMRA3CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRA3CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRA3CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRA3CLR_R {
    type Target = crate::FieldReader<bool, TMRA3CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3CLR` writer - Counter/Timer A3 Clear bit."]
pub struct TMRA3CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer A3 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA3CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A3 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA3CLR_A::CLEAR)
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
#[doc = "Counter/Timer A3 Interrupt Enable bit based on COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3IE1_A {
    #[doc = "0: Disable counter/timer A3 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A3 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRA3IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3IE1` reader - Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
pub struct TMRA3IE1_R(crate::FieldReader<bool, TMRA3IE1_A>);
impl TMRA3IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3IE1_A {
        match self.bits {
            false => TMRA3IE1_A::DIS,
            true => TMRA3IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA3IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA3IE1_A::EN
    }
}
impl core::ops::Deref for TMRA3IE1_R {
    type Target = crate::FieldReader<bool, TMRA3IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3IE1` writer - Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
pub struct TMRA3IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3IE1_A::DIS)
    }
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3IE1_A::EN)
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
#[doc = "Counter/Timer A3 Interrupt Enable bit based on COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3IE0_A {
    #[doc = "0: Disable counter/timer A3 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A3 to generate an interrupt based on COMPR0. value."]
    EN = 1,
}
impl From<TMRA3IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3IE0` reader - Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
pub struct TMRA3IE0_R(crate::FieldReader<bool, TMRA3IE0_A>);
impl TMRA3IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3IE0_A {
        match self.bits {
            false => TMRA3IE0_A::DIS,
            true => TMRA3IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA3IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA3IE0_A::EN
    }
}
impl core::ops::Deref for TMRA3IE0_R {
    type Target = crate::FieldReader<bool, TMRA3IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3IE0` writer - Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
pub struct TMRA3IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A3 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3IE0_A::DIS)
    }
    #[doc = "Enable counter/timer A3 to generate an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3IE0_A::EN)
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
#[doc = "Counter/Timer A3 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA3FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A3, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A3, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A3, assert, count to CMPR1A3, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart. value."]
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
impl From<TMRA3FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA3FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA3FN` reader - Counter/Timer A3 Function Select."]
pub struct TMRA3FN_R(crate::FieldReader<u8, TMRA3FN_A>);
impl TMRA3FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA3FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3FN_A {
        match self.bits {
            0 => TMRA3FN_A::SINGLECOUNT,
            1 => TMRA3FN_A::REPEATEDCOUNT,
            2 => TMRA3FN_A::PULSE_ONCE,
            3 => TMRA3FN_A::PULSE_CONT,
            4 => TMRA3FN_A::SINGLEPATTERN,
            5 => TMRA3FN_A::REPEATPATTERN,
            6 => TMRA3FN_A::CONTINUOUS,
            7 => TMRA3FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRA3FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRA3FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRA3FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRA3FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRA3FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRA3FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRA3FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRA3FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRA3FN_R {
    type Target = crate::FieldReader<u8, TMRA3FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3FN` writer - Counter/Timer A3 Function Select."]
pub struct TMRA3FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A3, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA3FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A3, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA3FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A3, assert, count to CMPR1A3, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA3FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A3, assert, count to CMPR1A3, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA3FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA3FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA3FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA3FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA3FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A3 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA3CLK_A {
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
    #[doc = "20: Clock source is CTIMERB3 OUT. value."]
    CTMRB3 = 20,
    #[doc = "21: Clock source is CTIMERA2 OUT. value."]
    CTMRA2 = 21,
    #[doc = "22: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 22,
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
impl From<TMRA3CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA3CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA3CLK` reader - Counter/Timer A3 Clock Select."]
pub struct TMRA3CLK_R(crate::FieldReader<u8, TMRA3CLK_A>);
impl TMRA3CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA3CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRA3CLK_A> {
        match self.bits {
            0 => Some(TMRA3CLK_A::TMRPIN),
            1 => Some(TMRA3CLK_A::HFRC_DIV4),
            2 => Some(TMRA3CLK_A::HFRC_DIV16),
            3 => Some(TMRA3CLK_A::HFRC_DIV256),
            4 => Some(TMRA3CLK_A::HFRC_DIV1024),
            5 => Some(TMRA3CLK_A::HFRC_DIV4K),
            6 => Some(TMRA3CLK_A::XT),
            7 => Some(TMRA3CLK_A::XT_DIV2),
            8 => Some(TMRA3CLK_A::XT_DIV16),
            9 => Some(TMRA3CLK_A::XT_DIV128),
            10 => Some(TMRA3CLK_A::LFRC_DIV2),
            11 => Some(TMRA3CLK_A::LFRC_DIV32),
            12 => Some(TMRA3CLK_A::LFRC_DIV1K),
            13 => Some(TMRA3CLK_A::LFRC),
            14 => Some(TMRA3CLK_A::RTC_100HZ),
            15 => Some(TMRA3CLK_A::HCLK_DIV4),
            16 => Some(TMRA3CLK_A::XT_DIV4),
            17 => Some(TMRA3CLK_A::XT_DIV8),
            18 => Some(TMRA3CLK_A::XT_DIV32),
            20 => Some(TMRA3CLK_A::CTMRB3),
            21 => Some(TMRA3CLK_A::CTMRA2),
            22 => Some(TMRA3CLK_A::CTMRB2),
            23 => Some(TMRA3CLK_A::CTMRA4),
            24 => Some(TMRA3CLK_A::CTMRB4),
            25 => Some(TMRA3CLK_A::CTMRB0),
            26 => Some(TMRA3CLK_A::CTMRB1),
            27 => Some(TMRA3CLK_A::CTMRB5),
            28 => Some(TMRA3CLK_A::CTMRB6),
            29 => Some(TMRA3CLK_A::BUCKBLE),
            30 => Some(TMRA3CLK_A::BUCKB),
            31 => Some(TMRA3CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRA3CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRA3CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRA3CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRA3CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRA3CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRA3CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRA3CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRA3CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRA3CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRA3CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRA3CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRA3CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRA3CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRA3CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRA3CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRA3CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRA3CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRA3CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRA3CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        **self == TMRA3CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline(always)]
    pub fn is_ctmra2(&self) -> bool {
        **self == TMRA3CLK_A::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        **self == TMRA3CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRA4`"]
    #[inline(always)]
    pub fn is_ctmra4(&self) -> bool {
        **self == TMRA3CLK_A::CTMRA4
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRA3CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        **self == TMRA3CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRA3CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        **self == TMRA3CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        **self == TMRA3CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRA3CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRA3CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRA3CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRA3CLK_R {
    type Target = crate::FieldReader<u8, TMRA3CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3CLK` writer - Counter/Timer A3 Clock Select."]
pub struct TMRA3CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERA4 OUT. value."]
    #[inline(always)]
    pub fn ctmra4(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRA4)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA3CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A3 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA3EN_A {
    #[doc = "0: Counter/Timer A3 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer A3 Enable. value."]
    EN = 1,
}
impl From<TMRA3EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA3EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA3EN` reader - Counter/Timer A3 Enable bit."]
pub struct TMRA3EN_R(crate::FieldReader<bool, TMRA3EN_A>);
impl TMRA3EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA3EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA3EN_A {
        match self.bits {
            false => TMRA3EN_A::DIS,
            true => TMRA3EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA3EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA3EN_A::EN
    }
}
impl core::ops::Deref for TMRA3EN_R {
    type Target = crate::FieldReader<bool, TMRA3EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA3EN` writer - Counter/Timer A3 Enable bit."]
pub struct TMRA3EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA3EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA3EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer A3 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA3EN_A::DIS)
    }
    #[doc = "Counter/Timer A3 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA3EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A3/B3 Link bit."]
    #[inline(always)]
    pub fn ctlink3(&self) -> CTLINK3_R {
        CTLINK3_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B3 output polarity."]
    #[inline(always)]
    pub fn tmrb3pol(&self) -> TMRB3POL_R {
        TMRB3POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B3 Clear bit."]
    #[inline(always)]
    pub fn tmrb3clr(&self) -> TMRB3CLR_R {
        TMRB3CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B3 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb3ie1(&self) -> TMRB3IE1_R {
        TMRB3IE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B3 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb3ie0(&self) -> TMRB3IE0_R {
        TMRB3IE0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B3 Function Select."]
    #[inline(always)]
    pub fn tmrb3fn(&self) -> TMRB3FN_R {
        TMRB3FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B3 Clock Select."]
    #[inline(always)]
    pub fn tmrb3clk(&self) -> TMRB3CLK_R {
        TMRB3CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B3 Enable bit."]
    #[inline(always)]
    pub fn tmrb3en(&self) -> TMRB3EN_R {
        TMRB3EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Special Timer A3 enable for ADC function."]
    #[inline(always)]
    pub fn adcen(&self) -> ADCEN_R {
        ADCEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A3 output polarity."]
    #[inline(always)]
    pub fn tmra3pol(&self) -> TMRA3POL_R {
        TMRA3POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A3 Clear bit."]
    #[inline(always)]
    pub fn tmra3clr(&self) -> TMRA3CLR_R {
        TMRA3CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra3ie1(&self) -> TMRA3IE1_R {
        TMRA3IE1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra3ie0(&self) -> TMRA3IE0_R {
        TMRA3IE0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A3 Function Select."]
    #[inline(always)]
    pub fn tmra3fn(&self) -> TMRA3FN_R {
        TMRA3FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A3 Clock Select."]
    #[inline(always)]
    pub fn tmra3clk(&self) -> TMRA3CLK_R {
        TMRA3CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A3 Enable bit."]
    #[inline(always)]
    pub fn tmra3en(&self) -> TMRA3EN_R {
        TMRA3EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A3/B3 Link bit."]
    #[inline(always)]
    pub fn ctlink3(&mut self) -> CTLINK3_W {
        CTLINK3_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B3 output polarity."]
    #[inline(always)]
    pub fn tmrb3pol(&mut self) -> TMRB3POL_W {
        TMRB3POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B3 Clear bit."]
    #[inline(always)]
    pub fn tmrb3clr(&mut self) -> TMRB3CLR_W {
        TMRB3CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B3 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb3ie1(&mut self) -> TMRB3IE1_W {
        TMRB3IE1_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B3 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb3ie0(&mut self) -> TMRB3IE0_W {
        TMRB3IE0_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B3 Function Select."]
    #[inline(always)]
    pub fn tmrb3fn(&mut self) -> TMRB3FN_W {
        TMRB3FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B3 Clock Select."]
    #[inline(always)]
    pub fn tmrb3clk(&mut self) -> TMRB3CLK_W {
        TMRB3CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B3 Enable bit."]
    #[inline(always)]
    pub fn tmrb3en(&mut self) -> TMRB3EN_W {
        TMRB3EN_W { w: self }
    }
    #[doc = "Bit 15 - Special Timer A3 enable for ADC function."]
    #[inline(always)]
    pub fn adcen(&mut self) -> ADCEN_W {
        ADCEN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A3 output polarity."]
    #[inline(always)]
    pub fn tmra3pol(&mut self) -> TMRA3POL_W {
        TMRA3POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A3 Clear bit."]
    #[inline(always)]
    pub fn tmra3clr(&mut self) -> TMRA3CLR_W {
        TMRA3CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A3 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra3ie1(&mut self) -> TMRA3IE1_W {
        TMRA3IE1_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A3 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra3ie0(&mut self) -> TMRA3IE0_W {
        TMRA3IE0_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A3 Function Select."]
    #[inline(always)]
    pub fn tmra3fn(&mut self) -> TMRA3FN_W {
        TMRA3FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A3 Clock Select."]
    #[inline(always)]
    pub fn tmra3clk(&mut self) -> TMRA3CLK_W {
        TMRA3CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A3 Enable bit."]
    #[inline(always)]
    pub fn tmra3en(&mut self) -> TMRA3EN_W {
        TMRA3EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl3](index.html) module"]
pub struct CTRL3_SPEC;
impl crate::RegisterSpec for CTRL3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl3::R](R) reader structure"]
impl crate::Readable for CTRL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl3::W](W) writer structure"]
impl crate::Writable for CTRL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL3 to value 0"]
impl crate::Resettable for CTRL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

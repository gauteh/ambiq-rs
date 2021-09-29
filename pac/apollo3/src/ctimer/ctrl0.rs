#[doc = "Register `CTRL0` reader"]
pub struct R(crate::R<CTRL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL0` writer"]
pub struct W(crate::W<CTRL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL0_SPEC>;
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
impl From<crate::W<CTRL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer A0/B0 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK0_A {
    #[doc = "0: Use A0/B0 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A0/B0 timers into a single 32-bit timer. value."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK0_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLINK0` reader - Counter/Timer A0/B0 Link bit."]
pub struct CTLINK0_R(crate::FieldReader<bool, CTLINK0_A>);
impl CTLINK0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTLINK0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK0_A {
        match self.bits {
            false => CTLINK0_A::TWO_16BIT_TIMERS,
            true => CTLINK0_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        **self == CTLINK0_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        **self == CTLINK0_A::_32BIT_TIMER
    }
}
impl core::ops::Deref for CTLINK0_R {
    type Target = crate::FieldReader<bool, CTLINK0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTLINK0` writer - Counter/Timer A0/B0 Link bit."]
pub struct CTLINK0_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use A0/B0 timers as two independent 16-bit timers (default). value."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK0_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A0/B0 timers into a single 32-bit timer. value."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK0_A::_32BIT_TIMER)
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
#[doc = "Counter/Timer B0 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0POL_A {
    #[doc = "0: The polarity of the TMRPINB0 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB0 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRB0POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB0POL` reader - Counter/Timer B0 output polarity."]
pub struct TMRB0POL_R(crate::FieldReader<bool, TMRB0POL_A>);
impl TMRB0POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB0POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0POL_A {
        match self.bits {
            false => TMRB0POL_A::NORMAL,
            true => TMRB0POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRB0POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRB0POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRB0POL_R {
    type Target = crate::FieldReader<bool, TMRB0POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB0POL` writer - Counter/Timer B0 output polarity."]
pub struct TMRB0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINB0 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB0POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB0 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB0POL_A::INVERTED)
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
#[doc = "Counter/Timer B0 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0CLR_A {
    #[doc = "0: Allow counter/timer B0 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer B0 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRB0CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB0CLR` reader - Counter/Timer B0 Clear bit."]
pub struct TMRB0CLR_R(crate::FieldReader<bool, TMRB0CLR_A>);
impl TMRB0CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB0CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0CLR_A {
        match self.bits {
            false => TMRB0CLR_A::RUN,
            true => TMRB0CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRB0CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRB0CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRB0CLR_R {
    type Target = crate::FieldReader<bool, TMRB0CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB0CLR` writer - Counter/Timer B0 Clear bit."]
pub struct TMRB0CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer B0 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB0CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B0 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB0CLR_A::CLEAR)
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
#[doc = "Counter/Timer B0 Interrupt Enable bit for COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0IE1_A {
    #[doc = "0: Disable counter/timer B0 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B0 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRB0IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB0IE1` reader - Counter/Timer B0 Interrupt Enable bit for COMPR1."]
pub struct TMRB0IE1_R(crate::FieldReader<bool, TMRB0IE1_A>);
impl TMRB0IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB0IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0IE1_A {
        match self.bits {
            false => TMRB0IE1_A::DIS,
            true => TMRB0IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB0IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB0IE1_A::EN
    }
}
impl core::ops::Deref for TMRB0IE1_R {
    type Target = crate::FieldReader<bool, TMRB0IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB0IE1` writer - Counter/Timer B0 Interrupt Enable bit for COMPR1."]
pub struct TMRB0IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0IE1_A::DIS)
    }
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0IE1_A::EN)
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
#[doc = "Counter/Timer B0 Interrupt Enable bit for COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0IE0_A {
    #[doc = "0: Disable counter/timer B0 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B0 to generate an interrupt based on COMPR0 value."]
    EN = 1,
}
impl From<TMRB0IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB0IE0` reader - Counter/Timer B0 Interrupt Enable bit for COMPR0."]
pub struct TMRB0IE0_R(crate::FieldReader<bool, TMRB0IE0_A>);
impl TMRB0IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB0IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0IE0_A {
        match self.bits {
            false => TMRB0IE0_A::DIS,
            true => TMRB0IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB0IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB0IE0_A::EN
    }
}
impl core::ops::Deref for TMRB0IE0_R {
    type Target = crate::FieldReader<bool, TMRB0IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB0IE0` writer - Counter/Timer B0 Interrupt Enable bit for COMPR0."]
pub struct TMRB0IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B0 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0IE0_A::DIS)
    }
    #[doc = "Enable counter/timer B0 to generate an interrupt based on COMPR0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0IE0_A::EN)
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
#[doc = "Counter/Timer B0 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB0FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B0, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B0, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B0, assert, count to CMPR1B0, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart. value."]
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
impl From<TMRB0FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB0FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB0FN` reader - Counter/Timer B0 Function Select."]
pub struct TMRB0FN_R(crate::FieldReader<u8, TMRB0FN_A>);
impl TMRB0FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB0FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0FN_A {
        match self.bits {
            0 => TMRB0FN_A::SINGLECOUNT,
            1 => TMRB0FN_A::REPEATEDCOUNT,
            2 => TMRB0FN_A::PULSE_ONCE,
            3 => TMRB0FN_A::PULSE_CONT,
            4 => TMRB0FN_A::SINGLEPATTERN,
            5 => TMRB0FN_A::REPEATPATTERN,
            6 => TMRB0FN_A::CONTINUOUS,
            7 => TMRB0FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRB0FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRB0FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRB0FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRB0FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRB0FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRB0FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRB0FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRB0FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRB0FN_R {
    type Target = crate::FieldReader<u8, TMRB0FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB0FN` writer - Counter/Timer B0 Function Select."]
pub struct TMRB0FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B0, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB0FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B0, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB0FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B0, assert, count to CMPR1B0, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB0FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B0, assert, count to CMPR1B0, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB0FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB0FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB0FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB0FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB0FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B0 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB0CLK_A {
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
    #[doc = "20: Clock source is CTIMERA0 OUT. value."]
    CTMRA0 = 20,
    #[doc = "21: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 21,
    #[doc = "22: Clock source is CTIMERA1 OUT. value."]
    CTMRA1 = 22,
    #[doc = "23: Clock source is CTIMERA2 OUT. value."]
    CTMRA2 = 23,
    #[doc = "24: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 24,
    #[doc = "25: Clock source is CTIMERB3 OUT. value."]
    CTMRB3 = 25,
    #[doc = "26: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 26,
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
impl From<TMRB0CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB0CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB0CLK` reader - Counter/Timer B0 Clock Select."]
pub struct TMRB0CLK_R(crate::FieldReader<u8, TMRB0CLK_A>);
impl TMRB0CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB0CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRB0CLK_A> {
        match self.bits {
            0 => Some(TMRB0CLK_A::TMRPIN),
            1 => Some(TMRB0CLK_A::HFRC_DIV4),
            2 => Some(TMRB0CLK_A::HFRC_DIV16),
            3 => Some(TMRB0CLK_A::HFRC_DIV256),
            4 => Some(TMRB0CLK_A::HFRC_DIV1024),
            5 => Some(TMRB0CLK_A::HFRC_DIV4K),
            6 => Some(TMRB0CLK_A::XT),
            7 => Some(TMRB0CLK_A::XT_DIV2),
            8 => Some(TMRB0CLK_A::XT_DIV16),
            9 => Some(TMRB0CLK_A::XT_DIV128),
            10 => Some(TMRB0CLK_A::LFRC_DIV2),
            11 => Some(TMRB0CLK_A::LFRC_DIV32),
            12 => Some(TMRB0CLK_A::LFRC_DIV1K),
            13 => Some(TMRB0CLK_A::LFRC),
            14 => Some(TMRB0CLK_A::RTC_100HZ),
            15 => Some(TMRB0CLK_A::HCLK_DIV4),
            16 => Some(TMRB0CLK_A::XT_DIV4),
            17 => Some(TMRB0CLK_A::XT_DIV8),
            18 => Some(TMRB0CLK_A::XT_DIV32),
            20 => Some(TMRB0CLK_A::CTMRA0),
            21 => Some(TMRB0CLK_A::CTMRB1),
            22 => Some(TMRB0CLK_A::CTMRA1),
            23 => Some(TMRB0CLK_A::CTMRA2),
            24 => Some(TMRB0CLK_A::CTMRB2),
            25 => Some(TMRB0CLK_A::CTMRB3),
            26 => Some(TMRB0CLK_A::CTMRB4),
            27 => Some(TMRB0CLK_A::CTMRB5),
            28 => Some(TMRB0CLK_A::CTMRB6),
            29 => Some(TMRB0CLK_A::BUCKBLE),
            30 => Some(TMRB0CLK_A::BUCKB),
            31 => Some(TMRB0CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRB0CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRB0CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRB0CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRB0CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRB0CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRB0CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRB0CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRB0CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRB0CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRB0CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRB0CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRB0CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRB0CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRB0CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRB0CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRB0CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRB0CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRB0CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRB0CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA0`"]
    #[inline(always)]
    pub fn is_ctmra0(&self) -> bool {
        **self == TMRB0CLK_A::CTMRA0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRB0CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRA1`"]
    #[inline(always)]
    pub fn is_ctmra1(&self) -> bool {
        **self == TMRB0CLK_A::CTMRA1
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline(always)]
    pub fn is_ctmra2(&self) -> bool {
        **self == TMRB0CLK_A::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        **self == TMRB0CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        **self == TMRB0CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRB0CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        **self == TMRB0CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        **self == TMRB0CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRB0CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRB0CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRB0CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRB0CLK_R {
    type Target = crate::FieldReader<u8, TMRB0CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB0CLK` writer - Counter/Timer B0 Clock Select."]
pub struct TMRB0CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA0 OUT. value."]
    #[inline(always)]
    pub fn ctmra0(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRA0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn ctmra1(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRA1)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB0CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B0 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB0EN_A {
    #[doc = "0: Counter/Timer B0 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer B0 Enable. value."]
    EN = 1,
}
impl From<TMRB0EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB0EN` reader - Counter/Timer B0 Enable bit."]
pub struct TMRB0EN_R(crate::FieldReader<bool, TMRB0EN_A>);
impl TMRB0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB0EN_A {
        match self.bits {
            false => TMRB0EN_A::DIS,
            true => TMRB0EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB0EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB0EN_A::EN
    }
}
impl core::ops::Deref for TMRB0EN_R {
    type Target = crate::FieldReader<bool, TMRB0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB0EN` writer - Counter/Timer B0 Enable bit."]
pub struct TMRB0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer B0 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB0EN_A::DIS)
    }
    #[doc = "Counter/Timer B0 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB0EN_A::EN)
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
#[doc = "Counter/Timer A0 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0POL_A {
    #[doc = "0: The polarity of the TMRPINA0 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA0 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRA0POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA0POL` reader - Counter/Timer A0 output polarity."]
pub struct TMRA0POL_R(crate::FieldReader<bool, TMRA0POL_A>);
impl TMRA0POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA0POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0POL_A {
        match self.bits {
            false => TMRA0POL_A::NORMAL,
            true => TMRA0POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRA0POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRA0POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRA0POL_R {
    type Target = crate::FieldReader<bool, TMRA0POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA0POL` writer - Counter/Timer A0 output polarity."]
pub struct TMRA0POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINA0 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA0POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA0 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA0POL_A::INVERTED)
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
#[doc = "Counter/Timer A0 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0CLR_A {
    #[doc = "0: Allow counter/timer A0 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer A0 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRA0CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA0CLR` reader - Counter/Timer A0 Clear bit."]
pub struct TMRA0CLR_R(crate::FieldReader<bool, TMRA0CLR_A>);
impl TMRA0CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA0CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0CLR_A {
        match self.bits {
            false => TMRA0CLR_A::RUN,
            true => TMRA0CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRA0CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRA0CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRA0CLR_R {
    type Target = crate::FieldReader<bool, TMRA0CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA0CLR` writer - Counter/Timer A0 Clear bit."]
pub struct TMRA0CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer A0 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA0CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A0 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA0CLR_A::CLEAR)
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
#[doc = "Counter/Timer A0 Interrupt Enable bit based on COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0IE1_A {
    #[doc = "0: Disable counter/timer A0 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A0 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRA0IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA0IE1` reader - Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
pub struct TMRA0IE1_R(crate::FieldReader<bool, TMRA0IE1_A>);
impl TMRA0IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA0IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0IE1_A {
        match self.bits {
            false => TMRA0IE1_A::DIS,
            true => TMRA0IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA0IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA0IE1_A::EN
    }
}
impl core::ops::Deref for TMRA0IE1_R {
    type Target = crate::FieldReader<bool, TMRA0IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA0IE1` writer - Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
pub struct TMRA0IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0IE1_A::DIS)
    }
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0IE1_A::EN)
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
#[doc = "Counter/Timer A0 Interrupt Enable bit based on COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0IE0_A {
    #[doc = "0: Disable counter/timer A0 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A0 to generate an interrupt based on COMPR0. value."]
    EN = 1,
}
impl From<TMRA0IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA0IE0` reader - Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
pub struct TMRA0IE0_R(crate::FieldReader<bool, TMRA0IE0_A>);
impl TMRA0IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA0IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0IE0_A {
        match self.bits {
            false => TMRA0IE0_A::DIS,
            true => TMRA0IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA0IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA0IE0_A::EN
    }
}
impl core::ops::Deref for TMRA0IE0_R {
    type Target = crate::FieldReader<bool, TMRA0IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA0IE0` writer - Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
pub struct TMRA0IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A0 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0IE0_A::DIS)
    }
    #[doc = "Enable counter/timer A0 to generate an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0IE0_A::EN)
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
#[doc = "Counter/Timer A0 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA0FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A0, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A0, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A0, assert, count to CMPR1A0, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart. value."]
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
impl From<TMRA0FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA0FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA0FN` reader - Counter/Timer A0 Function Select."]
pub struct TMRA0FN_R(crate::FieldReader<u8, TMRA0FN_A>);
impl TMRA0FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA0FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0FN_A {
        match self.bits {
            0 => TMRA0FN_A::SINGLECOUNT,
            1 => TMRA0FN_A::REPEATEDCOUNT,
            2 => TMRA0FN_A::PULSE_ONCE,
            3 => TMRA0FN_A::PULSE_CONT,
            4 => TMRA0FN_A::SINGLEPATTERN,
            5 => TMRA0FN_A::REPEATPATTERN,
            6 => TMRA0FN_A::CONTINUOUS,
            7 => TMRA0FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRA0FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRA0FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRA0FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRA0FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRA0FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRA0FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRA0FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRA0FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRA0FN_R {
    type Target = crate::FieldReader<u8, TMRA0FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA0FN` writer - Counter/Timer A0 Function Select."]
pub struct TMRA0FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A0, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA0FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A0, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA0FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A0, assert, count to CMPR1A0, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA0FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A0, assert, count to CMPR1A0, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA0FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA0FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA0FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA0FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA0FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A0 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA0CLK_A {
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
    #[doc = "20: Clock source is CTIMERB0 OUT. value."]
    CTMRB0 = 20,
    #[doc = "21: Clock source is CTIMERA1 OUT. value."]
    CTMRA1 = 21,
    #[doc = "22: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 22,
    #[doc = "23: Clock source is CTIMERA2 OUT. value."]
    CTMRA2 = 23,
    #[doc = "24: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 24,
    #[doc = "25: Clock source is CTIMERB3 OUT. value."]
    CTMRB3 = 25,
    #[doc = "26: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 26,
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
impl From<TMRA0CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA0CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA0CLK` reader - Counter/Timer A0 Clock Select."]
pub struct TMRA0CLK_R(crate::FieldReader<u8, TMRA0CLK_A>);
impl TMRA0CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA0CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRA0CLK_A> {
        match self.bits {
            0 => Some(TMRA0CLK_A::TMRPIN),
            1 => Some(TMRA0CLK_A::HFRC_DIV4),
            2 => Some(TMRA0CLK_A::HFRC_DIV16),
            3 => Some(TMRA0CLK_A::HFRC_DIV256),
            4 => Some(TMRA0CLK_A::HFRC_DIV1024),
            5 => Some(TMRA0CLK_A::HFRC_DIV4K),
            6 => Some(TMRA0CLK_A::XT),
            7 => Some(TMRA0CLK_A::XT_DIV2),
            8 => Some(TMRA0CLK_A::XT_DIV16),
            9 => Some(TMRA0CLK_A::XT_DIV128),
            10 => Some(TMRA0CLK_A::LFRC_DIV2),
            11 => Some(TMRA0CLK_A::LFRC_DIV32),
            12 => Some(TMRA0CLK_A::LFRC_DIV1K),
            13 => Some(TMRA0CLK_A::LFRC),
            14 => Some(TMRA0CLK_A::RTC_100HZ),
            15 => Some(TMRA0CLK_A::HCLK_DIV4),
            16 => Some(TMRA0CLK_A::XT_DIV4),
            17 => Some(TMRA0CLK_A::XT_DIV8),
            18 => Some(TMRA0CLK_A::XT_DIV32),
            20 => Some(TMRA0CLK_A::CTMRB0),
            21 => Some(TMRA0CLK_A::CTMRA1),
            22 => Some(TMRA0CLK_A::CTMRB1),
            23 => Some(TMRA0CLK_A::CTMRA2),
            24 => Some(TMRA0CLK_A::CTMRB2),
            25 => Some(TMRA0CLK_A::CTMRB3),
            26 => Some(TMRA0CLK_A::CTMRB4),
            27 => Some(TMRA0CLK_A::CTMRB5),
            28 => Some(TMRA0CLK_A::CTMRB6),
            29 => Some(TMRA0CLK_A::BUCKBLE),
            30 => Some(TMRA0CLK_A::BUCKB),
            31 => Some(TMRA0CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRA0CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRA0CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRA0CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRA0CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRA0CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRA0CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRA0CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRA0CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRA0CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRA0CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRA0CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRA0CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRA0CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRA0CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRA0CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRA0CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRA0CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRA0CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRA0CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        **self == TMRA0CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRA1`"]
    #[inline(always)]
    pub fn is_ctmra1(&self) -> bool {
        **self == TMRA0CLK_A::CTMRA1
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRA0CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRA2`"]
    #[inline(always)]
    pub fn is_ctmra2(&self) -> bool {
        **self == TMRA0CLK_A::CTMRA2
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        **self == TMRA0CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        **self == TMRA0CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRA0CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `CTMRB5`"]
    #[inline(always)]
    pub fn is_ctmrb5(&self) -> bool {
        **self == TMRA0CLK_A::CTMRB5
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        **self == TMRA0CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRA0CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRA0CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRA0CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRA0CLK_R {
    type Target = crate::FieldReader<u8, TMRA0CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA0CLK` writer - Counter/Timer A0 Clock Select."]
pub struct TMRA0CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERA1 OUT. value."]
    #[inline(always)]
    pub fn ctmra1(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRA1)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERA2 OUT. value."]
    #[inline(always)]
    pub fn ctmra2(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRA2)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRB4)
    }
    #[doc = "Clock source is CTIMERB5 OUT. value."]
    #[inline(always)]
    pub fn ctmrb5(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRB5)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::CTMRB6)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA0CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A0 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA0EN_A {
    #[doc = "0: Counter/Timer A0 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer A0 Enable. value."]
    EN = 1,
}
impl From<TMRA0EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA0EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA0EN` reader - Counter/Timer A0 Enable bit."]
pub struct TMRA0EN_R(crate::FieldReader<bool, TMRA0EN_A>);
impl TMRA0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA0EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA0EN_A {
        match self.bits {
            false => TMRA0EN_A::DIS,
            true => TMRA0EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA0EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA0EN_A::EN
    }
}
impl core::ops::Deref for TMRA0EN_R {
    type Target = crate::FieldReader<bool, TMRA0EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA0EN` writer - Counter/Timer A0 Enable bit."]
pub struct TMRA0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA0EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA0EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer A0 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA0EN_A::DIS)
    }
    #[doc = "Counter/Timer A0 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA0EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A0/B0 Link bit."]
    #[inline(always)]
    pub fn ctlink0(&self) -> CTLINK0_R {
        CTLINK0_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B0 output polarity."]
    #[inline(always)]
    pub fn tmrb0pol(&self) -> TMRB0POL_R {
        TMRB0POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B0 Clear bit."]
    #[inline(always)]
    pub fn tmrb0clr(&self) -> TMRB0CLR_R {
        TMRB0CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B0 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb0ie1(&self) -> TMRB0IE1_R {
        TMRB0IE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B0 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb0ie0(&self) -> TMRB0IE0_R {
        TMRB0IE0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B0 Function Select."]
    #[inline(always)]
    pub fn tmrb0fn(&self) -> TMRB0FN_R {
        TMRB0FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B0 Clock Select."]
    #[inline(always)]
    pub fn tmrb0clk(&self) -> TMRB0CLK_R {
        TMRB0CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B0 Enable bit."]
    #[inline(always)]
    pub fn tmrb0en(&self) -> TMRB0EN_R {
        TMRB0EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A0 output polarity."]
    #[inline(always)]
    pub fn tmra0pol(&self) -> TMRA0POL_R {
        TMRA0POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A0 Clear bit."]
    #[inline(always)]
    pub fn tmra0clr(&self) -> TMRA0CLR_R {
        TMRA0CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra0ie1(&self) -> TMRA0IE1_R {
        TMRA0IE1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra0ie0(&self) -> TMRA0IE0_R {
        TMRA0IE0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A0 Function Select."]
    #[inline(always)]
    pub fn tmra0fn(&self) -> TMRA0FN_R {
        TMRA0FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A0 Clock Select."]
    #[inline(always)]
    pub fn tmra0clk(&self) -> TMRA0CLK_R {
        TMRA0CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A0 Enable bit."]
    #[inline(always)]
    pub fn tmra0en(&self) -> TMRA0EN_R {
        TMRA0EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A0/B0 Link bit."]
    #[inline(always)]
    pub fn ctlink0(&mut self) -> CTLINK0_W {
        CTLINK0_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B0 output polarity."]
    #[inline(always)]
    pub fn tmrb0pol(&mut self) -> TMRB0POL_W {
        TMRB0POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B0 Clear bit."]
    #[inline(always)]
    pub fn tmrb0clr(&mut self) -> TMRB0CLR_W {
        TMRB0CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B0 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb0ie1(&mut self) -> TMRB0IE1_W {
        TMRB0IE1_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B0 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb0ie0(&mut self) -> TMRB0IE0_W {
        TMRB0IE0_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B0 Function Select."]
    #[inline(always)]
    pub fn tmrb0fn(&mut self) -> TMRB0FN_W {
        TMRB0FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B0 Clock Select."]
    #[inline(always)]
    pub fn tmrb0clk(&mut self) -> TMRB0CLK_W {
        TMRB0CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B0 Enable bit."]
    #[inline(always)]
    pub fn tmrb0en(&mut self) -> TMRB0EN_W {
        TMRB0EN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A0 output polarity."]
    #[inline(always)]
    pub fn tmra0pol(&mut self) -> TMRA0POL_W {
        TMRA0POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A0 Clear bit."]
    #[inline(always)]
    pub fn tmra0clr(&mut self) -> TMRA0CLR_W {
        TMRA0CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A0 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra0ie1(&mut self) -> TMRA0IE1_W {
        TMRA0IE1_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A0 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra0ie0(&mut self) -> TMRA0IE0_W {
        TMRA0IE0_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A0 Function Select."]
    #[inline(always)]
    pub fn tmra0fn(&mut self) -> TMRA0FN_W {
        TMRA0FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A0 Clock Select."]
    #[inline(always)]
    pub fn tmra0clk(&mut self) -> TMRA0CLK_W {
        TMRA0CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A0 Enable bit."]
    #[inline(always)]
    pub fn tmra0en(&mut self) -> TMRA0EN_W {
        TMRA0EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl0](index.html) module"]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl0::R](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl0::W](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL0 to value 0"]
impl crate::Resettable for CTRL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

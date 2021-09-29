#[doc = "Register `CTRL6` reader"]
pub struct R(crate::R<CTRL6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL6` writer"]
pub struct W(crate::W<CTRL6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL6_SPEC>;
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
impl From<crate::W<CTRL6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Counter/Timer A6/B6 Link bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CTLINK6_A {
    #[doc = "0: Use A6/B6 timers as two independent 16-bit timers (default). value."]
    TWO_16BIT_TIMERS = 0,
    #[doc = "1: Link A6/B6 timers into a single 32-bit timer. value."]
    _32BIT_TIMER = 1,
}
impl From<CTLINK6_A> for bool {
    #[inline(always)]
    fn from(variant: CTLINK6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTLINK6` reader - Counter/Timer A6/B6 Link bit."]
pub struct CTLINK6_R(crate::FieldReader<bool, CTLINK6_A>);
impl CTLINK6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTLINK6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTLINK6_A {
        match self.bits {
            false => CTLINK6_A::TWO_16BIT_TIMERS,
            true => CTLINK6_A::_32BIT_TIMER,
        }
    }
    #[doc = "Checks if the value of the field is `TWO_16BIT_TIMERS`"]
    #[inline(always)]
    pub fn is_two_16bit_timers(&self) -> bool {
        **self == CTLINK6_A::TWO_16BIT_TIMERS
    }
    #[doc = "Checks if the value of the field is `_32BIT_TIMER`"]
    #[inline(always)]
    pub fn is_32bit_timer(&self) -> bool {
        **self == CTLINK6_A::_32BIT_TIMER
    }
}
impl core::ops::Deref for CTLINK6_R {
    type Target = crate::FieldReader<bool, CTLINK6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTLINK6` writer - Counter/Timer A6/B6 Link bit."]
pub struct CTLINK6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTLINK6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CTLINK6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Use A6/B6 timers as two independent 16-bit timers (default). value."]
    #[inline(always)]
    pub fn two_16bit_timers(self) -> &'a mut W {
        self.variant(CTLINK6_A::TWO_16BIT_TIMERS)
    }
    #[doc = "Link A6/B6 timers into a single 32-bit timer. value."]
    #[inline(always)]
    pub fn _32bit_timer(self) -> &'a mut W {
        self.variant(CTLINK6_A::_32BIT_TIMER)
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
#[doc = "Counter/Timer B6 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6POL_A {
    #[doc = "0: The polarity of the TMRPINB6 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINB6 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRB6POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6POL` reader - Counter/Timer B6 output polarity."]
pub struct TMRB6POL_R(crate::FieldReader<bool, TMRB6POL_A>);
impl TMRB6POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6POL_A {
        match self.bits {
            false => TMRB6POL_A::NORMAL,
            true => TMRB6POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRB6POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRB6POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRB6POL_R {
    type Target = crate::FieldReader<bool, TMRB6POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6POL` writer - Counter/Timer B6 output polarity."]
pub struct TMRB6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINB6 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRB6POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINB6 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRB6POL_A::INVERTED)
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
#[doc = "Counter/Timer B6 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6CLR_A {
    #[doc = "0: Allow counter/timer B6 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer B6 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRB6CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6CLR` reader - Counter/Timer B6 Clear bit."]
pub struct TMRB6CLR_R(crate::FieldReader<bool, TMRB6CLR_A>);
impl TMRB6CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6CLR_A {
        match self.bits {
            false => TMRB6CLR_A::RUN,
            true => TMRB6CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRB6CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRB6CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRB6CLR_R {
    type Target = crate::FieldReader<bool, TMRB6CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6CLR` writer - Counter/Timer B6 Clear bit."]
pub struct TMRB6CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer B6 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRB6CLR_A::RUN)
    }
    #[doc = "Holds counter/timer B6 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRB6CLR_A::CLEAR)
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
#[doc = "Counter/Timer B6 Interrupt Enable bit for COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6IE1_A {
    #[doc = "0: Disable counter/timer B6 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B6 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRB6IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6IE1` reader - Counter/Timer B6 Interrupt Enable bit for COMPR1."]
pub struct TMRB6IE1_R(crate::FieldReader<bool, TMRB6IE1_A>);
impl TMRB6IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6IE1_A {
        match self.bits {
            false => TMRB6IE1_A::DIS,
            true => TMRB6IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB6IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB6IE1_A::EN
    }
}
impl core::ops::Deref for TMRB6IE1_R {
    type Target = crate::FieldReader<bool, TMRB6IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6IE1` writer - Counter/Timer B6 Interrupt Enable bit for COMPR1."]
pub struct TMRB6IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6IE1_A::DIS)
    }
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6IE1_A::EN)
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
#[doc = "Counter/Timer B6 Interrupt Enable bit for COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6IE0_A {
    #[doc = "0: Disable counter/timer B6 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer B6 to generate an interrupt based on COMPR0 value."]
    EN = 1,
}
impl From<TMRB6IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6IE0` reader - Counter/Timer B6 Interrupt Enable bit for COMPR0."]
pub struct TMRB6IE0_R(crate::FieldReader<bool, TMRB6IE0_A>);
impl TMRB6IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6IE0_A {
        match self.bits {
            false => TMRB6IE0_A::DIS,
            true => TMRB6IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB6IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB6IE0_A::EN
    }
}
impl core::ops::Deref for TMRB6IE0_R {
    type Target = crate::FieldReader<bool, TMRB6IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6IE0` writer - Counter/Timer B6 Interrupt Enable bit for COMPR0."]
pub struct TMRB6IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer B6 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6IE0_A::DIS)
    }
    #[doc = "Enable counter/timer B6 to generate an interrupt based on COMPR0 value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6IE0_A::EN)
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
#[doc = "Counter/Timer B6 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB6FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0B6, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0B6, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0B6, assert, count to CMPR1B6, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0B6, assert, count to CMPR1B6, deassert, restart. value."]
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
impl From<TMRB6FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB6FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB6FN` reader - Counter/Timer B6 Function Select."]
pub struct TMRB6FN_R(crate::FieldReader<u8, TMRB6FN_A>);
impl TMRB6FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB6FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6FN_A {
        match self.bits {
            0 => TMRB6FN_A::SINGLECOUNT,
            1 => TMRB6FN_A::REPEATEDCOUNT,
            2 => TMRB6FN_A::PULSE_ONCE,
            3 => TMRB6FN_A::PULSE_CONT,
            4 => TMRB6FN_A::SINGLEPATTERN,
            5 => TMRB6FN_A::REPEATPATTERN,
            6 => TMRB6FN_A::CONTINUOUS,
            7 => TMRB6FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRB6FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRB6FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRB6FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRB6FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRB6FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRB6FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRB6FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRB6FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRB6FN_R {
    type Target = crate::FieldReader<u8, TMRB6FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6FN` writer - Counter/Timer B6 Function Select."]
pub struct TMRB6FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0B6, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRB6FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0B6, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRB6FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0B6, assert, count to CMPR1B6, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRB6FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0B6, assert, count to CMPR1B6, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRB6FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRB6FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRB6FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRB6FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRB6FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Counter/Timer B6 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRB6CLK_A {
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
    #[doc = "20: Clock source is CTIMERA6 OUT. value."]
    CTMRA6 = 20,
    #[doc = "21: Clock source is CTIMERA3 OUT. value."]
    CTMRA3 = 21,
    #[doc = "22: Clock source is CTIMERB3 OUT. value."]
    CTMRB3 = 22,
    #[doc = "23: Clock source is CTIMERA7 OUT. value."]
    CTMRA7 = 23,
    #[doc = "24: Clock source is CTIMERB7 OUT. value."]
    CTMRB7 = 24,
    #[doc = "25: Clock source is CTIMERB0 OUT. value."]
    CTMRB0 = 25,
    #[doc = "26: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 26,
    #[doc = "27: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 27,
    #[doc = "28: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses. value."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses. value."]
    BUCKA = 31,
}
impl From<TMRB6CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRB6CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRB6CLK` reader - Counter/Timer B6 Clock Select."]
pub struct TMRB6CLK_R(crate::FieldReader<u8, TMRB6CLK_A>);
impl TMRB6CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRB6CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRB6CLK_A> {
        match self.bits {
            0 => Some(TMRB6CLK_A::TMRPIN),
            1 => Some(TMRB6CLK_A::HFRC_DIV4),
            2 => Some(TMRB6CLK_A::HFRC_DIV16),
            3 => Some(TMRB6CLK_A::HFRC_DIV256),
            4 => Some(TMRB6CLK_A::HFRC_DIV1024),
            5 => Some(TMRB6CLK_A::HFRC_DIV4K),
            6 => Some(TMRB6CLK_A::XT),
            7 => Some(TMRB6CLK_A::XT_DIV2),
            8 => Some(TMRB6CLK_A::XT_DIV16),
            9 => Some(TMRB6CLK_A::XT_DIV128),
            10 => Some(TMRB6CLK_A::LFRC_DIV2),
            11 => Some(TMRB6CLK_A::LFRC_DIV32),
            12 => Some(TMRB6CLK_A::LFRC_DIV1K),
            13 => Some(TMRB6CLK_A::LFRC),
            14 => Some(TMRB6CLK_A::RTC_100HZ),
            15 => Some(TMRB6CLK_A::HCLK_DIV4),
            16 => Some(TMRB6CLK_A::XT_DIV4),
            17 => Some(TMRB6CLK_A::XT_DIV8),
            18 => Some(TMRB6CLK_A::XT_DIV32),
            20 => Some(TMRB6CLK_A::CTMRA6),
            21 => Some(TMRB6CLK_A::CTMRA3),
            22 => Some(TMRB6CLK_A::CTMRB3),
            23 => Some(TMRB6CLK_A::CTMRA7),
            24 => Some(TMRB6CLK_A::CTMRB7),
            25 => Some(TMRB6CLK_A::CTMRB0),
            26 => Some(TMRB6CLK_A::CTMRB1),
            27 => Some(TMRB6CLK_A::CTMRB2),
            28 => Some(TMRB6CLK_A::CTMRB4),
            29 => Some(TMRB6CLK_A::BUCKBLE),
            30 => Some(TMRB6CLK_A::BUCKB),
            31 => Some(TMRB6CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRB6CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRB6CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRB6CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRB6CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRB6CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRB6CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRB6CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRB6CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRB6CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRB6CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRB6CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRB6CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRB6CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRB6CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRB6CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRB6CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRB6CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRB6CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRB6CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRA6`"]
    #[inline(always)]
    pub fn is_ctmra6(&self) -> bool {
        **self == TMRB6CLK_A::CTMRA6
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline(always)]
    pub fn is_ctmra3(&self) -> bool {
        **self == TMRB6CLK_A::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        **self == TMRB6CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA7`"]
    #[inline(always)]
    pub fn is_ctmra7(&self) -> bool {
        **self == TMRB6CLK_A::CTMRA7
    }
    #[doc = "Checks if the value of the field is `CTMRB7`"]
    #[inline(always)]
    pub fn is_ctmrb7(&self) -> bool {
        **self == TMRB6CLK_A::CTMRB7
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        **self == TMRB6CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRB6CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        **self == TMRB6CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRB6CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRB6CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRB6CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRB6CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRB6CLK_R {
    type Target = crate::FieldReader<u8, TMRB6CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6CLK` writer - Counter/Timer B6 Clock Select."]
pub struct TMRB6CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINB. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERA6 OUT. value."]
    #[inline(always)]
    pub fn ctmra6(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRA6)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRA3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    #[inline(always)]
    pub fn ctmra7(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRA7)
    }
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    #[inline(always)]
    pub fn ctmrb7(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRB7)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRB6CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 17)) | ((value as u32 & 0x1f) << 17);
        self.w
    }
}
#[doc = "Counter/Timer B6 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRB6EN_A {
    #[doc = "0: Counter/Timer B6 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer B6 Enable. value."]
    EN = 1,
}
impl From<TMRB6EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRB6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRB6EN` reader - Counter/Timer B6 Enable bit."]
pub struct TMRB6EN_R(crate::FieldReader<bool, TMRB6EN_A>);
impl TMRB6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRB6EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRB6EN_A {
        match self.bits {
            false => TMRB6EN_A::DIS,
            true => TMRB6EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRB6EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRB6EN_A::EN
    }
}
impl core::ops::Deref for TMRB6EN_R {
    type Target = crate::FieldReader<bool, TMRB6EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRB6EN` writer - Counter/Timer B6 Enable bit."]
pub struct TMRB6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRB6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRB6EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer B6 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRB6EN_A::DIS)
    }
    #[doc = "Counter/Timer B6 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRB6EN_A::EN)
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
#[doc = "Counter/Timer A6 output polarity.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6POL_A {
    #[doc = "0: The polarity of the TMRPINA6 pin is the same as the timer output. value."]
    NORMAL = 0,
    #[doc = "1: The polarity of the TMRPINA6 pin is the inverse of the timer output. value."]
    INVERTED = 1,
}
impl From<TMRA6POL_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6POL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6POL` reader - Counter/Timer A6 output polarity."]
pub struct TMRA6POL_R(crate::FieldReader<bool, TMRA6POL_A>);
impl TMRA6POL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6POL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6POL_A {
        match self.bits {
            false => TMRA6POL_A::NORMAL,
            true => TMRA6POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == TMRA6POL_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        **self == TMRA6POL_A::INVERTED
    }
}
impl core::ops::Deref for TMRA6POL_R {
    type Target = crate::FieldReader<bool, TMRA6POL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6POL` writer - Counter/Timer A6 output polarity."]
pub struct TMRA6POL_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6POL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6POL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "The polarity of the TMRPINA6 pin is the same as the timer output. value."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TMRA6POL_A::NORMAL)
    }
    #[doc = "The polarity of the TMRPINA6 pin is the inverse of the timer output. value."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TMRA6POL_A::INVERTED)
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
#[doc = "Counter/Timer A6 Clear bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6CLR_A {
    #[doc = "0: Allow counter/timer A6 to run value."]
    RUN = 0,
    #[doc = "1: Holds counter/timer A6 at 0x0000. value."]
    CLEAR = 1,
}
impl From<TMRA6CLR_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6CLR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6CLR` reader - Counter/Timer A6 Clear bit."]
pub struct TMRA6CLR_R(crate::FieldReader<bool, TMRA6CLR_A>);
impl TMRA6CLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6CLR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6CLR_A {
        match self.bits {
            false => TMRA6CLR_A::RUN,
            true => TMRA6CLR_A::CLEAR,
        }
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == TMRA6CLR_A::RUN
    }
    #[doc = "Checks if the value of the field is `CLEAR`"]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        **self == TMRA6CLR_A::CLEAR
    }
}
impl core::ops::Deref for TMRA6CLR_R {
    type Target = crate::FieldReader<bool, TMRA6CLR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6CLR` writer - Counter/Timer A6 Clear bit."]
pub struct TMRA6CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6CLR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6CLR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Allow counter/timer A6 to run value."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(TMRA6CLR_A::RUN)
    }
    #[doc = "Holds counter/timer A6 at 0x0000. value."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(TMRA6CLR_A::CLEAR)
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
#[doc = "Counter/Timer A6 Interrupt Enable bit based on COMPR1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6IE1_A {
    #[doc = "0: Disable counter/timer A6 from generating an interrupt based on COMPR1. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A6 to generate an interrupt based on COMPR1. value."]
    EN = 1,
}
impl From<TMRA6IE1_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6IE1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6IE1` reader - Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
pub struct TMRA6IE1_R(crate::FieldReader<bool, TMRA6IE1_A>);
impl TMRA6IE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6IE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6IE1_A {
        match self.bits {
            false => TMRA6IE1_A::DIS,
            true => TMRA6IE1_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA6IE1_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA6IE1_A::EN
    }
}
impl core::ops::Deref for TMRA6IE1_R {
    type Target = crate::FieldReader<bool, TMRA6IE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6IE1` writer - Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
pub struct TMRA6IE1_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6IE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6IE1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6IE1_A::DIS)
    }
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR1. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6IE1_A::EN)
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
#[doc = "Counter/Timer A6 Interrupt Enable bit based on COMPR0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6IE0_A {
    #[doc = "0: Disable counter/timer A6 from generating an interrupt based on COMPR0. value."]
    DIS = 0,
    #[doc = "1: Enable counter/timer A6 to generate an interrupt based on COMPR0. value."]
    EN = 1,
}
impl From<TMRA6IE0_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6IE0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6IE0` reader - Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
pub struct TMRA6IE0_R(crate::FieldReader<bool, TMRA6IE0_A>);
impl TMRA6IE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6IE0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6IE0_A {
        match self.bits {
            false => TMRA6IE0_A::DIS,
            true => TMRA6IE0_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA6IE0_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA6IE0_A::EN
    }
}
impl core::ops::Deref for TMRA6IE0_R {
    type Target = crate::FieldReader<bool, TMRA6IE0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6IE0` writer - Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
pub struct TMRA6IE0_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6IE0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6IE0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable counter/timer A6 from generating an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6IE0_A::DIS)
    }
    #[doc = "Enable counter/timer A6 to generate an interrupt based on COMPR0. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6IE0_A::EN)
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
#[doc = "Counter/Timer A6 Function Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA6FN_A {
    #[doc = "0: Single count (output toggles and sticks).  Count to CMPR0A6, stop. value."]
    SINGLECOUNT = 0,
    #[doc = "1: Repeated count (periodic 1-clock-cycle-wide pulses).  Count to CMPR0A6, restart. value."]
    REPEATEDCOUNT = 1,
    #[doc = "2: Pulse once (aka one-shot).  Count to CMPR0A6, assert, count to CMPR1A6, deassert, stop. value."]
    PULSE_ONCE = 2,
    #[doc = "3: Pulse continously.  Count to CMPR0A6, assert, count to CMPR1A6, deassert, restart. value."]
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
impl From<TMRA6FN_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA6FN_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA6FN` reader - Counter/Timer A6 Function Select."]
pub struct TMRA6FN_R(crate::FieldReader<u8, TMRA6FN_A>);
impl TMRA6FN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA6FN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6FN_A {
        match self.bits {
            0 => TMRA6FN_A::SINGLECOUNT,
            1 => TMRA6FN_A::REPEATEDCOUNT,
            2 => TMRA6FN_A::PULSE_ONCE,
            3 => TMRA6FN_A::PULSE_CONT,
            4 => TMRA6FN_A::SINGLEPATTERN,
            5 => TMRA6FN_A::REPEATPATTERN,
            6 => TMRA6FN_A::CONTINUOUS,
            7 => TMRA6FN_A::ALTPWN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLECOUNT`"]
    #[inline(always)]
    pub fn is_singlecount(&self) -> bool {
        **self == TMRA6FN_A::SINGLECOUNT
    }
    #[doc = "Checks if the value of the field is `REPEATEDCOUNT`"]
    #[inline(always)]
    pub fn is_repeatedcount(&self) -> bool {
        **self == TMRA6FN_A::REPEATEDCOUNT
    }
    #[doc = "Checks if the value of the field is `PULSE_ONCE`"]
    #[inline(always)]
    pub fn is_pulse_once(&self) -> bool {
        **self == TMRA6FN_A::PULSE_ONCE
    }
    #[doc = "Checks if the value of the field is `PULSE_CONT`"]
    #[inline(always)]
    pub fn is_pulse_cont(&self) -> bool {
        **self == TMRA6FN_A::PULSE_CONT
    }
    #[doc = "Checks if the value of the field is `SINGLEPATTERN`"]
    #[inline(always)]
    pub fn is_singlepattern(&self) -> bool {
        **self == TMRA6FN_A::SINGLEPATTERN
    }
    #[doc = "Checks if the value of the field is `REPEATPATTERN`"]
    #[inline(always)]
    pub fn is_repeatpattern(&self) -> bool {
        **self == TMRA6FN_A::REPEATPATTERN
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        **self == TMRA6FN_A::CONTINUOUS
    }
    #[doc = "Checks if the value of the field is `ALTPWN`"]
    #[inline(always)]
    pub fn is_altpwn(&self) -> bool {
        **self == TMRA6FN_A::ALTPWN
    }
}
impl core::ops::Deref for TMRA6FN_R {
    type Target = crate::FieldReader<u8, TMRA6FN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6FN` writer - Counter/Timer A6 Function Select."]
pub struct TMRA6FN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6FN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6FN_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Single count (output toggles and sticks). Count to CMPR0A6, stop. value."]
    #[inline(always)]
    pub fn singlecount(self) -> &'a mut W {
        self.variant(TMRA6FN_A::SINGLECOUNT)
    }
    #[doc = "Repeated count (periodic 1-clock-cycle-wide pulses). Count to CMPR0A6, restart. value."]
    #[inline(always)]
    pub fn repeatedcount(self) -> &'a mut W {
        self.variant(TMRA6FN_A::REPEATEDCOUNT)
    }
    #[doc = "Pulse once (aka one-shot). Count to CMPR0A6, assert, count to CMPR1A6, deassert, stop. value."]
    #[inline(always)]
    pub fn pulse_once(self) -> &'a mut W {
        self.variant(TMRA6FN_A::PULSE_ONCE)
    }
    #[doc = "Pulse continously. Count to CMPR0A6, assert, count to CMPR1A6, deassert, restart. value."]
    #[inline(always)]
    pub fn pulse_cont(self) -> &'a mut W {
        self.variant(TMRA6FN_A::PULSE_CONT)
    }
    #[doc = "Single pattern. value."]
    #[inline(always)]
    pub fn singlepattern(self) -> &'a mut W {
        self.variant(TMRA6FN_A::SINGLEPATTERN)
    }
    #[doc = "Repeated pattern. value."]
    #[inline(always)]
    pub fn repeatpattern(self) -> &'a mut W {
        self.variant(TMRA6FN_A::REPEATPATTERN)
    }
    #[doc = "Continuous run (aka Free Run). Count continuously. value."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TMRA6FN_A::CONTINUOUS)
    }
    #[doc = "Alternate PWM value."]
    #[inline(always)]
    pub fn altpwn(self) -> &'a mut W {
        self.variant(TMRA6FN_A::ALTPWN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Counter/Timer A6 Clock Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMRA6CLK_A {
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
    #[doc = "20: Clock source is CTIMERB6 OUT. value."]
    CTMRB6 = 20,
    #[doc = "21: Clock source is CTIMERA3 OUT. value."]
    CTMRA3 = 21,
    #[doc = "22: Clock source is CTIMERB3 OUT. value."]
    CTMRB3 = 22,
    #[doc = "23: Clock source is CTIMERA7 OUT. value."]
    CTMRA7 = 23,
    #[doc = "24: Clock source is CTIMERB7 OUT. value."]
    CTMRB7 = 24,
    #[doc = "25: Clock source is CTIMERB0 OUT. value."]
    CTMRB0 = 25,
    #[doc = "26: Clock source is CTIMERB1 OUT. value."]
    CTMRB1 = 26,
    #[doc = "27: Clock source is CTIMERB2 OUT. value."]
    CTMRB2 = 27,
    #[doc = "28: Clock source is CTIMERB4 OUT. value."]
    CTMRB4 = 28,
    #[doc = "29: Clock source is BLE buck converter TON pulses. value."]
    BUCKBLE = 29,
    #[doc = "30: Clock source is Memory buck converter TON pulses. value."]
    BUCKB = 30,
    #[doc = "31: Clock source is CPU buck converter TON pulses. value."]
    BUCKA = 31,
}
impl From<TMRA6CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: TMRA6CLK_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TMRA6CLK` reader - Counter/Timer A6 Clock Select."]
pub struct TMRA6CLK_R(crate::FieldReader<u8, TMRA6CLK_A>);
impl TMRA6CLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMRA6CLK_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMRA6CLK_A> {
        match self.bits {
            0 => Some(TMRA6CLK_A::TMRPIN),
            1 => Some(TMRA6CLK_A::HFRC_DIV4),
            2 => Some(TMRA6CLK_A::HFRC_DIV16),
            3 => Some(TMRA6CLK_A::HFRC_DIV256),
            4 => Some(TMRA6CLK_A::HFRC_DIV1024),
            5 => Some(TMRA6CLK_A::HFRC_DIV4K),
            6 => Some(TMRA6CLK_A::XT),
            7 => Some(TMRA6CLK_A::XT_DIV2),
            8 => Some(TMRA6CLK_A::XT_DIV16),
            9 => Some(TMRA6CLK_A::XT_DIV128),
            10 => Some(TMRA6CLK_A::LFRC_DIV2),
            11 => Some(TMRA6CLK_A::LFRC_DIV32),
            12 => Some(TMRA6CLK_A::LFRC_DIV1K),
            13 => Some(TMRA6CLK_A::LFRC),
            14 => Some(TMRA6CLK_A::RTC_100HZ),
            15 => Some(TMRA6CLK_A::HCLK_DIV4),
            16 => Some(TMRA6CLK_A::XT_DIV4),
            17 => Some(TMRA6CLK_A::XT_DIV8),
            18 => Some(TMRA6CLK_A::XT_DIV32),
            20 => Some(TMRA6CLK_A::CTMRB6),
            21 => Some(TMRA6CLK_A::CTMRA3),
            22 => Some(TMRA6CLK_A::CTMRB3),
            23 => Some(TMRA6CLK_A::CTMRA7),
            24 => Some(TMRA6CLK_A::CTMRB7),
            25 => Some(TMRA6CLK_A::CTMRB0),
            26 => Some(TMRA6CLK_A::CTMRB1),
            27 => Some(TMRA6CLK_A::CTMRB2),
            28 => Some(TMRA6CLK_A::CTMRB4),
            29 => Some(TMRA6CLK_A::BUCKBLE),
            30 => Some(TMRA6CLK_A::BUCKB),
            31 => Some(TMRA6CLK_A::BUCKA),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TMRPIN`"]
    #[inline(always)]
    pub fn is_tmrpin(&self) -> bool {
        **self == TMRA6CLK_A::TMRPIN
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == TMRA6CLK_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == TMRA6CLK_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == TMRA6CLK_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV1024`"]
    #[inline(always)]
    pub fn is_hfrc_div1024(&self) -> bool {
        **self == TMRA6CLK_A::HFRC_DIV1024
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_hfrc_div4k(&self) -> bool {
        **self == TMRA6CLK_A::HFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == TMRA6CLK_A::XT
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == TMRA6CLK_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == TMRA6CLK_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV128`"]
    #[inline(always)]
    pub fn is_xt_div128(&self) -> bool {
        **self == TMRA6CLK_A::XT_DIV128
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == TMRA6CLK_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == TMRA6CLK_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1K`"]
    #[inline(always)]
    pub fn is_lfrc_div1k(&self) -> bool {
        **self == TMRA6CLK_A::LFRC_DIV1K
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == TMRA6CLK_A::LFRC
    }
    #[doc = "Checks if the value of the field is `RTC_100HZ`"]
    #[inline(always)]
    pub fn is_rtc_100hz(&self) -> bool {
        **self == TMRA6CLK_A::RTC_100HZ
    }
    #[doc = "Checks if the value of the field is `HCLK_DIV4`"]
    #[inline(always)]
    pub fn is_hclk_div4(&self) -> bool {
        **self == TMRA6CLK_A::HCLK_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == TMRA6CLK_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == TMRA6CLK_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == TMRA6CLK_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `CTMRB6`"]
    #[inline(always)]
    pub fn is_ctmrb6(&self) -> bool {
        **self == TMRA6CLK_A::CTMRB6
    }
    #[doc = "Checks if the value of the field is `CTMRA3`"]
    #[inline(always)]
    pub fn is_ctmra3(&self) -> bool {
        **self == TMRA6CLK_A::CTMRA3
    }
    #[doc = "Checks if the value of the field is `CTMRB3`"]
    #[inline(always)]
    pub fn is_ctmrb3(&self) -> bool {
        **self == TMRA6CLK_A::CTMRB3
    }
    #[doc = "Checks if the value of the field is `CTMRA7`"]
    #[inline(always)]
    pub fn is_ctmra7(&self) -> bool {
        **self == TMRA6CLK_A::CTMRA7
    }
    #[doc = "Checks if the value of the field is `CTMRB7`"]
    #[inline(always)]
    pub fn is_ctmrb7(&self) -> bool {
        **self == TMRA6CLK_A::CTMRB7
    }
    #[doc = "Checks if the value of the field is `CTMRB0`"]
    #[inline(always)]
    pub fn is_ctmrb0(&self) -> bool {
        **self == TMRA6CLK_A::CTMRB0
    }
    #[doc = "Checks if the value of the field is `CTMRB1`"]
    #[inline(always)]
    pub fn is_ctmrb1(&self) -> bool {
        **self == TMRA6CLK_A::CTMRB1
    }
    #[doc = "Checks if the value of the field is `CTMRB2`"]
    #[inline(always)]
    pub fn is_ctmrb2(&self) -> bool {
        **self == TMRA6CLK_A::CTMRB2
    }
    #[doc = "Checks if the value of the field is `CTMRB4`"]
    #[inline(always)]
    pub fn is_ctmrb4(&self) -> bool {
        **self == TMRA6CLK_A::CTMRB4
    }
    #[doc = "Checks if the value of the field is `BUCKBLE`"]
    #[inline(always)]
    pub fn is_buckble(&self) -> bool {
        **self == TMRA6CLK_A::BUCKBLE
    }
    #[doc = "Checks if the value of the field is `BUCKB`"]
    #[inline(always)]
    pub fn is_buckb(&self) -> bool {
        **self == TMRA6CLK_A::BUCKB
    }
    #[doc = "Checks if the value of the field is `BUCKA`"]
    #[inline(always)]
    pub fn is_bucka(&self) -> bool {
        **self == TMRA6CLK_A::BUCKA
    }
}
impl core::ops::Deref for TMRA6CLK_R {
    type Target = crate::FieldReader<u8, TMRA6CLK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6CLK` writer - Counter/Timer A6 Clock Select."]
pub struct TMRA6CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6CLK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6CLK_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Clock source is TMRPINA. value."]
    #[inline(always)]
    pub fn tmrpin(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::TMRPIN)
    }
    #[doc = "Clock source is the HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::HFRC_DIV4)
    }
    #[doc = "Clock source is HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::HFRC_DIV16)
    }
    #[doc = "Clock source is HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::HFRC_DIV256)
    }
    #[doc = "Clock source is HFRC / 1024 value."]
    #[inline(always)]
    pub fn hfrc_div1024(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::HFRC_DIV1024)
    }
    #[doc = "Clock source is HFRC / 4096 value."]
    #[inline(always)]
    pub fn hfrc_div4k(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::HFRC_DIV4K)
    }
    #[doc = "Clock source is the XT (uncalibrated). value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::XT)
    }
    #[doc = "Clock source is XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::XT_DIV2)
    }
    #[doc = "Clock source is XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::XT_DIV16)
    }
    #[doc = "Clock source is XT / 128 value."]
    #[inline(always)]
    pub fn xt_div128(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::XT_DIV128)
    }
    #[doc = "Clock source is LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::LFRC_DIV2)
    }
    #[doc = "Clock source is LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::LFRC_DIV32)
    }
    #[doc = "Clock source is LFRC / 1024 value."]
    #[inline(always)]
    pub fn lfrc_div1k(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::LFRC_DIV1K)
    }
    #[doc = "Clock source is LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::LFRC)
    }
    #[doc = "Clock source is 100 Hz from the current RTC oscillator. value."]
    #[inline(always)]
    pub fn rtc_100hz(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::RTC_100HZ)
    }
    #[doc = "Clock source is HCLK / 4 (note: this clock is only available when MCU is in active mode) value."]
    #[inline(always)]
    pub fn hclk_div4(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::HCLK_DIV4)
    }
    #[doc = "Clock source is XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::XT_DIV4)
    }
    #[doc = "Clock source is XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::XT_DIV8)
    }
    #[doc = "Clock source is XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::XT_DIV32)
    }
    #[doc = "Clock source is CTIMERB6 OUT. value."]
    #[inline(always)]
    pub fn ctmrb6(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRB6)
    }
    #[doc = "Clock source is CTIMERA3 OUT. value."]
    #[inline(always)]
    pub fn ctmra3(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRA3)
    }
    #[doc = "Clock source is CTIMERB3 OUT. value."]
    #[inline(always)]
    pub fn ctmrb3(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRB3)
    }
    #[doc = "Clock source is CTIMERA7 OUT. value."]
    #[inline(always)]
    pub fn ctmra7(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRA7)
    }
    #[doc = "Clock source is CTIMERB7 OUT. value."]
    #[inline(always)]
    pub fn ctmrb7(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRB7)
    }
    #[doc = "Clock source is CTIMERB0 OUT. value."]
    #[inline(always)]
    pub fn ctmrb0(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRB0)
    }
    #[doc = "Clock source is CTIMERB1 OUT. value."]
    #[inline(always)]
    pub fn ctmrb1(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRB1)
    }
    #[doc = "Clock source is CTIMERB2 OUT. value."]
    #[inline(always)]
    pub fn ctmrb2(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRB2)
    }
    #[doc = "Clock source is CTIMERB4 OUT. value."]
    #[inline(always)]
    pub fn ctmrb4(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::CTMRB4)
    }
    #[doc = "Clock source is BLE buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckble(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::BUCKBLE)
    }
    #[doc = "Clock source is Memory buck converter TON pulses. value."]
    #[inline(always)]
    pub fn buckb(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::BUCKB)
    }
    #[doc = "Clock source is CPU buck converter TON pulses. value."]
    #[inline(always)]
    pub fn bucka(self) -> &'a mut W {
        self.variant(TMRA6CLK_A::BUCKA)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 1)) | ((value as u32 & 0x1f) << 1);
        self.w
    }
}
#[doc = "Counter/Timer A6 Enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMRA6EN_A {
    #[doc = "0: Counter/Timer A6 Disable. value."]
    DIS = 0,
    #[doc = "1: Counter/Timer A6 Enable. value."]
    EN = 1,
}
impl From<TMRA6EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMRA6EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TMRA6EN` reader - Counter/Timer A6 Enable bit."]
pub struct TMRA6EN_R(crate::FieldReader<bool, TMRA6EN_A>);
impl TMRA6EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMRA6EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMRA6EN_A {
        match self.bits {
            false => TMRA6EN_A::DIS,
            true => TMRA6EN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == TMRA6EN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == TMRA6EN_A::EN
    }
}
impl core::ops::Deref for TMRA6EN_R {
    type Target = crate::FieldReader<bool, TMRA6EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMRA6EN` writer - Counter/Timer A6 Enable bit."]
pub struct TMRA6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMRA6EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMRA6EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Counter/Timer A6 Disable. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(TMRA6EN_A::DIS)
    }
    #[doc = "Counter/Timer A6 Enable. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(TMRA6EN_A::EN)
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
    #[doc = "Bit 31 - Counter/Timer A6/B6 Link bit."]
    #[inline(always)]
    pub fn ctlink6(&self) -> CTLINK6_R {
        CTLINK6_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Counter/Timer B6 output polarity."]
    #[inline(always)]
    pub fn tmrb6pol(&self) -> TMRB6POL_R {
        TMRB6POL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Counter/Timer B6 Clear bit."]
    #[inline(always)]
    pub fn tmrb6clr(&self) -> TMRB6CLR_R {
        TMRB6CLR_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Counter/Timer B6 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb6ie1(&self) -> TMRB6IE1_R {
        TMRB6IE1_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Counter/Timer B6 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb6ie0(&self) -> TMRB6IE0_R {
        TMRB6IE0_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bits 22:24 - Counter/Timer B6 Function Select."]
    #[inline(always)]
    pub fn tmrb6fn(&self) -> TMRB6FN_R {
        TMRB6FN_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 17:21 - Counter/Timer B6 Clock Select."]
    #[inline(always)]
    pub fn tmrb6clk(&self) -> TMRB6CLK_R {
        TMRB6CLK_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Counter/Timer B6 Enable bit."]
    #[inline(always)]
    pub fn tmrb6en(&self) -> TMRB6EN_R {
        TMRB6EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Counter/Timer A6 output polarity."]
    #[inline(always)]
    pub fn tmra6pol(&self) -> TMRA6POL_R {
        TMRA6POL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Counter/Timer A6 Clear bit."]
    #[inline(always)]
    pub fn tmra6clr(&self) -> TMRA6CLR_R {
        TMRA6CLR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra6ie1(&self) -> TMRA6IE1_R {
        TMRA6IE1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra6ie0(&self) -> TMRA6IE0_R {
        TMRA6IE0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 6:8 - Counter/Timer A6 Function Select."]
    #[inline(always)]
    pub fn tmra6fn(&self) -> TMRA6FN_R {
        TMRA6FN_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 1:5 - Counter/Timer A6 Clock Select."]
    #[inline(always)]
    pub fn tmra6clk(&self) -> TMRA6CLK_R {
        TMRA6CLK_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bit 0 - Counter/Timer A6 Enable bit."]
    #[inline(always)]
    pub fn tmra6en(&self) -> TMRA6EN_R {
        TMRA6EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Counter/Timer A6/B6 Link bit."]
    #[inline(always)]
    pub fn ctlink6(&mut self) -> CTLINK6_W {
        CTLINK6_W { w: self }
    }
    #[doc = "Bit 28 - Counter/Timer B6 output polarity."]
    #[inline(always)]
    pub fn tmrb6pol(&mut self) -> TMRB6POL_W {
        TMRB6POL_W { w: self }
    }
    #[doc = "Bit 27 - Counter/Timer B6 Clear bit."]
    #[inline(always)]
    pub fn tmrb6clr(&mut self) -> TMRB6CLR_W {
        TMRB6CLR_W { w: self }
    }
    #[doc = "Bit 26 - Counter/Timer B6 Interrupt Enable bit for COMPR1."]
    #[inline(always)]
    pub fn tmrb6ie1(&mut self) -> TMRB6IE1_W {
        TMRB6IE1_W { w: self }
    }
    #[doc = "Bit 25 - Counter/Timer B6 Interrupt Enable bit for COMPR0."]
    #[inline(always)]
    pub fn tmrb6ie0(&mut self) -> TMRB6IE0_W {
        TMRB6IE0_W { w: self }
    }
    #[doc = "Bits 22:24 - Counter/Timer B6 Function Select."]
    #[inline(always)]
    pub fn tmrb6fn(&mut self) -> TMRB6FN_W {
        TMRB6FN_W { w: self }
    }
    #[doc = "Bits 17:21 - Counter/Timer B6 Clock Select."]
    #[inline(always)]
    pub fn tmrb6clk(&mut self) -> TMRB6CLK_W {
        TMRB6CLK_W { w: self }
    }
    #[doc = "Bit 16 - Counter/Timer B6 Enable bit."]
    #[inline(always)]
    pub fn tmrb6en(&mut self) -> TMRB6EN_W {
        TMRB6EN_W { w: self }
    }
    #[doc = "Bit 12 - Counter/Timer A6 output polarity."]
    #[inline(always)]
    pub fn tmra6pol(&mut self) -> TMRA6POL_W {
        TMRA6POL_W { w: self }
    }
    #[doc = "Bit 11 - Counter/Timer A6 Clear bit."]
    #[inline(always)]
    pub fn tmra6clr(&mut self) -> TMRA6CLR_W {
        TMRA6CLR_W { w: self }
    }
    #[doc = "Bit 10 - Counter/Timer A6 Interrupt Enable bit based on COMPR1."]
    #[inline(always)]
    pub fn tmra6ie1(&mut self) -> TMRA6IE1_W {
        TMRA6IE1_W { w: self }
    }
    #[doc = "Bit 9 - Counter/Timer A6 Interrupt Enable bit based on COMPR0."]
    #[inline(always)]
    pub fn tmra6ie0(&mut self) -> TMRA6IE0_W {
        TMRA6IE0_W { w: self }
    }
    #[doc = "Bits 6:8 - Counter/Timer A6 Function Select."]
    #[inline(always)]
    pub fn tmra6fn(&mut self) -> TMRA6FN_W {
        TMRA6FN_W { w: self }
    }
    #[doc = "Bits 1:5 - Counter/Timer A6 Clock Select."]
    #[inline(always)]
    pub fn tmra6clk(&mut self) -> TMRA6CLK_W {
        TMRA6CLK_W { w: self }
    }
    #[doc = "Bit 0 - Counter/Timer A6 Enable bit."]
    #[inline(always)]
    pub fn tmra6en(&mut self) -> TMRA6EN_W {
        TMRA6EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter/Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl6](index.html) module"]
pub struct CTRL6_SPEC;
impl crate::RegisterSpec for CTRL6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl6::R](R) reader structure"]
impl crate::Readable for CTRL6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl6::W](W) writer structure"]
impl crate::Writable for CTRL6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL6 to value 0"]
impl crate::Resettable for CTRL6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

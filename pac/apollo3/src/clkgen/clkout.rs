#[doc = "Register `CLKOUT` reader"]
pub struct R(crate::R<CLKOUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKOUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKOUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKOUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKOUT` writer"]
pub struct W(crate::W<CLKOUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKOUT_SPEC>;
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
impl From<crate::W<CLKOUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKOUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Enable the CLKOUT signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKEN_A {
    #[doc = "0: Disable CLKOUT value."]
    DIS = 0,
    #[doc = "1: Enable CLKOUT value."]
    EN = 1,
}
impl From<CKEN_A> for bool {
    #[inline(always)]
    fn from(variant: CKEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CKEN` reader - Enable the CLKOUT signal"]
pub struct CKEN_R(crate::FieldReader<bool, CKEN_A>);
impl CKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CKEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKEN_A {
        match self.bits {
            false => CKEN_A::DIS,
            true => CKEN_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == CKEN_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == CKEN_A::EN
    }
}
impl core::ops::Deref for CKEN_R {
    type Target = crate::FieldReader<bool, CKEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKEN` writer - Enable the CLKOUT signal"]
pub struct CKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable CLKOUT value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(CKEN_A::DIS)
    }
    #[doc = "Enable CLKOUT value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(CKEN_A::EN)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "CLKOUT signal select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKSEL_A {
    #[doc = "0: LFRC value."]
    LFRC = 0,
    #[doc = "1: XT / 2 value."]
    XT_DIV2 = 1,
    #[doc = "2: XT / 4 value."]
    XT_DIV4 = 2,
    #[doc = "3: XT / 8 value."]
    XT_DIV8 = 3,
    #[doc = "4: XT / 16 value."]
    XT_DIV16 = 4,
    #[doc = "5: XT / 32 value."]
    XT_DIV32 = 5,
    #[doc = "16: 1 Hz as selected in RTC value."]
    RTC_1HZ = 16,
    #[doc = "22: XT / 2^21 value."]
    XT_DIV2M = 22,
    #[doc = "23: XT value."]
    XT = 23,
    #[doc = "24: 100 Hz as selected in CLKGEN value."]
    CG_100HZ = 24,
    #[doc = "25: HFRC value."]
    HFRC = 25,
    #[doc = "26: HFRC / 4 value."]
    HFRC_DIV4 = 26,
    #[doc = "27: HFRC / 8 value."]
    HFRC_DIV8 = 27,
    #[doc = "28: HFRC / 16 value."]
    HFRC_DIV16 = 28,
    #[doc = "29: HFRC / 64 value."]
    HFRC_DIV64 = 29,
    #[doc = "30: HFRC / 128 value."]
    HFRC_DIV128 = 30,
    #[doc = "31: HFRC / 256 value."]
    HFRC_DIV256 = 31,
    #[doc = "32: HFRC / 512 value."]
    HFRC_DIV512 = 32,
    #[doc = "34: Flash Clock value."]
    FLASH_CLK = 34,
    #[doc = "35: LFRC / 2 value."]
    LFRC_DIV2 = 35,
    #[doc = "36: LFRC / 32 value."]
    LFRC_DIV32 = 36,
    #[doc = "37: LFRC / 512 value."]
    LFRC_DIV512 = 37,
    #[doc = "38: LFRC / 32768 value."]
    LFRC_DIV32K = 38,
    #[doc = "39: XT / 256 value."]
    XT_DIV256 = 39,
    #[doc = "40: XT / 8192 value."]
    XT_DIV8K = 40,
    #[doc = "41: XT / 2^16 value."]
    XT_DIV64K = 41,
    #[doc = "42: Uncal LFRC / 16 value."]
    ULFRC_DIV16 = 42,
    #[doc = "43: Uncal LFRC / 128 value."]
    ULFRC_DIV128 = 43,
    #[doc = "44: Uncal LFRC / 1024 value."]
    ULFRC_1HZ = 44,
    #[doc = "45: Uncal LFRC / 4096 value."]
    ULFRC_DIV4K = 45,
    #[doc = "46: Uncal LFRC / 2^20 value."]
    ULFRC_DIV1M = 46,
    #[doc = "47: HFRC / 2^16 value."]
    HFRC_DIV64K = 47,
    #[doc = "48: HFRC / 2^24 value."]
    HFRC_DIV16M = 48,
    #[doc = "49: LFRC / 2^20 value."]
    LFRC_DIV1M = 49,
    #[doc = "50: HFRC (not autoenabled) value."]
    HFRCNE = 50,
    #[doc = "51: HFRC / 8 (not autoenabled) value."]
    HFRCNE_DIV8 = 51,
    #[doc = "53: XT (not autoenabled) value."]
    XTNE = 53,
    #[doc = "54: XT / 16 (not autoenabled) value."]
    XTNE_DIV16 = 54,
    #[doc = "55: LFRC / 32 (not autoenabled) value."]
    LFRCNE_DIV32 = 55,
    #[doc = "57: LFRC (not autoenabled) - Default for undefined values value."]
    LFRCNE = 57,
}
impl From<CKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKSEL` reader - CLKOUT signal select"]
pub struct CKSEL_R(crate::FieldReader<u8, CKSEL_A>);
impl CKSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKSEL_A> {
        match self.bits {
            0 => Some(CKSEL_A::LFRC),
            1 => Some(CKSEL_A::XT_DIV2),
            2 => Some(CKSEL_A::XT_DIV4),
            3 => Some(CKSEL_A::XT_DIV8),
            4 => Some(CKSEL_A::XT_DIV16),
            5 => Some(CKSEL_A::XT_DIV32),
            16 => Some(CKSEL_A::RTC_1HZ),
            22 => Some(CKSEL_A::XT_DIV2M),
            23 => Some(CKSEL_A::XT),
            24 => Some(CKSEL_A::CG_100HZ),
            25 => Some(CKSEL_A::HFRC),
            26 => Some(CKSEL_A::HFRC_DIV4),
            27 => Some(CKSEL_A::HFRC_DIV8),
            28 => Some(CKSEL_A::HFRC_DIV16),
            29 => Some(CKSEL_A::HFRC_DIV64),
            30 => Some(CKSEL_A::HFRC_DIV128),
            31 => Some(CKSEL_A::HFRC_DIV256),
            32 => Some(CKSEL_A::HFRC_DIV512),
            34 => Some(CKSEL_A::FLASH_CLK),
            35 => Some(CKSEL_A::LFRC_DIV2),
            36 => Some(CKSEL_A::LFRC_DIV32),
            37 => Some(CKSEL_A::LFRC_DIV512),
            38 => Some(CKSEL_A::LFRC_DIV32K),
            39 => Some(CKSEL_A::XT_DIV256),
            40 => Some(CKSEL_A::XT_DIV8K),
            41 => Some(CKSEL_A::XT_DIV64K),
            42 => Some(CKSEL_A::ULFRC_DIV16),
            43 => Some(CKSEL_A::ULFRC_DIV128),
            44 => Some(CKSEL_A::ULFRC_1HZ),
            45 => Some(CKSEL_A::ULFRC_DIV4K),
            46 => Some(CKSEL_A::ULFRC_DIV1M),
            47 => Some(CKSEL_A::HFRC_DIV64K),
            48 => Some(CKSEL_A::HFRC_DIV16M),
            49 => Some(CKSEL_A::LFRC_DIV1M),
            50 => Some(CKSEL_A::HFRCNE),
            51 => Some(CKSEL_A::HFRCNE_DIV8),
            53 => Some(CKSEL_A::XTNE),
            54 => Some(CKSEL_A::XTNE_DIV16),
            55 => Some(CKSEL_A::LFRCNE_DIV32),
            57 => Some(CKSEL_A::LFRCNE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LFRC`"]
    #[inline(always)]
    pub fn is_lfrc(&self) -> bool {
        **self == CKSEL_A::LFRC
    }
    #[doc = "Checks if the value of the field is `XT_DIV2`"]
    #[inline(always)]
    pub fn is_xt_div2(&self) -> bool {
        **self == CKSEL_A::XT_DIV2
    }
    #[doc = "Checks if the value of the field is `XT_DIV4`"]
    #[inline(always)]
    pub fn is_xt_div4(&self) -> bool {
        **self == CKSEL_A::XT_DIV4
    }
    #[doc = "Checks if the value of the field is `XT_DIV8`"]
    #[inline(always)]
    pub fn is_xt_div8(&self) -> bool {
        **self == CKSEL_A::XT_DIV8
    }
    #[doc = "Checks if the value of the field is `XT_DIV16`"]
    #[inline(always)]
    pub fn is_xt_div16(&self) -> bool {
        **self == CKSEL_A::XT_DIV16
    }
    #[doc = "Checks if the value of the field is `XT_DIV32`"]
    #[inline(always)]
    pub fn is_xt_div32(&self) -> bool {
        **self == CKSEL_A::XT_DIV32
    }
    #[doc = "Checks if the value of the field is `RTC_1HZ`"]
    #[inline(always)]
    pub fn is_rtc_1hz(&self) -> bool {
        **self == CKSEL_A::RTC_1HZ
    }
    #[doc = "Checks if the value of the field is `XT_DIV2M`"]
    #[inline(always)]
    pub fn is_xt_div2m(&self) -> bool {
        **self == CKSEL_A::XT_DIV2M
    }
    #[doc = "Checks if the value of the field is `XT`"]
    #[inline(always)]
    pub fn is_xt(&self) -> bool {
        **self == CKSEL_A::XT
    }
    #[doc = "Checks if the value of the field is `CG_100HZ`"]
    #[inline(always)]
    pub fn is_cg_100hz(&self) -> bool {
        **self == CKSEL_A::CG_100HZ
    }
    #[doc = "Checks if the value of the field is `HFRC`"]
    #[inline(always)]
    pub fn is_hfrc(&self) -> bool {
        **self == CKSEL_A::HFRC
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV4`"]
    #[inline(always)]
    pub fn is_hfrc_div4(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV4
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV8`"]
    #[inline(always)]
    pub fn is_hfrc_div8(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV8
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16`"]
    #[inline(always)]
    pub fn is_hfrc_div16(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV64`"]
    #[inline(always)]
    pub fn is_hfrc_div64(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV64
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV128`"]
    #[inline(always)]
    pub fn is_hfrc_div128(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV256`"]
    #[inline(always)]
    pub fn is_hfrc_div256(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV256
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV512`"]
    #[inline(always)]
    pub fn is_hfrc_div512(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `FLASH_CLK`"]
    #[inline(always)]
    pub fn is_flash_clk(&self) -> bool {
        **self == CKSEL_A::FLASH_CLK
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV2`"]
    #[inline(always)]
    pub fn is_lfrc_div2(&self) -> bool {
        **self == CKSEL_A::LFRC_DIV2
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32`"]
    #[inline(always)]
    pub fn is_lfrc_div32(&self) -> bool {
        **self == CKSEL_A::LFRC_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV512`"]
    #[inline(always)]
    pub fn is_lfrc_div512(&self) -> bool {
        **self == CKSEL_A::LFRC_DIV512
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV32K`"]
    #[inline(always)]
    pub fn is_lfrc_div32k(&self) -> bool {
        **self == CKSEL_A::LFRC_DIV32K
    }
    #[doc = "Checks if the value of the field is `XT_DIV256`"]
    #[inline(always)]
    pub fn is_xt_div256(&self) -> bool {
        **self == CKSEL_A::XT_DIV256
    }
    #[doc = "Checks if the value of the field is `XT_DIV8K`"]
    #[inline(always)]
    pub fn is_xt_div8k(&self) -> bool {
        **self == CKSEL_A::XT_DIV8K
    }
    #[doc = "Checks if the value of the field is `XT_DIV64K`"]
    #[inline(always)]
    pub fn is_xt_div64k(&self) -> bool {
        **self == CKSEL_A::XT_DIV64K
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV16`"]
    #[inline(always)]
    pub fn is_ulfrc_div16(&self) -> bool {
        **self == CKSEL_A::ULFRC_DIV16
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV128`"]
    #[inline(always)]
    pub fn is_ulfrc_div128(&self) -> bool {
        **self == CKSEL_A::ULFRC_DIV128
    }
    #[doc = "Checks if the value of the field is `ULFRC_1HZ`"]
    #[inline(always)]
    pub fn is_ulfrc_1hz(&self) -> bool {
        **self == CKSEL_A::ULFRC_1HZ
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV4K`"]
    #[inline(always)]
    pub fn is_ulfrc_div4k(&self) -> bool {
        **self == CKSEL_A::ULFRC_DIV4K
    }
    #[doc = "Checks if the value of the field is `ULFRC_DIV1M`"]
    #[inline(always)]
    pub fn is_ulfrc_div1m(&self) -> bool {
        **self == CKSEL_A::ULFRC_DIV1M
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV64K`"]
    #[inline(always)]
    pub fn is_hfrc_div64k(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV64K
    }
    #[doc = "Checks if the value of the field is `HFRC_DIV16M`"]
    #[inline(always)]
    pub fn is_hfrc_div16m(&self) -> bool {
        **self == CKSEL_A::HFRC_DIV16M
    }
    #[doc = "Checks if the value of the field is `LFRC_DIV1M`"]
    #[inline(always)]
    pub fn is_lfrc_div1m(&self) -> bool {
        **self == CKSEL_A::LFRC_DIV1M
    }
    #[doc = "Checks if the value of the field is `HFRCNE`"]
    #[inline(always)]
    pub fn is_hfrcne(&self) -> bool {
        **self == CKSEL_A::HFRCNE
    }
    #[doc = "Checks if the value of the field is `HFRCNE_DIV8`"]
    #[inline(always)]
    pub fn is_hfrcne_div8(&self) -> bool {
        **self == CKSEL_A::HFRCNE_DIV8
    }
    #[doc = "Checks if the value of the field is `XTNE`"]
    #[inline(always)]
    pub fn is_xtne(&self) -> bool {
        **self == CKSEL_A::XTNE
    }
    #[doc = "Checks if the value of the field is `XTNE_DIV16`"]
    #[inline(always)]
    pub fn is_xtne_div16(&self) -> bool {
        **self == CKSEL_A::XTNE_DIV16
    }
    #[doc = "Checks if the value of the field is `LFRCNE_DIV32`"]
    #[inline(always)]
    pub fn is_lfrcne_div32(&self) -> bool {
        **self == CKSEL_A::LFRCNE_DIV32
    }
    #[doc = "Checks if the value of the field is `LFRCNE`"]
    #[inline(always)]
    pub fn is_lfrcne(&self) -> bool {
        **self == CKSEL_A::LFRCNE
    }
}
impl core::ops::Deref for CKSEL_R {
    type Target = crate::FieldReader<u8, CKSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKSEL` writer - CLKOUT signal select"]
pub struct CKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LFRC value."]
    #[inline(always)]
    pub fn lfrc(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRC)
    }
    #[doc = "XT / 2 value."]
    #[inline(always)]
    pub fn xt_div2(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV2)
    }
    #[doc = "XT / 4 value."]
    #[inline(always)]
    pub fn xt_div4(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV4)
    }
    #[doc = "XT / 8 value."]
    #[inline(always)]
    pub fn xt_div8(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV8)
    }
    #[doc = "XT / 16 value."]
    #[inline(always)]
    pub fn xt_div16(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV16)
    }
    #[doc = "XT / 32 value."]
    #[inline(always)]
    pub fn xt_div32(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV32)
    }
    #[doc = "1 Hz as selected in RTC value."]
    #[inline(always)]
    pub fn rtc_1hz(self) -> &'a mut W {
        self.variant(CKSEL_A::RTC_1HZ)
    }
    #[doc = "XT / 2^21 value."]
    #[inline(always)]
    pub fn xt_div2m(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV2M)
    }
    #[doc = "XT value."]
    #[inline(always)]
    pub fn xt(self) -> &'a mut W {
        self.variant(CKSEL_A::XT)
    }
    #[doc = "100 Hz as selected in CLKGEN value."]
    #[inline(always)]
    pub fn cg_100hz(self) -> &'a mut W {
        self.variant(CKSEL_A::CG_100HZ)
    }
    #[doc = "HFRC value."]
    #[inline(always)]
    pub fn hfrc(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC)
    }
    #[doc = "HFRC / 4 value."]
    #[inline(always)]
    pub fn hfrc_div4(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV4)
    }
    #[doc = "HFRC / 8 value."]
    #[inline(always)]
    pub fn hfrc_div8(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV8)
    }
    #[doc = "HFRC / 16 value."]
    #[inline(always)]
    pub fn hfrc_div16(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV16)
    }
    #[doc = "HFRC / 64 value."]
    #[inline(always)]
    pub fn hfrc_div64(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV64)
    }
    #[doc = "HFRC / 128 value."]
    #[inline(always)]
    pub fn hfrc_div128(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV128)
    }
    #[doc = "HFRC / 256 value."]
    #[inline(always)]
    pub fn hfrc_div256(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV256)
    }
    #[doc = "HFRC / 512 value."]
    #[inline(always)]
    pub fn hfrc_div512(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV512)
    }
    #[doc = "Flash Clock value."]
    #[inline(always)]
    pub fn flash_clk(self) -> &'a mut W {
        self.variant(CKSEL_A::FLASH_CLK)
    }
    #[doc = "LFRC / 2 value."]
    #[inline(always)]
    pub fn lfrc_div2(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRC_DIV2)
    }
    #[doc = "LFRC / 32 value."]
    #[inline(always)]
    pub fn lfrc_div32(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRC_DIV32)
    }
    #[doc = "LFRC / 512 value."]
    #[inline(always)]
    pub fn lfrc_div512(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRC_DIV512)
    }
    #[doc = "LFRC / 32768 value."]
    #[inline(always)]
    pub fn lfrc_div32k(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRC_DIV32K)
    }
    #[doc = "XT / 256 value."]
    #[inline(always)]
    pub fn xt_div256(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV256)
    }
    #[doc = "XT / 8192 value."]
    #[inline(always)]
    pub fn xt_div8k(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV8K)
    }
    #[doc = "XT / 2^16 value."]
    #[inline(always)]
    pub fn xt_div64k(self) -> &'a mut W {
        self.variant(CKSEL_A::XT_DIV64K)
    }
    #[doc = "Uncal LFRC / 16 value."]
    #[inline(always)]
    pub fn ulfrc_div16(self) -> &'a mut W {
        self.variant(CKSEL_A::ULFRC_DIV16)
    }
    #[doc = "Uncal LFRC / 128 value."]
    #[inline(always)]
    pub fn ulfrc_div128(self) -> &'a mut W {
        self.variant(CKSEL_A::ULFRC_DIV128)
    }
    #[doc = "Uncal LFRC / 1024 value."]
    #[inline(always)]
    pub fn ulfrc_1hz(self) -> &'a mut W {
        self.variant(CKSEL_A::ULFRC_1HZ)
    }
    #[doc = "Uncal LFRC / 4096 value."]
    #[inline(always)]
    pub fn ulfrc_div4k(self) -> &'a mut W {
        self.variant(CKSEL_A::ULFRC_DIV4K)
    }
    #[doc = "Uncal LFRC / 2^20 value."]
    #[inline(always)]
    pub fn ulfrc_div1m(self) -> &'a mut W {
        self.variant(CKSEL_A::ULFRC_DIV1M)
    }
    #[doc = "HFRC / 2^16 value."]
    #[inline(always)]
    pub fn hfrc_div64k(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV64K)
    }
    #[doc = "HFRC / 2^24 value."]
    #[inline(always)]
    pub fn hfrc_div16m(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRC_DIV16M)
    }
    #[doc = "LFRC / 2^20 value."]
    #[inline(always)]
    pub fn lfrc_div1m(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRC_DIV1M)
    }
    #[doc = "HFRC (not autoenabled) value."]
    #[inline(always)]
    pub fn hfrcne(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRCNE)
    }
    #[doc = "HFRC / 8 (not autoenabled) value."]
    #[inline(always)]
    pub fn hfrcne_div8(self) -> &'a mut W {
        self.variant(CKSEL_A::HFRCNE_DIV8)
    }
    #[doc = "XT (not autoenabled) value."]
    #[inline(always)]
    pub fn xtne(self) -> &'a mut W {
        self.variant(CKSEL_A::XTNE)
    }
    #[doc = "XT / 16 (not autoenabled) value."]
    #[inline(always)]
    pub fn xtne_div16(self) -> &'a mut W {
        self.variant(CKSEL_A::XTNE_DIV16)
    }
    #[doc = "LFRC / 32 (not autoenabled) value."]
    #[inline(always)]
    pub fn lfrcne_div32(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRCNE_DIV32)
    }
    #[doc = "LFRC (not autoenabled) - Default for undefined values value."]
    #[inline(always)]
    pub fn lfrcne(self) -> &'a mut W {
        self.variant(CKSEL_A::LFRCNE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Enable the CLKOUT signal"]
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 0:5 - CLKOUT signal select"]
    #[inline(always)]
    pub fn cksel(&self) -> CKSEL_R {
        CKSEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 7 - Enable the CLKOUT signal"]
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W {
        CKEN_W { w: self }
    }
    #[doc = "Bits 0:5 - CLKOUT signal select"]
    #[inline(always)]
    pub fn cksel(&mut self) -> CKSEL_W {
        CKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CLKOUT Frequency Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkout](index.html) module"]
pub struct CLKOUT_SPEC;
impl crate::RegisterSpec for CLKOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkout::R](R) reader structure"]
impl crate::Readable for CLKOUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkout::W](W) writer structure"]
impl crate::Writable for CLKOUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKOUT to value 0"]
impl crate::Resettable for CLKOUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

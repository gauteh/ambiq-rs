#[doc = "Register `SL6CFG` reader"]
pub struct R(crate::R<SL6CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SL6CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SL6CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SL6CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SL6CFG` writer"]
pub struct W(crate::W<SL6CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SL6CFG_SPEC>;
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
impl From<crate::W<SL6CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SL6CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select the number of measurements to average in the accumulate divide module for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADSEL6_A {
    #[doc = "0: Average in 1 measurement in the accumulate divide module for this slot. value."]
    AVG_1_MSRMT = 0,
    #[doc = "1: Average in 2 measurements in the accumulate divide module for this slot. value."]
    AVG_2_MSRMTS = 1,
    #[doc = "2: Average in 4 measurements in the accumulate divide module for this slot. value."]
    AVG_4_MSRMTS = 2,
    #[doc = "3: Average in 8 measurements in the accumulate divide module for this slot. value."]
    AVG_8_MSRMT = 3,
    #[doc = "4: Average in 16 measurements in the accumulate divide module for this slot. value."]
    AVG_16_MSRMTS = 4,
    #[doc = "5: Average in 32 measurements in the accumulate divide module for this slot. value."]
    AVG_32_MSRMTS = 5,
    #[doc = "6: Average in 64 measurements in the accumulate divide module for this slot. value."]
    AVG_64_MSRMTS = 6,
    #[doc = "7: Average in 128 measurements in the accumulate divide module for this slot. value."]
    AVG_128_MSRMTS = 7,
}
impl From<ADSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: ADSEL6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADSEL6` reader - Select the number of measurements to average in the accumulate divide module for this slot."]
pub struct ADSEL6_R(crate::FieldReader<u8, ADSEL6_A>);
impl ADSEL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADSEL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADSEL6_A {
        match self.bits {
            0 => ADSEL6_A::AVG_1_MSRMT,
            1 => ADSEL6_A::AVG_2_MSRMTS,
            2 => ADSEL6_A::AVG_4_MSRMTS,
            3 => ADSEL6_A::AVG_8_MSRMT,
            4 => ADSEL6_A::AVG_16_MSRMTS,
            5 => ADSEL6_A::AVG_32_MSRMTS,
            6 => ADSEL6_A::AVG_64_MSRMTS,
            7 => ADSEL6_A::AVG_128_MSRMTS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AVG_1_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_1_msrmt(&self) -> bool {
        **self == ADSEL6_A::AVG_1_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_2_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_2_msrmts(&self) -> bool {
        **self == ADSEL6_A::AVG_2_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_4_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_4_msrmts(&self) -> bool {
        **self == ADSEL6_A::AVG_4_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_8_MSRMT`"]
    #[inline(always)]
    pub fn is_avg_8_msrmt(&self) -> bool {
        **self == ADSEL6_A::AVG_8_MSRMT
    }
    #[doc = "Checks if the value of the field is `AVG_16_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_16_msrmts(&self) -> bool {
        **self == ADSEL6_A::AVG_16_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_32_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_32_msrmts(&self) -> bool {
        **self == ADSEL6_A::AVG_32_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_64_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_64_msrmts(&self) -> bool {
        **self == ADSEL6_A::AVG_64_MSRMTS
    }
    #[doc = "Checks if the value of the field is `AVG_128_MSRMTS`"]
    #[inline(always)]
    pub fn is_avg_128_msrmts(&self) -> bool {
        **self == ADSEL6_A::AVG_128_MSRMTS
    }
}
impl core::ops::Deref for ADSEL6_R {
    type Target = crate::FieldReader<u8, ADSEL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADSEL6` writer - Select the number of measurements to average in the accumulate divide module for this slot."]
pub struct ADSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> ADSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADSEL6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Average in 1 measurement in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_1_msrmt(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_1_MSRMT)
    }
    #[doc = "Average in 2 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_2_msrmts(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_2_MSRMTS)
    }
    #[doc = "Average in 4 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_4_msrmts(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_4_MSRMTS)
    }
    #[doc = "Average in 8 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_8_msrmt(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_8_MSRMT)
    }
    #[doc = "Average in 16 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_16_msrmts(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_16_MSRMTS)
    }
    #[doc = "Average in 32 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_32_msrmts(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_32_MSRMTS)
    }
    #[doc = "Average in 64 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_64_msrmts(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_64_MSRMTS)
    }
    #[doc = "Average in 128 measurements in the accumulate divide module for this slot. value."]
    #[inline(always)]
    pub fn avg_128_msrmts(self) -> &'a mut W {
        self.variant(ADSEL6_A::AVG_128_MSRMTS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Set the Precision Mode For Slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRMODE6_A {
    #[doc = "0: 14-bit precision mode value."]
    P14B = 0,
    #[doc = "1: 12-bit precision mode value."]
    P12B = 1,
    #[doc = "2: 10-bit precision mode value."]
    P10B = 2,
    #[doc = "3: 8-bit precision mode value."]
    P8B = 3,
}
impl From<PRMODE6_A> for u8 {
    #[inline(always)]
    fn from(variant: PRMODE6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PRMODE6` reader - Set the Precision Mode For Slot."]
pub struct PRMODE6_R(crate::FieldReader<u8, PRMODE6_A>);
impl PRMODE6_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRMODE6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRMODE6_A {
        match self.bits {
            0 => PRMODE6_A::P14B,
            1 => PRMODE6_A::P12B,
            2 => PRMODE6_A::P10B,
            3 => PRMODE6_A::P8B,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P14B`"]
    #[inline(always)]
    pub fn is_p14b(&self) -> bool {
        **self == PRMODE6_A::P14B
    }
    #[doc = "Checks if the value of the field is `P12B`"]
    #[inline(always)]
    pub fn is_p12b(&self) -> bool {
        **self == PRMODE6_A::P12B
    }
    #[doc = "Checks if the value of the field is `P10B`"]
    #[inline(always)]
    pub fn is_p10b(&self) -> bool {
        **self == PRMODE6_A::P10B
    }
    #[doc = "Checks if the value of the field is `P8B`"]
    #[inline(always)]
    pub fn is_p8b(&self) -> bool {
        **self == PRMODE6_A::P8B
    }
}
impl core::ops::Deref for PRMODE6_R {
    type Target = crate::FieldReader<u8, PRMODE6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRMODE6` writer - Set the Precision Mode For Slot."]
pub struct PRMODE6_W<'a> {
    w: &'a mut W,
}
impl<'a> PRMODE6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRMODE6_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "14-bit precision mode value."]
    #[inline(always)]
    pub fn p14b(self) -> &'a mut W {
        self.variant(PRMODE6_A::P14B)
    }
    #[doc = "12-bit precision mode value."]
    #[inline(always)]
    pub fn p12b(self) -> &'a mut W {
        self.variant(PRMODE6_A::P12B)
    }
    #[doc = "10-bit precision mode value."]
    #[inline(always)]
    pub fn p10b(self) -> &'a mut W {
        self.variant(PRMODE6_A::P10B)
    }
    #[doc = "8-bit precision mode value."]
    #[inline(always)]
    pub fn p8b(self) -> &'a mut W {
        self.variant(PRMODE6_A::P8B)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Select one of the 14 channel inputs for this slot.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CHSEL6_A {
    #[doc = "0: single ended external GPIO connection to pad16. value."]
    SE0 = 0,
    #[doc = "1: single ended external GPIO connection to pad29. value."]
    SE1 = 1,
    #[doc = "2: single ended external GPIO connection to pad11. value."]
    SE2 = 2,
    #[doc = "3: single ended external GPIO connection to pad31. value."]
    SE3 = 3,
    #[doc = "4: single ended external GPIO connection to pad32. value."]
    SE4 = 4,
    #[doc = "5: single ended external GPIO connection to pad33. value."]
    SE5 = 5,
    #[doc = "6: single ended external GPIO connection to pad34. value."]
    SE6 = 6,
    #[doc = "7: single ended external GPIO connection to pad35. value."]
    SE7 = 7,
    #[doc = "8: single ended external GPIO connection to pad13. value."]
    SE8 = 8,
    #[doc = "9: single ended external GPIO connection to pad12. value."]
    SE9 = 9,
    #[doc = "10: differential external GPIO connections to pad12(N) and pad13(P). value."]
    DF0 = 10,
    #[doc = "11: differential external GPIO connections to pad15(N) and pad14(P). value."]
    DF1 = 11,
    #[doc = "12: internal temperature sensor. value."]
    TEMP = 12,
    #[doc = "13: internal voltage divide-by-3 connection. value."]
    BATT = 13,
    #[doc = "14: Input VSS value."]
    VSS = 14,
}
impl From<CHSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: CHSEL6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CHSEL6` reader - Select one of the 14 channel inputs for this slot."]
pub struct CHSEL6_R(crate::FieldReader<u8, CHSEL6_A>);
impl CHSEL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        CHSEL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CHSEL6_A> {
        match self.bits {
            0 => Some(CHSEL6_A::SE0),
            1 => Some(CHSEL6_A::SE1),
            2 => Some(CHSEL6_A::SE2),
            3 => Some(CHSEL6_A::SE3),
            4 => Some(CHSEL6_A::SE4),
            5 => Some(CHSEL6_A::SE5),
            6 => Some(CHSEL6_A::SE6),
            7 => Some(CHSEL6_A::SE7),
            8 => Some(CHSEL6_A::SE8),
            9 => Some(CHSEL6_A::SE9),
            10 => Some(CHSEL6_A::DF0),
            11 => Some(CHSEL6_A::DF1),
            12 => Some(CHSEL6_A::TEMP),
            13 => Some(CHSEL6_A::BATT),
            14 => Some(CHSEL6_A::VSS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SE0`"]
    #[inline(always)]
    pub fn is_se0(&self) -> bool {
        **self == CHSEL6_A::SE0
    }
    #[doc = "Checks if the value of the field is `SE1`"]
    #[inline(always)]
    pub fn is_se1(&self) -> bool {
        **self == CHSEL6_A::SE1
    }
    #[doc = "Checks if the value of the field is `SE2`"]
    #[inline(always)]
    pub fn is_se2(&self) -> bool {
        **self == CHSEL6_A::SE2
    }
    #[doc = "Checks if the value of the field is `SE3`"]
    #[inline(always)]
    pub fn is_se3(&self) -> bool {
        **self == CHSEL6_A::SE3
    }
    #[doc = "Checks if the value of the field is `SE4`"]
    #[inline(always)]
    pub fn is_se4(&self) -> bool {
        **self == CHSEL6_A::SE4
    }
    #[doc = "Checks if the value of the field is `SE5`"]
    #[inline(always)]
    pub fn is_se5(&self) -> bool {
        **self == CHSEL6_A::SE5
    }
    #[doc = "Checks if the value of the field is `SE6`"]
    #[inline(always)]
    pub fn is_se6(&self) -> bool {
        **self == CHSEL6_A::SE6
    }
    #[doc = "Checks if the value of the field is `SE7`"]
    #[inline(always)]
    pub fn is_se7(&self) -> bool {
        **self == CHSEL6_A::SE7
    }
    #[doc = "Checks if the value of the field is `SE8`"]
    #[inline(always)]
    pub fn is_se8(&self) -> bool {
        **self == CHSEL6_A::SE8
    }
    #[doc = "Checks if the value of the field is `SE9`"]
    #[inline(always)]
    pub fn is_se9(&self) -> bool {
        **self == CHSEL6_A::SE9
    }
    #[doc = "Checks if the value of the field is `DF0`"]
    #[inline(always)]
    pub fn is_df0(&self) -> bool {
        **self == CHSEL6_A::DF0
    }
    #[doc = "Checks if the value of the field is `DF1`"]
    #[inline(always)]
    pub fn is_df1(&self) -> bool {
        **self == CHSEL6_A::DF1
    }
    #[doc = "Checks if the value of the field is `TEMP`"]
    #[inline(always)]
    pub fn is_temp(&self) -> bool {
        **self == CHSEL6_A::TEMP
    }
    #[doc = "Checks if the value of the field is `BATT`"]
    #[inline(always)]
    pub fn is_batt(&self) -> bool {
        **self == CHSEL6_A::BATT
    }
    #[doc = "Checks if the value of the field is `VSS`"]
    #[inline(always)]
    pub fn is_vss(&self) -> bool {
        **self == CHSEL6_A::VSS
    }
}
impl core::ops::Deref for CHSEL6_R {
    type Target = crate::FieldReader<u8, CHSEL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CHSEL6` writer - Select one of the 14 channel inputs for this slot."]
pub struct CHSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> CHSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CHSEL6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "single ended external GPIO connection to pad16. value."]
    #[inline(always)]
    pub fn se0(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE0)
    }
    #[doc = "single ended external GPIO connection to pad29. value."]
    #[inline(always)]
    pub fn se1(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE1)
    }
    #[doc = "single ended external GPIO connection to pad11. value."]
    #[inline(always)]
    pub fn se2(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE2)
    }
    #[doc = "single ended external GPIO connection to pad31. value."]
    #[inline(always)]
    pub fn se3(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE3)
    }
    #[doc = "single ended external GPIO connection to pad32. value."]
    #[inline(always)]
    pub fn se4(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE4)
    }
    #[doc = "single ended external GPIO connection to pad33. value."]
    #[inline(always)]
    pub fn se5(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE5)
    }
    #[doc = "single ended external GPIO connection to pad34. value."]
    #[inline(always)]
    pub fn se6(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE6)
    }
    #[doc = "single ended external GPIO connection to pad35. value."]
    #[inline(always)]
    pub fn se7(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE7)
    }
    #[doc = "single ended external GPIO connection to pad13. value."]
    #[inline(always)]
    pub fn se8(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE8)
    }
    #[doc = "single ended external GPIO connection to pad12. value."]
    #[inline(always)]
    pub fn se9(self) -> &'a mut W {
        self.variant(CHSEL6_A::SE9)
    }
    #[doc = "differential external GPIO connections to pad12(N) and pad13(P). value."]
    #[inline(always)]
    pub fn df0(self) -> &'a mut W {
        self.variant(CHSEL6_A::DF0)
    }
    #[doc = "differential external GPIO connections to pad15(N) and pad14(P). value."]
    #[inline(always)]
    pub fn df1(self) -> &'a mut W {
        self.variant(CHSEL6_A::DF1)
    }
    #[doc = "internal temperature sensor. value."]
    #[inline(always)]
    pub fn temp(self) -> &'a mut W {
        self.variant(CHSEL6_A::TEMP)
    }
    #[doc = "internal voltage divide-by-3 connection. value."]
    #[inline(always)]
    pub fn batt(self) -> &'a mut W {
        self.variant(CHSEL6_A::BATT)
    }
    #[doc = "Input VSS value."]
    #[inline(always)]
    pub fn vss(self) -> &'a mut W {
        self.variant(CHSEL6_A::VSS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "This bit enables the window compare function for slot 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WCEN6_A {
    #[doc = "1: Enable the window compare for slot 6. value."]
    WCEN = 1,
}
impl From<WCEN6_A> for bool {
    #[inline(always)]
    fn from(variant: WCEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WCEN6` reader - This bit enables the window compare function for slot 6."]
pub struct WCEN6_R(crate::FieldReader<bool, WCEN6_A>);
impl WCEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        WCEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WCEN6_A> {
        match self.bits {
            true => Some(WCEN6_A::WCEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WCEN`"]
    #[inline(always)]
    pub fn is_wcen(&self) -> bool {
        **self == WCEN6_A::WCEN
    }
}
impl core::ops::Deref for WCEN6_R {
    type Target = crate::FieldReader<bool, WCEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WCEN6` writer - This bit enables the window compare function for slot 6."]
pub struct WCEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> WCEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WCEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable the window compare for slot 6. value."]
    #[inline(always)]
    pub fn wcen(self) -> &'a mut W {
        self.variant(WCEN6_A::WCEN)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "This bit enables slot 6 for ADC conversions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEN6_A {
    #[doc = "1: Enable slot 6 for ADC conversions. value."]
    SLEN = 1,
}
impl From<SLEN6_A> for bool {
    #[inline(always)]
    fn from(variant: SLEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEN6` reader - This bit enables slot 6 for ADC conversions."]
pub struct SLEN6_R(crate::FieldReader<bool, SLEN6_A>);
impl SLEN6_R {
    pub(crate) fn new(bits: bool) -> Self {
        SLEN6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SLEN6_A> {
        match self.bits {
            true => Some(SLEN6_A::SLEN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SLEN`"]
    #[inline(always)]
    pub fn is_slen(&self) -> bool {
        **self == SLEN6_A::SLEN
    }
}
impl core::ops::Deref for SLEN6_R {
    type Target = crate::FieldReader<bool, SLEN6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SLEN6` writer - This bit enables slot 6 for ADC conversions."]
pub struct SLEN6_W<'a> {
    w: &'a mut W,
}
impl<'a> SLEN6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SLEN6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable slot 6 for ADC conversions. value."]
    #[inline(always)]
    pub fn slen(self) -> &'a mut W {
        self.variant(SLEN6_A::SLEN)
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
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel6(&self) -> ADSEL6_R {
        ADSEL6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode6(&self) -> PRMODE6_R {
        PRMODE6_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel6(&self) -> CHSEL6_R {
        CHSEL6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 6."]
    #[inline(always)]
    pub fn wcen6(&self) -> WCEN6_R {
        WCEN6_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - This bit enables slot 6 for ADC conversions."]
    #[inline(always)]
    pub fn slen6(&self) -> SLEN6_R {
        SLEN6_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 24:26 - Select the number of measurements to average in the accumulate divide module for this slot."]
    #[inline(always)]
    pub fn adsel6(&mut self) -> ADSEL6_W {
        ADSEL6_W { w: self }
    }
    #[doc = "Bits 16:17 - Set the Precision Mode For Slot."]
    #[inline(always)]
    pub fn prmode6(&mut self) -> PRMODE6_W {
        PRMODE6_W { w: self }
    }
    #[doc = "Bits 8:11 - Select one of the 14 channel inputs for this slot."]
    #[inline(always)]
    pub fn chsel6(&mut self) -> CHSEL6_W {
        CHSEL6_W { w: self }
    }
    #[doc = "Bit 1 - This bit enables the window compare function for slot 6."]
    #[inline(always)]
    pub fn wcen6(&mut self) -> WCEN6_W {
        WCEN6_W { w: self }
    }
    #[doc = "Bit 0 - This bit enables slot 6 for ADC conversions."]
    #[inline(always)]
    pub fn slen6(&mut self) -> SLEN6_W {
        SLEN6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slot 6 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sl6cfg](index.html) module"]
pub struct SL6CFG_SPEC;
impl crate::RegisterSpec for SL6CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sl6cfg::R](R) reader structure"]
impl crate::Readable for SL6CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sl6cfg::W](W) writer structure"]
impl crate::Writable for SL6CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SL6CFG to value 0"]
impl crate::Resettable for SL6CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

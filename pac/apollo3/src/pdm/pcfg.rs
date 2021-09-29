#[doc = "Register `PCFG` reader"]
pub struct R(crate::R<PCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCFG` writer"]
pub struct W(crate::W<PCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCFG_SPEC>;
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
impl From<crate::W<PCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Left/right channel swap.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LRSWAP_A {
    #[doc = "1: Swap left and right channels (FIFO Read RIGHT_LEFT). value."]
    EN = 1,
    #[doc = "0: No channel swapping (IFO Read LEFT_RIGHT). value."]
    NOSWAP = 0,
}
impl From<LRSWAP_A> for bool {
    #[inline(always)]
    fn from(variant: LRSWAP_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LRSWAP` reader - Left/right channel swap."]
pub struct LRSWAP_R(crate::FieldReader<bool, LRSWAP_A>);
impl LRSWAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        LRSWAP_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRSWAP_A {
        match self.bits {
            true => LRSWAP_A::EN,
            false => LRSWAP_A::NOSWAP,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == LRSWAP_A::EN
    }
    #[doc = "Checks if the value of the field is `NOSWAP`"]
    #[inline(always)]
    pub fn is_noswap(&self) -> bool {
        **self == LRSWAP_A::NOSWAP
    }
}
impl core::ops::Deref for LRSWAP_R {
    type Target = crate::FieldReader<bool, LRSWAP_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LRSWAP` writer - Left/right channel swap."]
pub struct LRSWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> LRSWAP_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LRSWAP_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Swap left and right channels (FIFO Read RIGHT_LEFT). value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(LRSWAP_A::EN)
    }
    #[doc = "No channel swapping (IFO Read LEFT_RIGHT). value."]
    #[inline(always)]
    pub fn noswap(self) -> &'a mut W {
        self.variant(LRSWAP_A::NOSWAP)
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
#[doc = "Right channel PGA gain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGARIGHT_A {
    #[doc = "31: 40.5 db gain. value."]
    P405DB = 31,
    #[doc = "30: 39.0 db gain. value."]
    P390DB = 30,
    #[doc = "29: 37.5 db gain. value."]
    P375DB = 29,
    #[doc = "28: 36.0 db gain. value."]
    P360DB = 28,
    #[doc = "27: 34.5 db gain. value."]
    P345DB = 27,
    #[doc = "26: 33.0 db gain. value."]
    P330DB = 26,
    #[doc = "25: 31.5 db gain. value."]
    P315DB = 25,
    #[doc = "24: 30.0 db gain. value."]
    P300DB = 24,
    #[doc = "23: 28.5 db gain. value."]
    P285DB = 23,
    #[doc = "22: 27.0 db gain. value."]
    P270DB = 22,
    #[doc = "21: 25.5 db gain. value."]
    P255DB = 21,
    #[doc = "20: 24.0 db gain. value."]
    P240DB = 20,
    #[doc = "19: 22.5 db gain. value."]
    P225DB = 19,
    #[doc = "18: 21.0 db gain. value."]
    P210DB = 18,
    #[doc = "17: 19.5 db gain. value."]
    P195DB = 17,
    #[doc = "16: 18.0 db gain. value."]
    P180DB = 16,
    #[doc = "15: 16.5 db gain. value."]
    P165DB = 15,
    #[doc = "14: 15.0 db gain. value."]
    P150DB = 14,
    #[doc = "13: 13.5 db gain. value."]
    P135DB = 13,
    #[doc = "12: 12.0 db gain. value."]
    P120DB = 12,
    #[doc = "11: 10.5 db gain. value."]
    P105DB = 11,
    #[doc = "10: 9.0 db gain. value."]
    P90DB = 10,
    #[doc = "9: 7.5 db gain. value."]
    P75DB = 9,
    #[doc = "8: 6.0 db gain. value."]
    P60DB = 8,
    #[doc = "7: 4.5 db gain. value."]
    P45DB = 7,
    #[doc = "6: 3.0 db gain. value."]
    P30DB = 6,
    #[doc = "5: 1.5 db gain. value."]
    P15DB = 5,
    #[doc = "4: 0.0 db gain. value."]
    _0DB = 4,
    #[doc = "3: -1.5 db gain. value."]
    M15DB = 3,
    #[doc = "2: -3.0 db gain. value."]
    M300DB = 2,
    #[doc = "1: -4.5 db gain. value."]
    M45DB = 1,
    #[doc = "0: -6.0 db gain. value."]
    M60DB = 0,
}
impl From<PGARIGHT_A> for u8 {
    #[inline(always)]
    fn from(variant: PGARIGHT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PGARIGHT` reader - Right channel PGA gain."]
pub struct PGARIGHT_R(crate::FieldReader<u8, PGARIGHT_A>);
impl PGARIGHT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGARIGHT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGARIGHT_A {
        match self.bits {
            31 => PGARIGHT_A::P405DB,
            30 => PGARIGHT_A::P390DB,
            29 => PGARIGHT_A::P375DB,
            28 => PGARIGHT_A::P360DB,
            27 => PGARIGHT_A::P345DB,
            26 => PGARIGHT_A::P330DB,
            25 => PGARIGHT_A::P315DB,
            24 => PGARIGHT_A::P300DB,
            23 => PGARIGHT_A::P285DB,
            22 => PGARIGHT_A::P270DB,
            21 => PGARIGHT_A::P255DB,
            20 => PGARIGHT_A::P240DB,
            19 => PGARIGHT_A::P225DB,
            18 => PGARIGHT_A::P210DB,
            17 => PGARIGHT_A::P195DB,
            16 => PGARIGHT_A::P180DB,
            15 => PGARIGHT_A::P165DB,
            14 => PGARIGHT_A::P150DB,
            13 => PGARIGHT_A::P135DB,
            12 => PGARIGHT_A::P120DB,
            11 => PGARIGHT_A::P105DB,
            10 => PGARIGHT_A::P90DB,
            9 => PGARIGHT_A::P75DB,
            8 => PGARIGHT_A::P60DB,
            7 => PGARIGHT_A::P45DB,
            6 => PGARIGHT_A::P30DB,
            5 => PGARIGHT_A::P15DB,
            4 => PGARIGHT_A::_0DB,
            3 => PGARIGHT_A::M15DB,
            2 => PGARIGHT_A::M300DB,
            1 => PGARIGHT_A::M45DB,
            0 => PGARIGHT_A::M60DB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P405DB`"]
    #[inline(always)]
    pub fn is_p405db(&self) -> bool {
        **self == PGARIGHT_A::P405DB
    }
    #[doc = "Checks if the value of the field is `P390DB`"]
    #[inline(always)]
    pub fn is_p390db(&self) -> bool {
        **self == PGARIGHT_A::P390DB
    }
    #[doc = "Checks if the value of the field is `P375DB`"]
    #[inline(always)]
    pub fn is_p375db(&self) -> bool {
        **self == PGARIGHT_A::P375DB
    }
    #[doc = "Checks if the value of the field is `P360DB`"]
    #[inline(always)]
    pub fn is_p360db(&self) -> bool {
        **self == PGARIGHT_A::P360DB
    }
    #[doc = "Checks if the value of the field is `P345DB`"]
    #[inline(always)]
    pub fn is_p345db(&self) -> bool {
        **self == PGARIGHT_A::P345DB
    }
    #[doc = "Checks if the value of the field is `P330DB`"]
    #[inline(always)]
    pub fn is_p330db(&self) -> bool {
        **self == PGARIGHT_A::P330DB
    }
    #[doc = "Checks if the value of the field is `P315DB`"]
    #[inline(always)]
    pub fn is_p315db(&self) -> bool {
        **self == PGARIGHT_A::P315DB
    }
    #[doc = "Checks if the value of the field is `P300DB`"]
    #[inline(always)]
    pub fn is_p300db(&self) -> bool {
        **self == PGARIGHT_A::P300DB
    }
    #[doc = "Checks if the value of the field is `P285DB`"]
    #[inline(always)]
    pub fn is_p285db(&self) -> bool {
        **self == PGARIGHT_A::P285DB
    }
    #[doc = "Checks if the value of the field is `P270DB`"]
    #[inline(always)]
    pub fn is_p270db(&self) -> bool {
        **self == PGARIGHT_A::P270DB
    }
    #[doc = "Checks if the value of the field is `P255DB`"]
    #[inline(always)]
    pub fn is_p255db(&self) -> bool {
        **self == PGARIGHT_A::P255DB
    }
    #[doc = "Checks if the value of the field is `P240DB`"]
    #[inline(always)]
    pub fn is_p240db(&self) -> bool {
        **self == PGARIGHT_A::P240DB
    }
    #[doc = "Checks if the value of the field is `P225DB`"]
    #[inline(always)]
    pub fn is_p225db(&self) -> bool {
        **self == PGARIGHT_A::P225DB
    }
    #[doc = "Checks if the value of the field is `P210DB`"]
    #[inline(always)]
    pub fn is_p210db(&self) -> bool {
        **self == PGARIGHT_A::P210DB
    }
    #[doc = "Checks if the value of the field is `P195DB`"]
    #[inline(always)]
    pub fn is_p195db(&self) -> bool {
        **self == PGARIGHT_A::P195DB
    }
    #[doc = "Checks if the value of the field is `P180DB`"]
    #[inline(always)]
    pub fn is_p180db(&self) -> bool {
        **self == PGARIGHT_A::P180DB
    }
    #[doc = "Checks if the value of the field is `P165DB`"]
    #[inline(always)]
    pub fn is_p165db(&self) -> bool {
        **self == PGARIGHT_A::P165DB
    }
    #[doc = "Checks if the value of the field is `P150DB`"]
    #[inline(always)]
    pub fn is_p150db(&self) -> bool {
        **self == PGARIGHT_A::P150DB
    }
    #[doc = "Checks if the value of the field is `P135DB`"]
    #[inline(always)]
    pub fn is_p135db(&self) -> bool {
        **self == PGARIGHT_A::P135DB
    }
    #[doc = "Checks if the value of the field is `P120DB`"]
    #[inline(always)]
    pub fn is_p120db(&self) -> bool {
        **self == PGARIGHT_A::P120DB
    }
    #[doc = "Checks if the value of the field is `P105DB`"]
    #[inline(always)]
    pub fn is_p105db(&self) -> bool {
        **self == PGARIGHT_A::P105DB
    }
    #[doc = "Checks if the value of the field is `P90DB`"]
    #[inline(always)]
    pub fn is_p90db(&self) -> bool {
        **self == PGARIGHT_A::P90DB
    }
    #[doc = "Checks if the value of the field is `P75DB`"]
    #[inline(always)]
    pub fn is_p75db(&self) -> bool {
        **self == PGARIGHT_A::P75DB
    }
    #[doc = "Checks if the value of the field is `P60DB`"]
    #[inline(always)]
    pub fn is_p60db(&self) -> bool {
        **self == PGARIGHT_A::P60DB
    }
    #[doc = "Checks if the value of the field is `P45DB`"]
    #[inline(always)]
    pub fn is_p45db(&self) -> bool {
        **self == PGARIGHT_A::P45DB
    }
    #[doc = "Checks if the value of the field is `P30DB`"]
    #[inline(always)]
    pub fn is_p30db(&self) -> bool {
        **self == PGARIGHT_A::P30DB
    }
    #[doc = "Checks if the value of the field is `P15DB`"]
    #[inline(always)]
    pub fn is_p15db(&self) -> bool {
        **self == PGARIGHT_A::P15DB
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        **self == PGARIGHT_A::_0DB
    }
    #[doc = "Checks if the value of the field is `M15DB`"]
    #[inline(always)]
    pub fn is_m15db(&self) -> bool {
        **self == PGARIGHT_A::M15DB
    }
    #[doc = "Checks if the value of the field is `M300DB`"]
    #[inline(always)]
    pub fn is_m300db(&self) -> bool {
        **self == PGARIGHT_A::M300DB
    }
    #[doc = "Checks if the value of the field is `M45DB`"]
    #[inline(always)]
    pub fn is_m45db(&self) -> bool {
        **self == PGARIGHT_A::M45DB
    }
    #[doc = "Checks if the value of the field is `M60DB`"]
    #[inline(always)]
    pub fn is_m60db(&self) -> bool {
        **self == PGARIGHT_A::M60DB
    }
}
impl core::ops::Deref for PGARIGHT_R {
    type Target = crate::FieldReader<u8, PGARIGHT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGARIGHT` writer - Right channel PGA gain."]
pub struct PGARIGHT_W<'a> {
    w: &'a mut W,
}
impl<'a> PGARIGHT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGARIGHT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "40.5 db gain. value."]
    #[inline(always)]
    pub fn p405db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P405DB)
    }
    #[doc = "39.0 db gain. value."]
    #[inline(always)]
    pub fn p390db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P390DB)
    }
    #[doc = "37.5 db gain. value."]
    #[inline(always)]
    pub fn p375db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P375DB)
    }
    #[doc = "36.0 db gain. value."]
    #[inline(always)]
    pub fn p360db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P360DB)
    }
    #[doc = "34.5 db gain. value."]
    #[inline(always)]
    pub fn p345db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P345DB)
    }
    #[doc = "33.0 db gain. value."]
    #[inline(always)]
    pub fn p330db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P330DB)
    }
    #[doc = "31.5 db gain. value."]
    #[inline(always)]
    pub fn p315db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P315DB)
    }
    #[doc = "30.0 db gain. value."]
    #[inline(always)]
    pub fn p300db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P300DB)
    }
    #[doc = "28.5 db gain. value."]
    #[inline(always)]
    pub fn p285db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P285DB)
    }
    #[doc = "27.0 db gain. value."]
    #[inline(always)]
    pub fn p270db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P270DB)
    }
    #[doc = "25.5 db gain. value."]
    #[inline(always)]
    pub fn p255db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P255DB)
    }
    #[doc = "24.0 db gain. value."]
    #[inline(always)]
    pub fn p240db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P240DB)
    }
    #[doc = "22.5 db gain. value."]
    #[inline(always)]
    pub fn p225db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P225DB)
    }
    #[doc = "21.0 db gain. value."]
    #[inline(always)]
    pub fn p210db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P210DB)
    }
    #[doc = "19.5 db gain. value."]
    #[inline(always)]
    pub fn p195db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P195DB)
    }
    #[doc = "18.0 db gain. value."]
    #[inline(always)]
    pub fn p180db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P180DB)
    }
    #[doc = "16.5 db gain. value."]
    #[inline(always)]
    pub fn p165db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P165DB)
    }
    #[doc = "15.0 db gain. value."]
    #[inline(always)]
    pub fn p150db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P150DB)
    }
    #[doc = "13.5 db gain. value."]
    #[inline(always)]
    pub fn p135db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P135DB)
    }
    #[doc = "12.0 db gain. value."]
    #[inline(always)]
    pub fn p120db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P120DB)
    }
    #[doc = "10.5 db gain. value."]
    #[inline(always)]
    pub fn p105db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P105DB)
    }
    #[doc = "9.0 db gain. value."]
    #[inline(always)]
    pub fn p90db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P90DB)
    }
    #[doc = "7.5 db gain. value."]
    #[inline(always)]
    pub fn p75db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P75DB)
    }
    #[doc = "6.0 db gain. value."]
    #[inline(always)]
    pub fn p60db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P60DB)
    }
    #[doc = "4.5 db gain. value."]
    #[inline(always)]
    pub fn p45db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P45DB)
    }
    #[doc = "3.0 db gain. value."]
    #[inline(always)]
    pub fn p30db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P30DB)
    }
    #[doc = "1.5 db gain. value."]
    #[inline(always)]
    pub fn p15db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::P15DB)
    }
    #[doc = "0.0 db gain. value."]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::_0DB)
    }
    #[doc = "-1.5 db gain. value."]
    #[inline(always)]
    pub fn m15db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M15DB)
    }
    #[doc = "-3.0 db gain. value."]
    #[inline(always)]
    pub fn m300db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M300DB)
    }
    #[doc = "-4.5 db gain. value."]
    #[inline(always)]
    pub fn m45db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M45DB)
    }
    #[doc = "-6.0 db gain. value."]
    #[inline(always)]
    pub fn m60db(self) -> &'a mut W {
        self.variant(PGARIGHT_A::M60DB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Left channel PGA gain.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PGALEFT_A {
    #[doc = "31: 40.5 db gain. value."]
    P405DB = 31,
    #[doc = "30: 39.0 db gain. value."]
    P390DB = 30,
    #[doc = "29: 37.5 db gain. value."]
    P375DB = 29,
    #[doc = "28: 36.0 db gain. value."]
    P360DB = 28,
    #[doc = "27: 34.5 db gain. value."]
    P345DB = 27,
    #[doc = "26: 33.0 db gain. value."]
    P330DB = 26,
    #[doc = "25: 31.5 db gain. value."]
    P315DB = 25,
    #[doc = "24: 30.0 db gain. value."]
    P300DB = 24,
    #[doc = "23: 28.5 db gain. value."]
    P285DB = 23,
    #[doc = "22: 27.0 db gain. value."]
    P270DB = 22,
    #[doc = "21: 25.5 db gain. value."]
    P255DB = 21,
    #[doc = "20: 24.0 db gain. value."]
    P240DB = 20,
    #[doc = "19: 22.5 db gain. value."]
    P225DB = 19,
    #[doc = "18: 21.0 db gain. value."]
    P210DB = 18,
    #[doc = "17: 19.5 db gain. value."]
    P195DB = 17,
    #[doc = "16: 18.0 db gain. value."]
    P180DB = 16,
    #[doc = "15: 16.5 db gain. value."]
    P165DB = 15,
    #[doc = "14: 15.0 db gain. value."]
    P150DB = 14,
    #[doc = "13: 13.5 db gain. value."]
    P135DB = 13,
    #[doc = "12: 12.0 db gain. value."]
    P120DB = 12,
    #[doc = "11: 10.5 db gain. value."]
    P105DB = 11,
    #[doc = "10: 9.0 db gain. value."]
    P90DB = 10,
    #[doc = "9: 7.5 db gain. value."]
    P75DB = 9,
    #[doc = "8: 6.0 db gain. value."]
    P60DB = 8,
    #[doc = "7: 4.5 db gain. value."]
    P45DB = 7,
    #[doc = "6: 3.0 db gain. value."]
    P30DB = 6,
    #[doc = "5: 1.5 db gain. value."]
    P15DB = 5,
    #[doc = "4: 0.0 db gain. value."]
    _0DB = 4,
    #[doc = "3: -1.5 db gain. value."]
    M15DB = 3,
    #[doc = "2: -3.0 db gain. value."]
    M300DB = 2,
    #[doc = "1: -4.5 db gain. value."]
    M45DB = 1,
    #[doc = "0: -6.0 db gain. value."]
    M60DB = 0,
}
impl From<PGALEFT_A> for u8 {
    #[inline(always)]
    fn from(variant: PGALEFT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PGALEFT` reader - Left channel PGA gain."]
pub struct PGALEFT_R(crate::FieldReader<u8, PGALEFT_A>);
impl PGALEFT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PGALEFT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PGALEFT_A {
        match self.bits {
            31 => PGALEFT_A::P405DB,
            30 => PGALEFT_A::P390DB,
            29 => PGALEFT_A::P375DB,
            28 => PGALEFT_A::P360DB,
            27 => PGALEFT_A::P345DB,
            26 => PGALEFT_A::P330DB,
            25 => PGALEFT_A::P315DB,
            24 => PGALEFT_A::P300DB,
            23 => PGALEFT_A::P285DB,
            22 => PGALEFT_A::P270DB,
            21 => PGALEFT_A::P255DB,
            20 => PGALEFT_A::P240DB,
            19 => PGALEFT_A::P225DB,
            18 => PGALEFT_A::P210DB,
            17 => PGALEFT_A::P195DB,
            16 => PGALEFT_A::P180DB,
            15 => PGALEFT_A::P165DB,
            14 => PGALEFT_A::P150DB,
            13 => PGALEFT_A::P135DB,
            12 => PGALEFT_A::P120DB,
            11 => PGALEFT_A::P105DB,
            10 => PGALEFT_A::P90DB,
            9 => PGALEFT_A::P75DB,
            8 => PGALEFT_A::P60DB,
            7 => PGALEFT_A::P45DB,
            6 => PGALEFT_A::P30DB,
            5 => PGALEFT_A::P15DB,
            4 => PGALEFT_A::_0DB,
            3 => PGALEFT_A::M15DB,
            2 => PGALEFT_A::M300DB,
            1 => PGALEFT_A::M45DB,
            0 => PGALEFT_A::M60DB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P405DB`"]
    #[inline(always)]
    pub fn is_p405db(&self) -> bool {
        **self == PGALEFT_A::P405DB
    }
    #[doc = "Checks if the value of the field is `P390DB`"]
    #[inline(always)]
    pub fn is_p390db(&self) -> bool {
        **self == PGALEFT_A::P390DB
    }
    #[doc = "Checks if the value of the field is `P375DB`"]
    #[inline(always)]
    pub fn is_p375db(&self) -> bool {
        **self == PGALEFT_A::P375DB
    }
    #[doc = "Checks if the value of the field is `P360DB`"]
    #[inline(always)]
    pub fn is_p360db(&self) -> bool {
        **self == PGALEFT_A::P360DB
    }
    #[doc = "Checks if the value of the field is `P345DB`"]
    #[inline(always)]
    pub fn is_p345db(&self) -> bool {
        **self == PGALEFT_A::P345DB
    }
    #[doc = "Checks if the value of the field is `P330DB`"]
    #[inline(always)]
    pub fn is_p330db(&self) -> bool {
        **self == PGALEFT_A::P330DB
    }
    #[doc = "Checks if the value of the field is `P315DB`"]
    #[inline(always)]
    pub fn is_p315db(&self) -> bool {
        **self == PGALEFT_A::P315DB
    }
    #[doc = "Checks if the value of the field is `P300DB`"]
    #[inline(always)]
    pub fn is_p300db(&self) -> bool {
        **self == PGALEFT_A::P300DB
    }
    #[doc = "Checks if the value of the field is `P285DB`"]
    #[inline(always)]
    pub fn is_p285db(&self) -> bool {
        **self == PGALEFT_A::P285DB
    }
    #[doc = "Checks if the value of the field is `P270DB`"]
    #[inline(always)]
    pub fn is_p270db(&self) -> bool {
        **self == PGALEFT_A::P270DB
    }
    #[doc = "Checks if the value of the field is `P255DB`"]
    #[inline(always)]
    pub fn is_p255db(&self) -> bool {
        **self == PGALEFT_A::P255DB
    }
    #[doc = "Checks if the value of the field is `P240DB`"]
    #[inline(always)]
    pub fn is_p240db(&self) -> bool {
        **self == PGALEFT_A::P240DB
    }
    #[doc = "Checks if the value of the field is `P225DB`"]
    #[inline(always)]
    pub fn is_p225db(&self) -> bool {
        **self == PGALEFT_A::P225DB
    }
    #[doc = "Checks if the value of the field is `P210DB`"]
    #[inline(always)]
    pub fn is_p210db(&self) -> bool {
        **self == PGALEFT_A::P210DB
    }
    #[doc = "Checks if the value of the field is `P195DB`"]
    #[inline(always)]
    pub fn is_p195db(&self) -> bool {
        **self == PGALEFT_A::P195DB
    }
    #[doc = "Checks if the value of the field is `P180DB`"]
    #[inline(always)]
    pub fn is_p180db(&self) -> bool {
        **self == PGALEFT_A::P180DB
    }
    #[doc = "Checks if the value of the field is `P165DB`"]
    #[inline(always)]
    pub fn is_p165db(&self) -> bool {
        **self == PGALEFT_A::P165DB
    }
    #[doc = "Checks if the value of the field is `P150DB`"]
    #[inline(always)]
    pub fn is_p150db(&self) -> bool {
        **self == PGALEFT_A::P150DB
    }
    #[doc = "Checks if the value of the field is `P135DB`"]
    #[inline(always)]
    pub fn is_p135db(&self) -> bool {
        **self == PGALEFT_A::P135DB
    }
    #[doc = "Checks if the value of the field is `P120DB`"]
    #[inline(always)]
    pub fn is_p120db(&self) -> bool {
        **self == PGALEFT_A::P120DB
    }
    #[doc = "Checks if the value of the field is `P105DB`"]
    #[inline(always)]
    pub fn is_p105db(&self) -> bool {
        **self == PGALEFT_A::P105DB
    }
    #[doc = "Checks if the value of the field is `P90DB`"]
    #[inline(always)]
    pub fn is_p90db(&self) -> bool {
        **self == PGALEFT_A::P90DB
    }
    #[doc = "Checks if the value of the field is `P75DB`"]
    #[inline(always)]
    pub fn is_p75db(&self) -> bool {
        **self == PGALEFT_A::P75DB
    }
    #[doc = "Checks if the value of the field is `P60DB`"]
    #[inline(always)]
    pub fn is_p60db(&self) -> bool {
        **self == PGALEFT_A::P60DB
    }
    #[doc = "Checks if the value of the field is `P45DB`"]
    #[inline(always)]
    pub fn is_p45db(&self) -> bool {
        **self == PGALEFT_A::P45DB
    }
    #[doc = "Checks if the value of the field is `P30DB`"]
    #[inline(always)]
    pub fn is_p30db(&self) -> bool {
        **self == PGALEFT_A::P30DB
    }
    #[doc = "Checks if the value of the field is `P15DB`"]
    #[inline(always)]
    pub fn is_p15db(&self) -> bool {
        **self == PGALEFT_A::P15DB
    }
    #[doc = "Checks if the value of the field is `_0DB`"]
    #[inline(always)]
    pub fn is_0db(&self) -> bool {
        **self == PGALEFT_A::_0DB
    }
    #[doc = "Checks if the value of the field is `M15DB`"]
    #[inline(always)]
    pub fn is_m15db(&self) -> bool {
        **self == PGALEFT_A::M15DB
    }
    #[doc = "Checks if the value of the field is `M300DB`"]
    #[inline(always)]
    pub fn is_m300db(&self) -> bool {
        **self == PGALEFT_A::M300DB
    }
    #[doc = "Checks if the value of the field is `M45DB`"]
    #[inline(always)]
    pub fn is_m45db(&self) -> bool {
        **self == PGALEFT_A::M45DB
    }
    #[doc = "Checks if the value of the field is `M60DB`"]
    #[inline(always)]
    pub fn is_m60db(&self) -> bool {
        **self == PGALEFT_A::M60DB
    }
}
impl core::ops::Deref for PGALEFT_R {
    type Target = crate::FieldReader<u8, PGALEFT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PGALEFT` writer - Left channel PGA gain."]
pub struct PGALEFT_W<'a> {
    w: &'a mut W,
}
impl<'a> PGALEFT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PGALEFT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "40.5 db gain. value."]
    #[inline(always)]
    pub fn p405db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P405DB)
    }
    #[doc = "39.0 db gain. value."]
    #[inline(always)]
    pub fn p390db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P390DB)
    }
    #[doc = "37.5 db gain. value."]
    #[inline(always)]
    pub fn p375db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P375DB)
    }
    #[doc = "36.0 db gain. value."]
    #[inline(always)]
    pub fn p360db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P360DB)
    }
    #[doc = "34.5 db gain. value."]
    #[inline(always)]
    pub fn p345db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P345DB)
    }
    #[doc = "33.0 db gain. value."]
    #[inline(always)]
    pub fn p330db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P330DB)
    }
    #[doc = "31.5 db gain. value."]
    #[inline(always)]
    pub fn p315db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P315DB)
    }
    #[doc = "30.0 db gain. value."]
    #[inline(always)]
    pub fn p300db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P300DB)
    }
    #[doc = "28.5 db gain. value."]
    #[inline(always)]
    pub fn p285db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P285DB)
    }
    #[doc = "27.0 db gain. value."]
    #[inline(always)]
    pub fn p270db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P270DB)
    }
    #[doc = "25.5 db gain. value."]
    #[inline(always)]
    pub fn p255db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P255DB)
    }
    #[doc = "24.0 db gain. value."]
    #[inline(always)]
    pub fn p240db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P240DB)
    }
    #[doc = "22.5 db gain. value."]
    #[inline(always)]
    pub fn p225db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P225DB)
    }
    #[doc = "21.0 db gain. value."]
    #[inline(always)]
    pub fn p210db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P210DB)
    }
    #[doc = "19.5 db gain. value."]
    #[inline(always)]
    pub fn p195db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P195DB)
    }
    #[doc = "18.0 db gain. value."]
    #[inline(always)]
    pub fn p180db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P180DB)
    }
    #[doc = "16.5 db gain. value."]
    #[inline(always)]
    pub fn p165db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P165DB)
    }
    #[doc = "15.0 db gain. value."]
    #[inline(always)]
    pub fn p150db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P150DB)
    }
    #[doc = "13.5 db gain. value."]
    #[inline(always)]
    pub fn p135db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P135DB)
    }
    #[doc = "12.0 db gain. value."]
    #[inline(always)]
    pub fn p120db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P120DB)
    }
    #[doc = "10.5 db gain. value."]
    #[inline(always)]
    pub fn p105db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P105DB)
    }
    #[doc = "9.0 db gain. value."]
    #[inline(always)]
    pub fn p90db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P90DB)
    }
    #[doc = "7.5 db gain. value."]
    #[inline(always)]
    pub fn p75db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P75DB)
    }
    #[doc = "6.0 db gain. value."]
    #[inline(always)]
    pub fn p60db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P60DB)
    }
    #[doc = "4.5 db gain. value."]
    #[inline(always)]
    pub fn p45db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P45DB)
    }
    #[doc = "3.0 db gain. value."]
    #[inline(always)]
    pub fn p30db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P30DB)
    }
    #[doc = "1.5 db gain. value."]
    #[inline(always)]
    pub fn p15db(self) -> &'a mut W {
        self.variant(PGALEFT_A::P15DB)
    }
    #[doc = "0.0 db gain. value."]
    #[inline(always)]
    pub fn _0db(self) -> &'a mut W {
        self.variant(PGALEFT_A::_0DB)
    }
    #[doc = "-1.5 db gain. value."]
    #[inline(always)]
    pub fn m15db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M15DB)
    }
    #[doc = "-3.0 db gain. value."]
    #[inline(always)]
    pub fn m300db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M300DB)
    }
    #[doc = "-4.5 db gain. value."]
    #[inline(always)]
    pub fn m45db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M45DB)
    }
    #[doc = "-6.0 db gain. value."]
    #[inline(always)]
    pub fn m60db(self) -> &'a mut W {
        self.variant(PGALEFT_A::M60DB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
#[doc = "PDM_CLK frequency divisor.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCLKDIV_A {
    #[doc = "3: Divide input clock by 4 value."]
    MCKDIV4 = 3,
    #[doc = "2: Divide input clock by 3 value."]
    MCKDIV3 = 2,
    #[doc = "1: Divide input clock by 2 value."]
    MCKDIV2 = 1,
    #[doc = "0: Divide input clock by 1 value."]
    MCKDIV1 = 0,
}
impl From<MCLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MCLKDIV_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCLKDIV` reader - PDM_CLK frequency divisor."]
pub struct MCLKDIV_R(crate::FieldReader<u8, MCLKDIV_A>);
impl MCLKDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCLKDIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCLKDIV_A {
        match self.bits {
            3 => MCLKDIV_A::MCKDIV4,
            2 => MCLKDIV_A::MCKDIV3,
            1 => MCLKDIV_A::MCKDIV2,
            0 => MCLKDIV_A::MCKDIV1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCKDIV4`"]
    #[inline(always)]
    pub fn is_mckdiv4(&self) -> bool {
        **self == MCLKDIV_A::MCKDIV4
    }
    #[doc = "Checks if the value of the field is `MCKDIV3`"]
    #[inline(always)]
    pub fn is_mckdiv3(&self) -> bool {
        **self == MCLKDIV_A::MCKDIV3
    }
    #[doc = "Checks if the value of the field is `MCKDIV2`"]
    #[inline(always)]
    pub fn is_mckdiv2(&self) -> bool {
        **self == MCLKDIV_A::MCKDIV2
    }
    #[doc = "Checks if the value of the field is `MCKDIV1`"]
    #[inline(always)]
    pub fn is_mckdiv1(&self) -> bool {
        **self == MCLKDIV_A::MCKDIV1
    }
}
impl core::ops::Deref for MCLKDIV_R {
    type Target = crate::FieldReader<u8, MCLKDIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCLKDIV` writer - PDM_CLK frequency divisor."]
pub struct MCLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCLKDIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCLKDIV_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Divide input clock by 4 value."]
    #[inline(always)]
    pub fn mckdiv4(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV4)
    }
    #[doc = "Divide input clock by 3 value."]
    #[inline(always)]
    pub fn mckdiv3(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV3)
    }
    #[doc = "Divide input clock by 2 value."]
    #[inline(always)]
    pub fn mckdiv2(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV2)
    }
    #[doc = "Divide input clock by 1 value."]
    #[inline(always)]
    pub fn mckdiv1(self) -> &'a mut W {
        self.variant(MCLKDIV_A::MCKDIV1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `SINCRATE` reader - SINC decimation rate."]
pub struct SINCRATE_R(crate::FieldReader<u8, u8>);
impl SINCRATE_R {
    pub(crate) fn new(bits: u8) -> Self {
        SINCRATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINCRATE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINCRATE` writer - SINC decimation rate."]
pub struct SINCRATE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINCRATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 10)) | ((value as u32 & 0x7f) << 10);
        self.w
    }
}
#[doc = "High pass filter control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADCHPD_A {
    #[doc = "1: Enable high pass filter. value."]
    EN = 1,
    #[doc = "0: Disable high pass filter. value."]
    DIS = 0,
}
impl From<ADCHPD_A> for bool {
    #[inline(always)]
    fn from(variant: ADCHPD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCHPD` reader - High pass filter control."]
pub struct ADCHPD_R(crate::FieldReader<bool, ADCHPD_A>);
impl ADCHPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCHPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCHPD_A {
        match self.bits {
            true => ADCHPD_A::EN,
            false => ADCHPD_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == ADCHPD_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == ADCHPD_A::DIS
    }
}
impl core::ops::Deref for ADCHPD_R {
    type Target = crate::FieldReader<bool, ADCHPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCHPD` writer - High pass filter control."]
pub struct ADCHPD_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCHPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCHPD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable high pass filter. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ADCHPD_A::EN)
    }
    #[doc = "Disable high pass filter. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ADCHPD_A::DIS)
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
#[doc = "Field `HPCUTOFF` reader - High pass filter coefficients."]
pub struct HPCUTOFF_R(crate::FieldReader<u8, u8>);
impl HPCUTOFF_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPCUTOFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPCUTOFF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPCUTOFF` writer - High pass filter coefficients."]
pub struct HPCUTOFF_W<'a> {
    w: &'a mut W,
}
impl<'a> HPCUTOFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 5)) | ((value as u32 & 0x0f) << 5);
        self.w
    }
}
#[doc = "Field `CYCLES` reader - Number of clocks during gain-setting changes."]
pub struct CYCLES_R(crate::FieldReader<u8, u8>);
impl CYCLES_R {
    pub(crate) fn new(bits: u8) -> Self {
        CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCLES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCLES` writer - Number of clocks during gain-setting changes."]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 2)) | ((value as u32 & 0x07) << 2);
        self.w
    }
}
#[doc = "Soft mute control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SOFTMUTE_A {
    #[doc = "1: Enable Soft Mute. value."]
    EN = 1,
    #[doc = "0: Disable Soft Mute. value."]
    DIS = 0,
}
impl From<SOFTMUTE_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTMUTE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SOFTMUTE` reader - Soft mute control."]
pub struct SOFTMUTE_R(crate::FieldReader<bool, SOFTMUTE_A>);
impl SOFTMUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTMUTE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTMUTE_A {
        match self.bits {
            true => SOFTMUTE_A::EN,
            false => SOFTMUTE_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == SOFTMUTE_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == SOFTMUTE_A::DIS
    }
}
impl core::ops::Deref for SOFTMUTE_R {
    type Target = crate::FieldReader<bool, SOFTMUTE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTMUTE` writer - Soft mute control."]
pub struct SOFTMUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTMUTE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOFTMUTE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Soft Mute. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(SOFTMUTE_A::EN)
    }
    #[doc = "Disable Soft Mute. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(SOFTMUTE_A::DIS)
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
#[doc = "Data Streaming Control.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PDMCOREEN_A {
    #[doc = "1: Enable Data Streaming. value."]
    EN = 1,
    #[doc = "0: Disable Data Streaming. value."]
    DIS = 0,
}
impl From<PDMCOREEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDMCOREEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDMCOREEN` reader - Data Streaming Control."]
pub struct PDMCOREEN_R(crate::FieldReader<bool, PDMCOREEN_A>);
impl PDMCOREEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDMCOREEN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDMCOREEN_A {
        match self.bits {
            true => PDMCOREEN_A::EN,
            false => PDMCOREEN_A::DIS,
        }
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        **self == PDMCOREEN_A::EN
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        **self == PDMCOREEN_A::DIS
    }
}
impl core::ops::Deref for PDMCOREEN_R {
    type Target = crate::FieldReader<bool, PDMCOREEN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDMCOREEN` writer - Data Streaming Control."]
pub struct PDMCOREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDMCOREEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PDMCOREEN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable Data Streaming. value."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(PDMCOREEN_A::EN)
    }
    #[doc = "Disable Data Streaming. value."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(PDMCOREEN_A::DIS)
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
    #[doc = "Bit 31 - Left/right channel swap."]
    #[inline(always)]
    pub fn lrswap(&self) -> LRSWAP_R {
        LRSWAP_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 26:30 - Right channel PGA gain."]
    #[inline(always)]
    pub fn pgaright(&self) -> PGARIGHT_R {
        PGARIGHT_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Left channel PGA gain."]
    #[inline(always)]
    pub fn pgaleft(&self) -> PGALEFT_R {
        PGALEFT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 17:18 - PDM_CLK frequency divisor."]
    #[inline(always)]
    pub fn mclkdiv(&self) -> MCLKDIV_R {
        MCLKDIV_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 10:16 - SINC decimation rate."]
    #[inline(always)]
    pub fn sincrate(&self) -> SINCRATE_R {
        SINCRATE_R::new(((self.bits >> 10) & 0x7f) as u8)
    }
    #[doc = "Bit 9 - High pass filter control."]
    #[inline(always)]
    pub fn adchpd(&self) -> ADCHPD_R {
        ADCHPD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 5:8 - High pass filter coefficients."]
    #[inline(always)]
    pub fn hpcutoff(&self) -> HPCUTOFF_R {
        HPCUTOFF_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 2:4 - Number of clocks during gain-setting changes."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new(((self.bits >> 2) & 0x07) as u8)
    }
    #[doc = "Bit 1 - Soft mute control."]
    #[inline(always)]
    pub fn softmute(&self) -> SOFTMUTE_R {
        SOFTMUTE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Data Streaming Control."]
    #[inline(always)]
    pub fn pdmcoreen(&self) -> PDMCOREEN_R {
        PDMCOREEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Left/right channel swap."]
    #[inline(always)]
    pub fn lrswap(&mut self) -> LRSWAP_W {
        LRSWAP_W { w: self }
    }
    #[doc = "Bits 26:30 - Right channel PGA gain."]
    #[inline(always)]
    pub fn pgaright(&mut self) -> PGARIGHT_W {
        PGARIGHT_W { w: self }
    }
    #[doc = "Bits 21:25 - Left channel PGA gain."]
    #[inline(always)]
    pub fn pgaleft(&mut self) -> PGALEFT_W {
        PGALEFT_W { w: self }
    }
    #[doc = "Bits 17:18 - PDM_CLK frequency divisor."]
    #[inline(always)]
    pub fn mclkdiv(&mut self) -> MCLKDIV_W {
        MCLKDIV_W { w: self }
    }
    #[doc = "Bits 10:16 - SINC decimation rate."]
    #[inline(always)]
    pub fn sincrate(&mut self) -> SINCRATE_W {
        SINCRATE_W { w: self }
    }
    #[doc = "Bit 9 - High pass filter control."]
    #[inline(always)]
    pub fn adchpd(&mut self) -> ADCHPD_W {
        ADCHPD_W { w: self }
    }
    #[doc = "Bits 5:8 - High pass filter coefficients."]
    #[inline(always)]
    pub fn hpcutoff(&mut self) -> HPCUTOFF_W {
        HPCUTOFF_W { w: self }
    }
    #[doc = "Bits 2:4 - Number of clocks during gain-setting changes."]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
    #[doc = "Bit 1 - Soft mute control."]
    #[inline(always)]
    pub fn softmute(&mut self) -> SOFTMUTE_W {
        SOFTMUTE_W { w: self }
    }
    #[doc = "Bit 0 - Data Streaming Control."]
    #[inline(always)]
    pub fn pdmcoreen(&mut self) -> PDMCOREEN_W {
        PDMCOREEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcfg](index.html) module"]
pub struct PCFG_SPEC;
impl crate::RegisterSpec for PCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcfg::R](R) reader structure"]
impl crate::Readable for PCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcfg::W](W) writer structure"]
impl crate::Writable for PCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCFG to value 0xc365"]
impl crate::Resettable for PCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xc365
    }
}

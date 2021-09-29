#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LVLSEL_A {
    #[doc = "0: Set Reference input to 0.58 Volts. value."]
    _0P58V = 0,
    #[doc = "1: Set Reference input to 0.77 Volts. value."]
    _0P77V = 1,
    #[doc = "2: Set Reference input to 0.97 Volts. value."]
    _0P97V = 2,
    #[doc = "3: Set Reference input to 1.16 Volts. value."]
    _1P16V = 3,
    #[doc = "4: Set Reference input to 1.35 Volts. value."]
    _1P35V = 4,
    #[doc = "5: Set Reference input to 1.55 Volts. value."]
    _1P55V = 5,
    #[doc = "6: Set Reference input to 1.74 Volts. value."]
    _1P74V = 6,
    #[doc = "7: Set Reference input to 1.93 Volts. value."]
    _1P93V = 7,
    #[doc = "8: Set Reference input to 2.13 Volts. value."]
    _2P13V = 8,
    #[doc = "9: Set Reference input to 2.32 Volts. value."]
    _2P32V = 9,
    #[doc = "10: Set Reference input to 2.51 Volts. value."]
    _2P51V = 10,
    #[doc = "11: Set Reference input to 2.71 Volts. value."]
    _2P71V = 11,
    #[doc = "12: Set Reference input to 2.90 Volts. value."]
    _2P90V = 12,
    #[doc = "13: Set Reference input to 3.09 Volts. value."]
    _3P09V = 13,
    #[doc = "14: Set Reference input to 3.29 Volts. value."]
    _3P29V = 14,
    #[doc = "15: Set Reference input to 3.48 Volts. value."]
    _3P48V = 15,
}
impl From<LVLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LVLSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LVLSEL` reader - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
pub struct LVLSEL_R(crate::FieldReader<u8, LVLSEL_A>);
impl LVLSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LVLSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVLSEL_A {
        match self.bits {
            0 => LVLSEL_A::_0P58V,
            1 => LVLSEL_A::_0P77V,
            2 => LVLSEL_A::_0P97V,
            3 => LVLSEL_A::_1P16V,
            4 => LVLSEL_A::_1P35V,
            5 => LVLSEL_A::_1P55V,
            6 => LVLSEL_A::_1P74V,
            7 => LVLSEL_A::_1P93V,
            8 => LVLSEL_A::_2P13V,
            9 => LVLSEL_A::_2P32V,
            10 => LVLSEL_A::_2P51V,
            11 => LVLSEL_A::_2P71V,
            12 => LVLSEL_A::_2P90V,
            13 => LVLSEL_A::_3P09V,
            14 => LVLSEL_A::_3P29V,
            15 => LVLSEL_A::_3P48V,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0P58V`"]
    #[inline(always)]
    pub fn is_0p58v(&self) -> bool {
        **self == LVLSEL_A::_0P58V
    }
    #[doc = "Checks if the value of the field is `_0P77V`"]
    #[inline(always)]
    pub fn is_0p77v(&self) -> bool {
        **self == LVLSEL_A::_0P77V
    }
    #[doc = "Checks if the value of the field is `_0P97V`"]
    #[inline(always)]
    pub fn is_0p97v(&self) -> bool {
        **self == LVLSEL_A::_0P97V
    }
    #[doc = "Checks if the value of the field is `_1P16V`"]
    #[inline(always)]
    pub fn is_1p16v(&self) -> bool {
        **self == LVLSEL_A::_1P16V
    }
    #[doc = "Checks if the value of the field is `_1P35V`"]
    #[inline(always)]
    pub fn is_1p35v(&self) -> bool {
        **self == LVLSEL_A::_1P35V
    }
    #[doc = "Checks if the value of the field is `_1P55V`"]
    #[inline(always)]
    pub fn is_1p55v(&self) -> bool {
        **self == LVLSEL_A::_1P55V
    }
    #[doc = "Checks if the value of the field is `_1P74V`"]
    #[inline(always)]
    pub fn is_1p74v(&self) -> bool {
        **self == LVLSEL_A::_1P74V
    }
    #[doc = "Checks if the value of the field is `_1P93V`"]
    #[inline(always)]
    pub fn is_1p93v(&self) -> bool {
        **self == LVLSEL_A::_1P93V
    }
    #[doc = "Checks if the value of the field is `_2P13V`"]
    #[inline(always)]
    pub fn is_2p13v(&self) -> bool {
        **self == LVLSEL_A::_2P13V
    }
    #[doc = "Checks if the value of the field is `_2P32V`"]
    #[inline(always)]
    pub fn is_2p32v(&self) -> bool {
        **self == LVLSEL_A::_2P32V
    }
    #[doc = "Checks if the value of the field is `_2P51V`"]
    #[inline(always)]
    pub fn is_2p51v(&self) -> bool {
        **self == LVLSEL_A::_2P51V
    }
    #[doc = "Checks if the value of the field is `_2P71V`"]
    #[inline(always)]
    pub fn is_2p71v(&self) -> bool {
        **self == LVLSEL_A::_2P71V
    }
    #[doc = "Checks if the value of the field is `_2P90V`"]
    #[inline(always)]
    pub fn is_2p90v(&self) -> bool {
        **self == LVLSEL_A::_2P90V
    }
    #[doc = "Checks if the value of the field is `_3P09V`"]
    #[inline(always)]
    pub fn is_3p09v(&self) -> bool {
        **self == LVLSEL_A::_3P09V
    }
    #[doc = "Checks if the value of the field is `_3P29V`"]
    #[inline(always)]
    pub fn is_3p29v(&self) -> bool {
        **self == LVLSEL_A::_3P29V
    }
    #[doc = "Checks if the value of the field is `_3P48V`"]
    #[inline(always)]
    pub fn is_3p48v(&self) -> bool {
        **self == LVLSEL_A::_3P48V
    }
}
impl core::ops::Deref for LVLSEL_R {
    type Target = crate::FieldReader<u8, LVLSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LVLSEL` writer - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
pub struct LVLSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LVLSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LVLSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Set Reference input to 0.58 Volts. value."]
    #[inline(always)]
    pub fn _0p58v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_0P58V)
    }
    #[doc = "Set Reference input to 0.77 Volts. value."]
    #[inline(always)]
    pub fn _0p77v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_0P77V)
    }
    #[doc = "Set Reference input to 0.97 Volts. value."]
    #[inline(always)]
    pub fn _0p97v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_0P97V)
    }
    #[doc = "Set Reference input to 1.16 Volts. value."]
    #[inline(always)]
    pub fn _1p16v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_1P16V)
    }
    #[doc = "Set Reference input to 1.35 Volts. value."]
    #[inline(always)]
    pub fn _1p35v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_1P35V)
    }
    #[doc = "Set Reference input to 1.55 Volts. value."]
    #[inline(always)]
    pub fn _1p55v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_1P55V)
    }
    #[doc = "Set Reference input to 1.74 Volts. value."]
    #[inline(always)]
    pub fn _1p74v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_1P74V)
    }
    #[doc = "Set Reference input to 1.93 Volts. value."]
    #[inline(always)]
    pub fn _1p93v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_1P93V)
    }
    #[doc = "Set Reference input to 2.13 Volts. value."]
    #[inline(always)]
    pub fn _2p13v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_2P13V)
    }
    #[doc = "Set Reference input to 2.32 Volts. value."]
    #[inline(always)]
    pub fn _2p32v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_2P32V)
    }
    #[doc = "Set Reference input to 2.51 Volts. value."]
    #[inline(always)]
    pub fn _2p51v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_2P51V)
    }
    #[doc = "Set Reference input to 2.71 Volts. value."]
    #[inline(always)]
    pub fn _2p71v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_2P71V)
    }
    #[doc = "Set Reference input to 2.90 Volts. value."]
    #[inline(always)]
    pub fn _2p90v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_2P90V)
    }
    #[doc = "Set Reference input to 3.09 Volts. value."]
    #[inline(always)]
    pub fn _3p09v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_3P09V)
    }
    #[doc = "Set Reference input to 3.29 Volts. value."]
    #[inline(always)]
    pub fn _3p29v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_3P29V)
    }
    #[doc = "Set Reference input to 3.48 Volts. value."]
    #[inline(always)]
    pub fn _3p48v(self) -> &'a mut W {
        self.variant(LVLSEL_A::_3P48V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "This bitfield selects the negative input to the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NSEL_A {
    #[doc = "0: Use external reference 1 for reference input. value."]
    VREFEXT1 = 0,
    #[doc = "1: Use external reference 2 for reference input. value."]
    VREFEXT2 = 1,
    #[doc = "2: Use external reference 3 for reference input. value."]
    VREFEXT3 = 2,
    #[doc = "3: Use DAC output selected by LVLSEL for reference input. value."]
    DAC = 3,
}
impl From<NSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NSEL` reader - This bitfield selects the negative input to the comparator."]
pub struct NSEL_R(crate::FieldReader<u8, NSEL_A>);
impl NSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSEL_A {
        match self.bits {
            0 => NSEL_A::VREFEXT1,
            1 => NSEL_A::VREFEXT2,
            2 => NSEL_A::VREFEXT3,
            3 => NSEL_A::DAC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VREFEXT1`"]
    #[inline(always)]
    pub fn is_vrefext1(&self) -> bool {
        **self == NSEL_A::VREFEXT1
    }
    #[doc = "Checks if the value of the field is `VREFEXT2`"]
    #[inline(always)]
    pub fn is_vrefext2(&self) -> bool {
        **self == NSEL_A::VREFEXT2
    }
    #[doc = "Checks if the value of the field is `VREFEXT3`"]
    #[inline(always)]
    pub fn is_vrefext3(&self) -> bool {
        **self == NSEL_A::VREFEXT3
    }
    #[doc = "Checks if the value of the field is `DAC`"]
    #[inline(always)]
    pub fn is_dac(&self) -> bool {
        **self == NSEL_A::DAC
    }
}
impl core::ops::Deref for NSEL_R {
    type Target = crate::FieldReader<u8, NSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSEL` writer - This bitfield selects the negative input to the comparator."]
pub struct NSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Use external reference 1 for reference input. value."]
    #[inline(always)]
    pub fn vrefext1(self) -> &'a mut W {
        self.variant(NSEL_A::VREFEXT1)
    }
    #[doc = "Use external reference 2 for reference input. value."]
    #[inline(always)]
    pub fn vrefext2(self) -> &'a mut W {
        self.variant(NSEL_A::VREFEXT2)
    }
    #[doc = "Use external reference 3 for reference input. value."]
    #[inline(always)]
    pub fn vrefext3(self) -> &'a mut W {
        self.variant(NSEL_A::VREFEXT3)
    }
    #[doc = "Use DAC output selected by LVLSEL for reference input. value."]
    #[inline(always)]
    pub fn dac(self) -> &'a mut W {
        self.variant(NSEL_A::DAC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "This bitfield selects the positive input to the comparator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: Use VDDADJ for the positive input. value."]
    VDDADJ = 0,
    #[doc = "1: Use the temperature sensor output for the positive input.  Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on.  The bandgap circuit requires 11us to stabalize. value."]
    VTEMP = 1,
    #[doc = "2: Use external voltage 0 for positive input. value."]
    VEXT1 = 2,
    #[doc = "3: Use external voltage 1 for positive input. value."]
    VEXT2 = 3,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PSEL` reader - This bitfield selects the positive input to the comparator."]
pub struct PSEL_R(crate::FieldReader<u8, PSEL_A>);
impl PSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::VDDADJ,
            1 => PSEL_A::VTEMP,
            2 => PSEL_A::VEXT1,
            3 => PSEL_A::VEXT2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VDDADJ`"]
    #[inline(always)]
    pub fn is_vddadj(&self) -> bool {
        **self == PSEL_A::VDDADJ
    }
    #[doc = "Checks if the value of the field is `VTEMP`"]
    #[inline(always)]
    pub fn is_vtemp(&self) -> bool {
        **self == PSEL_A::VTEMP
    }
    #[doc = "Checks if the value of the field is `VEXT1`"]
    #[inline(always)]
    pub fn is_vext1(&self) -> bool {
        **self == PSEL_A::VEXT1
    }
    #[doc = "Checks if the value of the field is `VEXT2`"]
    #[inline(always)]
    pub fn is_vext2(&self) -> bool {
        **self == PSEL_A::VEXT2
    }
}
impl core::ops::Deref for PSEL_R {
    type Target = crate::FieldReader<u8, PSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSEL` writer - This bitfield selects the positive input to the comparator."]
pub struct PSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Use VDDADJ for the positive input. value."]
    #[inline(always)]
    pub fn vddadj(self) -> &'a mut W {
        self.variant(PSEL_A::VDDADJ)
    }
    #[doc = "Use the temperature sensor output for the positive input. Note: If this channel is selected for PSEL, the bandap circuit required for temperature comparisons will automatically turn on. The bandgap circuit requires 11us to stabalize. value."]
    #[inline(always)]
    pub fn vtemp(self) -> &'a mut W {
        self.variant(PSEL_A::VTEMP)
    }
    #[doc = "Use external voltage 0 for positive input. value."]
    #[inline(always)]
    pub fn vext1(self) -> &'a mut W {
        self.variant(PSEL_A::VEXT1)
    }
    #[doc = "Use external voltage 1 for positive input. value."]
    #[inline(always)]
    pub fn vext2(self) -> &'a mut W {
        self.variant(PSEL_A::VEXT2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:19 - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline(always)]
    pub fn lvlsel(&self) -> LVLSEL_R {
        LVLSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - This bitfield selects the negative input to the comparator."]
    #[inline(always)]
    pub fn nsel(&self) -> NSEL_R {
        NSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - This bitfield selects the positive input to the comparator."]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 16:19 - When the reference input NSEL is set to NSEL_DAC, this bitfield selects the voltage level for the negative input to the comparator."]
    #[inline(always)]
    pub fn lvlsel(&mut self) -> LVLSEL_W {
        LVLSEL_W { w: self }
    }
    #[doc = "Bits 8:9 - This bitfield selects the negative input to the comparator."]
    #[inline(always)]
    pub fn nsel(&mut self) -> NSEL_W {
        NSEL_W { w: self }
    }
    #[doc = "Bits 0:1 - This bitfield selects the positive input to the comparator."]
    #[inline(always)]
    pub fn psel(&mut self) -> PSEL_W {
        PSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

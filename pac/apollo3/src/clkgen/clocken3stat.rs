#[doc = "Register `CLOCKEN3STAT` reader"]
pub struct R(crate::R<CLOCKEN3STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLOCKEN3STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLOCKEN3STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLOCKEN3STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLOCKEN3STAT` writer"]
pub struct W(crate::W<CLOCKEN3STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLOCKEN3STAT_SPEC>;
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
impl From<crate::W<CLOCKEN3STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLOCKEN3STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Clock enable status 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum CLOCKEN3STAT_A {
    #[doc = "131072: DAP clock is enabled \\[17\\]
value."]
    DAP_ENABLED = 131072,
    #[doc = "262144: VCOMP powerdown indicator \\[18\\]
value."]
    VCOMP_ENABLED = 262144,
    #[doc = "16777216: XTAL is enabled \\[24\\]
value."]
    XTAL_ENABLED = 16777216,
    #[doc = "33554432: HFRC is enabled \\[25\\]
value."]
    HFRC_ENABLED = 33554432,
    #[doc = "67108864: HFRC Adjust enabled \\[26\\]
value."]
    HFADJEN = 67108864,
    #[doc = "134217728: HFRC Enabled out \\[27\\]
value."]
    HFRC_EN_OUT = 134217728,
    #[doc = "268435456: RTC use XT \\[28\\]
value."]
    RTC_XT = 268435456,
    #[doc = "536870912: XTAL clkout enabled \\[29\\]
value."]
    CLKOUT_XTAL_EN = 536870912,
    #[doc = "1073741824: HFRC clkout enabled \\[30\\]
value."]
    CLKOUT_HFRC_EN = 1073741824,
    #[doc = "2147483648: Flash clk is enabled \\[31\\]
value."]
    FLASHCLK_EN = 2147483648,
}
impl From<CLOCKEN3STAT_A> for u32 {
    #[inline(always)]
    fn from(variant: CLOCKEN3STAT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLOCKEN3STAT` reader - Clock enable status 3"]
pub struct CLOCKEN3STAT_R(crate::FieldReader<u32, CLOCKEN3STAT_A>);
impl CLOCKEN3STAT_R {
    pub(crate) fn new(bits: u32) -> Self {
        CLOCKEN3STAT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLOCKEN3STAT_A> {
        match self.bits {
            131072 => Some(CLOCKEN3STAT_A::DAP_ENABLED),
            262144 => Some(CLOCKEN3STAT_A::VCOMP_ENABLED),
            16777216 => Some(CLOCKEN3STAT_A::XTAL_ENABLED),
            33554432 => Some(CLOCKEN3STAT_A::HFRC_ENABLED),
            67108864 => Some(CLOCKEN3STAT_A::HFADJEN),
            134217728 => Some(CLOCKEN3STAT_A::HFRC_EN_OUT),
            268435456 => Some(CLOCKEN3STAT_A::RTC_XT),
            536870912 => Some(CLOCKEN3STAT_A::CLKOUT_XTAL_EN),
            1073741824 => Some(CLOCKEN3STAT_A::CLKOUT_HFRC_EN),
            2147483648 => Some(CLOCKEN3STAT_A::FLASHCLK_EN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DAP_ENABLED`"]
    #[inline(always)]
    pub fn is_dap_enabled(&self) -> bool {
        **self == CLOCKEN3STAT_A::DAP_ENABLED
    }
    #[doc = "Checks if the value of the field is `VCOMP_ENABLED`"]
    #[inline(always)]
    pub fn is_vcomp_enabled(&self) -> bool {
        **self == CLOCKEN3STAT_A::VCOMP_ENABLED
    }
    #[doc = "Checks if the value of the field is `XTAL_ENABLED`"]
    #[inline(always)]
    pub fn is_xtal_enabled(&self) -> bool {
        **self == CLOCKEN3STAT_A::XTAL_ENABLED
    }
    #[doc = "Checks if the value of the field is `HFRC_ENABLED`"]
    #[inline(always)]
    pub fn is_hfrc_enabled(&self) -> bool {
        **self == CLOCKEN3STAT_A::HFRC_ENABLED
    }
    #[doc = "Checks if the value of the field is `HFADJEN`"]
    #[inline(always)]
    pub fn is_hfadjen(&self) -> bool {
        **self == CLOCKEN3STAT_A::HFADJEN
    }
    #[doc = "Checks if the value of the field is `HFRC_EN_OUT`"]
    #[inline(always)]
    pub fn is_hfrc_en_out(&self) -> bool {
        **self == CLOCKEN3STAT_A::HFRC_EN_OUT
    }
    #[doc = "Checks if the value of the field is `RTC_XT`"]
    #[inline(always)]
    pub fn is_rtc_xt(&self) -> bool {
        **self == CLOCKEN3STAT_A::RTC_XT
    }
    #[doc = "Checks if the value of the field is `CLKOUT_XTAL_EN`"]
    #[inline(always)]
    pub fn is_clkout_xtal_en(&self) -> bool {
        **self == CLOCKEN3STAT_A::CLKOUT_XTAL_EN
    }
    #[doc = "Checks if the value of the field is `CLKOUT_HFRC_EN`"]
    #[inline(always)]
    pub fn is_clkout_hfrc_en(&self) -> bool {
        **self == CLOCKEN3STAT_A::CLKOUT_HFRC_EN
    }
    #[doc = "Checks if the value of the field is `FLASHCLK_EN`"]
    #[inline(always)]
    pub fn is_flashclk_en(&self) -> bool {
        **self == CLOCKEN3STAT_A::FLASHCLK_EN
    }
}
impl core::ops::Deref for CLOCKEN3STAT_R {
    type Target = crate::FieldReader<u32, CLOCKEN3STAT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLOCKEN3STAT` writer - Clock enable status 3"]
pub struct CLOCKEN3STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> CLOCKEN3STAT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLOCKEN3STAT_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "DAP clock is enabled \\[17\\]
value."]
    #[inline(always)]
    pub fn dap_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::DAP_ENABLED)
    }
    #[doc = "VCOMP powerdown indicator \\[18\\]
value."]
    #[inline(always)]
    pub fn vcomp_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::VCOMP_ENABLED)
    }
    #[doc = "XTAL is enabled \\[24\\]
value."]
    #[inline(always)]
    pub fn xtal_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::XTAL_ENABLED)
    }
    #[doc = "HFRC is enabled \\[25\\]
value."]
    #[inline(always)]
    pub fn hfrc_enabled(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::HFRC_ENABLED)
    }
    #[doc = "HFRC Adjust enabled \\[26\\]
value."]
    #[inline(always)]
    pub fn hfadjen(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::HFADJEN)
    }
    #[doc = "HFRC Enabled out \\[27\\]
value."]
    #[inline(always)]
    pub fn hfrc_en_out(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::HFRC_EN_OUT)
    }
    #[doc = "RTC use XT \\[28\\]
value."]
    #[inline(always)]
    pub fn rtc_xt(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::RTC_XT)
    }
    #[doc = "XTAL clkout enabled \\[29\\]
value."]
    #[inline(always)]
    pub fn clkout_xtal_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::CLKOUT_XTAL_EN)
    }
    #[doc = "HFRC clkout enabled \\[30\\]
value."]
    #[inline(always)]
    pub fn clkout_hfrc_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::CLKOUT_HFRC_EN)
    }
    #[doc = "Flash clk is enabled \\[31\\]
value."]
    #[inline(always)]
    pub fn flashclk_en(self) -> &'a mut W {
        self.variant(CLOCKEN3STAT_A::FLASHCLK_EN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    pub fn clocken3stat(&self) -> CLOCKEN3STAT_R {
        CLOCKEN3STAT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clock enable status 3"]
    #[inline(always)]
    pub fn clocken3stat(&mut self) -> CLOCKEN3STAT_W {
        CLOCKEN3STAT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock Enable Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clocken3stat](index.html) module"]
pub struct CLOCKEN3STAT_SPEC;
impl crate::RegisterSpec for CLOCKEN3STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clocken3stat::R](R) reader structure"]
impl crate::Readable for CLOCKEN3STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clocken3stat::W](W) writer structure"]
impl crate::Writable for CLOCKEN3STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLOCKEN3STAT to value 0"]
impl crate::Resettable for CLOCKEN3STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
